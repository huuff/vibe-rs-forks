{
  description = "Rust forks of upstream projects, patched with features they lack, exported as nix packages";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = f: lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});

      # Every directory under forks/ is a self-contained fork and becomes a package.
      forks = lib.attrNames (lib.filterAttrs (_: type: type == "directory") (builtins.readDir ./forks));
      mkForks = pkgs: lib.genAttrs forks (name: pkgs.callPackage ./nix/fork.nix { inherit name; });
    in
    {
      packages = forAllSystems mkForks;

      overlays.default = final: _prev: mkForks final;

      checks = forAllSystems mkForks;
    };
}
