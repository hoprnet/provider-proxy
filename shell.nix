{ pkgs ? import <nixpkgs> {}, ... }:
let
  linuxPkgs = with pkgs; lib.optional stdenv.isLinux (
    inotifyTools
  );
  macosPkgs = with pkgs; lib.optional stdenv.isDarwin (
    with darwin.apple_sdk.frameworks; [
      # macOS file watcher support
      CoreFoundation
      CoreServices
    ]
  );
in
with pkgs;
mkShell {
  buildInputs = [
    envsubst
    nodejs-18_x
    (yarn.override { nodejs = nodejs-18_x; })

    ## rust for development and required utils
    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
    wasm-pack # v0.11.1
    binaryen # v113 (includes wasm-opt)
    wasm-bindgen-cli # v0.2.83
    pkg-config

    macosPkgs
    linuxPkgs
  ];
}
