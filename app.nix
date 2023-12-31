
# This is where you define the derivation for your app.
# For this example, we'll build a basic website using nginx.
# Based on https://github.com/NixOS/nixpkgs/blob/5f33ded6018c9bd2e203cd6d7bad4d6a62e46c2f/pkgs/build-support/docker/examples.nix#L44
{ stdenv, writeShellScriptBin }:
let
  root = stdenv.mkDerivation {
    name = "app-static";
    src = ./.;
    buildPhase = ''
      cargo leptos build --release
    '';
    installPhase = ''
      ls
      mkdir $out
      cp -R root/* $out/
    '';
  };
in
  writeShellScriptBin "app" ''
    echo Starting build
    echo nginx exited with code $?
    exit $?
  ''