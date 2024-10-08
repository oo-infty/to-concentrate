{
  description = "Rust application environment.";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = { self', inputs', system, ... }: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) ];
        };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in {
        packages.to-concentrate = pkgs.callPackage ./pkgs/to-concentrate {};
        packages.default = self'.packages.to-concentrate;

        devShells.default = let
          mkShell = pkgs.mkShell.override { stdenv = pkgs.stdenvNoCC; };
        in
          mkShell {
            buildInputs = [ rust ];
            packages = with pkgs; [
              nil
              socat
            ];
          };

        devShells.ci = let
          mkShell = pkgs.mkShell.override { stdenv = pkgs.stdenvNoCC; };
        in
          mkShell {
            buildInputs = [ rust ];
          };
      };

      flake = {
        homeManagerModules.to-concentrate = import ./modules/home-manager/to-concentrate;
        homeManagerModules.default = self.homeManagerModules.to-concentrate;
      };
    };
}
