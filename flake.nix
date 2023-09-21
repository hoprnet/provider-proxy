{
  description = "provider proxy";

  inputs.nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
  inputs.rust-overlay.url = github:oxalica/rust-overlay;

  inputs.rust-overlay.inputs = {
    nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-parts, rust-overlay, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      perSystem = { config, self', inputs', system, ... }:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        {
          devShells.default = import ./shell.nix {
            inherit pkgs;
          };
        };
      systems = [ "x86_64-linux" "aarch64-darwin" ];
      flake = {
        overlays = [
          rust-overlay.overlays
        ];
      };
    };
}
