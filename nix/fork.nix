# Generic builder for one fork living in forks/<name>/.
# Per-fork tweaks (git-dependency hashes, native deps, features) go in
# nix/forks/<name>.nix, which is callPackage'd and merged into the
# buildRustPackage arguments.
{
  lib,
  rustPlatform,
  callPackage,
  name,
}:

let
  src = ../forks/${name};
  manifest = lib.importTOML (src + "/Cargo.toml");
  overridesFile = ./forks/${name}.nix;
  overrides = if builtins.pathExists overridesFile then callPackage overridesFile { } else { };
in
rustPlatform.buildRustPackage (
  {
    pname = name;
    version = manifest.package.version or manifest.workspace.package.version or "0-unstable";

    src = lib.cleanSource src;
    cargoLock.lockFile = src + "/Cargo.lock";

    meta = {
      description = manifest.package.description or "Fork of ${name}";
      mainProgram = name;
    };
  }
  // overrides
)
