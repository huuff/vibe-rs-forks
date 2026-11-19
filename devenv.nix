# Add `lib`, `config`, etc. to the lambda args when first needed (deadnix rejects
# unused args, statix rejects empty `{ ... }` patterns).
{ pkgs, ... }:

let
  # Each fork keeps its own Cargo.toml, so run cargo fmt once per fork that
  # has staged .rs changes instead of the stock single-manifest rustfmt hook.
  cargo-fmt-forks = pkgs.writeShellApplication {
    name = "cargo-fmt-forks";
    runtimeInputs = [
      pkgs.cargo
      pkgs.rustfmt
    ];
    text = ''
      printf '%s\n' "$@" | sed -n 's#^\(forks/[^/]*\)/.*#\1#p' | sort -u | while read -r fork; do
        cargo fmt --all --manifest-path "$fork/Cargo.toml" -- --check
      done
    '';
  };
in
{
  packages = [
    # Uncomment if a fork needs runtime secrets (see "Runtime secrets" in the new-project skill):
    # pkgs.sops
    # pkgs.age
  ];

  languages.rust.enable = true;
  languages.nix.enable = true;

  git-hooks.hooks = {
    # --- secrets: never commit credentials ---
    # Upstream sqlx ships fake credentials and a test PEM key in its sources.
    ripsecrets = {
      enable = true; # scans staged changes for API keys/tokens
      excludes = [
        "^forks/sqlx/sqlx-(core/src/net/tls/tls_rustls|(mysql|postgres)/src/options/(mod|parse))\\.rs$"
      ];
    };
    detect-private-keys = {
      enable = true;
      excludes = [
        "^forks/sqlx/sqlx-(core/src/net/tls/tls_rustls|(mysql|postgres)/src/options/mod)\\.rs$"
      ];
    };

    # --- hygiene ---
    check-added-large-files = {
      enable = true;
      excludes = [ "^forks/sqlx/sqlx-cli/tests/assets/sample_metadata\\.json$" ];
    };
    check-merge-conflicts.enable = true;
    # Golden files under forks/*/tests are compared byte-for-byte; leave them alone.
    end-of-file-fixer = {
      enable = true;
      excludes = [ "\\.(stdout|stderr)$" ];
    };
    trim-trailing-whitespace = {
      enable = true;
      excludes = [ "\\.(stdout|stderr)$" ];
    };

    # --- static analysis: nix ---
    nixfmt.enable = true; # RFC 166 style; nixfmt >= 1.0 (nixfmt-rfc-style is the deprecated alias)
    statix.enable = true;
    deadnix.enable = true;

    # --- static analysis: shell ---
    # Upstream maintainer scripts are not ours to lint.
    shellcheck = {
      enable = true;
      excludes = [ "^forks/sqlx/.*\\.sh$" ];
    };
    shfmt = {
      enable = true;
      excludes = [ "^forks/sqlx/.*\\.sh$" ];
    };

    # --- static analysis: rust ---
    # clippy is deliberately not a hook: upstream code is rarely warning-free
    # and -D warnings would block every fork. Run it by hand per fork.
    cargo-fmt-forks = {
      enable = true;
      name = "cargo fmt --check (per fork)";
      entry = pkgs.lib.getExe cargo-fmt-forks;
      files = "^forks/.*\\.rs$";
    };

    # --- commit messages: Conventional Commits (feat:, fix:, chore:, ...) ---
    commitizen.enable = true;
  };

  enterTest = ''
    nix flake check --no-build
  '';
}
