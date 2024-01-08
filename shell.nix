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
    macosPkgs
    linuxPkgs
  ];
}
