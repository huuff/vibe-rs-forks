# vibe-rs-forks

Forks of upstream Rust projects, each patched with a missing feature, exported
as nix packages (`nix build .#<name>`, or `overlays.default`).

## Layout

- `forks/<name>/` — one self-contained fork per directory: a plain copy of
  the upstream source, committed in this repo. Each keeps its own
  `Cargo.toml`/`Cargo.lock`; there is no root Cargo workspace (Cargo cannot
  nest workspaces, and most upstreams are workspaces).
- `nix/fork.nix` — generic `buildRustPackage` for any fork; `flake.nix`
  discovers `forks/*` automatically, no registration needed.
- `nix/forks/<name>.nix` — optional per-fork overrides merged into the build
  (`cargoLock.outputHashes` for git deps, `nativeBuildInputs`, features).

## Adding a fork

```sh
git clone --depth 1 <upstream-url> forks/<name> && rm -rf forks/<name>/.git
nix build .#<name>          # fails on git deps: add outputHashes in nix/forks/<name>.nix
```

Note the upstream URL and commit in `forks/<name>/FORK.md` so it can be
re-synced later. Keep patches minimal and in upstream style; don't reformat
or clippy-clean upstream code. Run clippy by hand:
`cargo clippy --manifest-path forks/<name>/Cargo.toml`.

## Dev

`devenv shell` (or `direnv allow`) installs the git hooks: secrets scan,
nix/shell linters, `cargo fmt --check` per touched fork, Conventional Commits.
`devenv test` evaluates the flake. Never bypass hooks with `--no-verify`.

devenv `scripts.*` with real logic: use nushell (`package = pkgs.nushell`);
CLI args need `def --wrapped main [...args]`, interpolation is `$"(...)"`.
