# Fork of leptos_i18n

- Upstream: https://github.com/Baptistemontan/leptos_i18n
- Base: main at commit 1607c41 (workspace version 0.6.2, after the v0.6.2 tag)

Everything outside the sections below is a plain copy of upstream. Keep
patches minimal and in upstream style; add a section here for each new one.

## Differences from upstream

### `prefix_default_locale` on `I18nRoute`

Upstream never prefixes the default locale: `/counter` is the `en` page and
switching to `en` strips the prefix. The `prefix_default_locale=true` prop
(default `false`) makes every URL carry a locale. The unprefixed routes still
match, but only to redirect: on SSR the view renders a `<Redirect>`, on the
client the prefix-correcting effect navigates with `replace`. Path building
and locale switching always push the locale segment.

- Code: `leptos_i18n_router/src/routing.rs`. `locale_of_prefix` is the one
  place that decides what a missing prefix means; `prefix_default` is
  threaded from `I18nRoute` through `i18n_routing`, `I18nNestedRoute`,
  `I18nRouteMatch` and `view_wrapper` into `get_new_path`, `maybe_redirect`
  and the two effects.
- Prop: `leptos_i18n_router/src/components.rs`.
- Tests: `mod tests` at the bottom of `routing.rs` (`cargo test -p
  leptos_i18n_router`). They build a `Locale` with `declare_locales!` and a
  `Location` by hand to drive `get_new_path` directly; the effects and the
  redirect are browser-only and untested.
- User docs: "Prefixing the Default Locale" in `docs/book/src/usage/07_router.md`.
