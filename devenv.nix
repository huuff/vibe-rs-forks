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
    ripsecrets.enable = true; # scans staged changes for API keys/tokens
    detect-private-keys.enable = true;

    # --- hygiene ---
    check-added-large-files.enable = true;
    check-merge-conflicts.enable = true;
    end-of-file-fixer.enable = true;
    trim-trailing-whitespace.enable = true;

    # --- static analysis: nix ---
    nixfmt.enable = true; # RFC 166 style; nixfmt >= 1.0 (nixfmt-rfc-style is the deprecated alias)
    statix.enable = true;
    deadnix.enable = true;

    # --- static analysis: shell ---
    shellcheck.enable = true;
    shfmt.enable = true;

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
