{
  description = "Little Bo Peep website";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain: toolchain.default.override { 
            extensions = [ "rust-analyzer" ];
            targets = [ "wasm32-unknown-unknown" ];
          }
        );
      in with pkgs; {
        # Development environment
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            sass
            tailwindcss
            rust
            binaryen
          ];
        };
      }
    );
}