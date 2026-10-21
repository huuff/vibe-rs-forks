//! Supertrait support.
//!
//! A proc macro only sees the tokens of the item it is attached to, so when
//! expanding `trait Foo: Bar` it cannot know `Bar`'s methods, yet it must
//! emit `impl Bar for DynFoo`. To bridge this, every `#[dynosaur]` trait also
//! emits a `macro_rules! __dynosaur_<Trait>` carrying its own definition and
//! those of its (transitively) resolved supertraits. When `Foo` has a
//! non-auto supertrait, its expansion becomes a call to `__dynosaur_Bar!`,
//! which appends `Bar`'s tokens and calls back into `__resolve_supertraits!`.
//! This repeats for every direct supertrait, then the real expansion runs.
//!
//! Paths: a supertrait is referred to by the path written in the bounds that
//! mention it. For a transitive one (`Qux` in `Foo: Bar`, `Bar: Qux`) that is
//! the path written in `Bar`, which must also resolve where `Foo` is defined;
//! if it does not, rustc reports "cannot find trait `Qux`" with the `use` to
//! add. (A generated re-export cannot help: re-exporting a privately imported
//! trait is an error.)
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_quote, Error, Ident, ItemTrait, Path, PathArguments, PathSegment, Result,
    TraitBoundModifier, TypeParamBound, Visibility,
};

/// A supertrait whose definition has been resolved, together with a path
/// that names it from the module of the trait being expanded.
pub(crate) struct Supertrait {
    pub(crate) path: Path,
    pub(crate) item: ItemTrait,
}

/// Input of `__resolve_supertraits!`:
///
/// ```text
/// attr { DynFoo = dyn(box) Foo }
/// item { trait Foo: Bar { ... } }
/// supertrait { trait Bar: Qux { ... } }   // appended by __dynosaur_Bar!, along
/// supertrait { trait Qux { ... } }        // with Bar's own resolved supertraits
/// ```
pub(crate) struct ResolveInput {
    pub(crate) attr: TokenStream,
    pub(crate) item: ItemTrait,
    pub(crate) supertraits: Vec<ItemTrait>,
}

impl Parse for ResolveInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attr = None;
        let mut item = None;
        let mut supertraits = Vec::new();
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let content;
            braced!(content in input);
            match key.to_string().as_str() {
                "attr" => attr = Some(content.parse()?),
                "item" => item = Some(content.parse()?),
                "supertrait" => supertraits.push(content.parse()?),
                _ => return Err(Error::new(key.span(), "unexpected key")),
            }
        }
        Ok(ResolveInput {
            attr: attr.ok_or_else(|| input.error("missing `attr`"))?,
            item: item.ok_or_else(|| input.error("missing `item`"))?,
            supertraits,
        })
    }
}

fn is_auto_trait(path: &Path) -> bool {
    matches!(
        last_ident(path).to_string().as_str(),
        "Send" | "Sync" | "Unpin" | "UnwindSafe" | "RefUnwindSafe" | "Sized"
    )
}

/// Trait bounds that must be resolved: everything except lifetimes, `?Sized`
/// and auto/marker traits.
pub(crate) fn needs_resolving(bound: &TypeParamBound) -> Option<&Path> {
    match bound {
        TypeParamBound::Trait(t) if matches!(t.modifier, TraitBoundModifier::None) => {
            if is_auto_trait(&t.path) {
                None
            } else {
                Some(&t.path)
            }
        }
        _ => None,
    }
}

fn last_ident(path: &Path) -> &Ident {
    &path
        .segments
        .last()
        .expect("path has at least one segment")
        .ident
}

/// `a::b::Bar` + `name` -> `a::b::name`; `Bar` + `name` -> `name`.
fn sibling_path(path: &Path, name: Ident) -> Path {
    let mut path = path.clone();
    path.segments.pop();
    path.segments.push(PathSegment::from(name));
    path
}

/// Resolve the trait's supertraits (transitively) against the definitions
/// received so far. Returns the resolved ones and the paths still pending.
/// Resolved traits are keyed by their ident; the path is the one written in
/// the bounds that first mention them.
fn classify(item: &ItemTrait, defs: Vec<ItemTrait>) -> (Vec<Supertrait>, Vec<Path>) {
    let mut resolved: Vec<Supertrait> = Vec::new();
    let mut pending: Vec<Path> = Vec::new();

    let mut queue: Vec<ItemTrait> = vec![item.clone()];
    while let Some(trait_item) = queue.pop() {
        for path in trait_item.supertraits.iter().filter_map(needs_resolving) {
            let name = last_ident(path);
            let known = resolved.iter().any(|s| s.item.ident == *name)
                || pending.iter().any(|p| last_ident(p) == name);
            if known {
                continue;
            }
            match defs.iter().find(|t| t.ident == *name) {
                Some(def) => {
                    queue.push(def.clone());
                    resolved.push(Supertrait {
                        path: path.clone(),
                        item: def.clone(),
                    });
                }
                None => pending.push(path.clone()),
            }
        }
    }
    (resolved, pending)
}

/// Either hand off to the next supertrait's callback macro, or run `expand`
/// once every supertrait is known.
pub(crate) fn resolve_or_expand(
    attr: TokenStream,
    item: ItemTrait,
    defs: Vec<ItemTrait>,
    expand: impl FnOnce(&ItemTrait, &[Supertrait]) -> TokenStream,
) -> TokenStream {
    let (resolved, pending) = classify(&item, defs);
    let Some(next) = pending.first() else {
        return expand(&item, &resolved);
    };

    if next
        .segments
        .iter()
        .any(|s| !matches!(s.arguments, PathArguments::None))
    {
        return Error::new_spanned(next, "generic supertraits are not supported by dynosaur")
            .to_compile_error();
    }

    // `a::b::Bar` -> `a::b::__dynosaur_Bar`, spanned at the bound so an
    // unresolved-macro error points there.
    let name = last_ident(next);
    let macro_path = sibling_path(next, callback_macro_name(name, name.span()));
    let defs = resolved.iter().map(|s| strip_for_carry(&s.item));
    quote! {
        #macro_path! {
            attr { #attr }
            item { #item }
            #(supertrait { #defs })*
        }
    }
}

pub(crate) fn callback_macro_name(trait_ident: &Ident, span: Span) -> Ident {
    Ident::new(&format!("__dynosaur_{trait_ident}"), span)
}

/// A `macro_rules!` can be re-exported by path at most crate-wide (anything
/// wider needs `#[macro_export]`, which macro-generated macros cannot use by
/// path), so `pub` is capped to `pub(crate)`.
fn crate_capped(vis: &Visibility) -> Visibility {
    match vis {
        Visibility::Public(_) => parse_quote!(pub(crate)),
        other => other.clone(),
    }
}

/// The definition as carried through the callback macro: no attributes (they
/// were already consumed or belong to the original item), no default bodies.
fn strip_for_carry(item: &ItemTrait) -> ItemTrait {
    let mut item = item.clone();
    item.attrs.clear();
    for trait_item in &mut item.items {
        if let syn::TraitItem::Fn(f) = trait_item {
            f.default = None;
        }
    }
    item
}

/// The callback `macro_rules!` emitted next to every `#[dynosaur]` trait so
/// it can be a supertrait of another one. It carries the trait's definition
/// and those of its resolved supertraits.
pub(crate) fn mk_callback_macro(item: &ItemTrait, supertraits: &[Supertrait]) -> TokenStream {
    let vis = crate_capped(&item.vis);
    let name = callback_macro_name(&item.ident, Span::call_site());
    let own = strip_for_carry(item);
    let carried = supertraits.iter().map(|s| strip_for_carry(&s.item));
    let doc = format!(
        "Internal: carries the definition of [`{}`] so it can be a supertrait of another `#[dynosaur]` trait.",
        item.ident
    );
    quote! {
        #[doc = #doc]
        #[doc(hidden)]
        macro_rules! #name {
            ($($rest:tt)*) => {
                ::dynosaur::__resolve_supertraits! {
                    $($rest)*
                    supertrait { #own }
                    #(supertrait { #carried })*
                }
            };
        }
        #[doc(hidden)]
        #vis use #name;
    }
}

/// Supertrait bounds of `item` that remain on the erased trait: everything
/// that was not resolved (auto traits, lifetimes, ...).
pub(crate) fn unresolved_bounds<'a>(
    item: &'a ItemTrait,
    supertraits: &'a [Supertrait],
) -> impl Iterator<Item = &'a TypeParamBound> + 'a {
    item.supertraits
        .iter()
        .filter(move |bound| match needs_resolving(bound) {
            Some(path) => !supertraits
                .iter()
                .any(|s| s.item.ident == *last_ident(path)),
            None => true,
        })
}

/// A `self` token sharing the receiver's hygiene context. Tokens that pass
/// through the callback `macro_rules!` get a different syntax context than
/// `Span::call_site()`, and `self` is hygienic, so a plain `quote!(self)`
/// would not resolve to the method's receiver.
pub(crate) fn self_token(receiver: &syn::Receiver) -> TokenStream {
    let span = receiver.self_token.span;
    quote_spanned!(span=> self)
}
