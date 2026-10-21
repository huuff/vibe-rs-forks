# vibe-rs-forks

Forks of upstream Rust libraries, each patched with a missing feature. Other
projects consume them as Cargo git dependencies:

```toml
[dependencies]
<crate> = { git = "https://github.com/haf/vibe-rs-forks", branch = "main" }
```

Cargo locates the crate by package name anywhere in the repo, so no workspace
or registration is needed.

## Layout

`forks/<name>/` — one fork per directory: a plain copy of the upstream source
with its own `Cargo.toml`/`Cargo.lock`. There is no root Cargo workspace
(Cargo cannot nest workspaces, and most upstreams are workspaces).

## Adding a fork

```sh
git clone --depth 1 <upstream-url> forks/<name> && rm -rf forks/<name>/.git
```

Note the upstream URL and commit in `forks/<name>/FORK.md` so it can be
re-synced later. Keep patches minimal and in upstream style; don't reformat
or clippy-clean upstream code. Run clippy by hand:
`cargo clippy --manifest-path forks/<name>/Cargo.toml`.

## Dev

`devenv shell` (or `direnv allow`) installs the git hooks: secrets scan,
nix/shell linters, `cargo fmt --check` per touched fork, Conventional Commits.
Never bypass hooks with `--no-verify`.

devenv `scripts.*` with real logic: use nushell (`package = pkgs.nushell`);
CLI args need `def --wrapped main [...args]`, interpolation is `$"(...)"`.
