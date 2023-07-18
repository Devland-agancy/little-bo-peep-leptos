{
  description = "Dev environment";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
    devshell.url = "github:numtide/devshell?rev=696acc29668b644df1740b69e1601119bf6da83b";
    crate2nix = {
      url = "github:kolloch/crate2nix?rev=d9854e53b5f17dc2a7fb5e8c7a32bb2299bd0a0a";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, devshell, crate2nix, ... }:
    let name = "little-bo-peep-leptos"; in
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
            rust-overlay.overlays.default devshell.overlay
            (self: super: {
              rustc = self.rust-bin.selectLatestNightlyWith (
                toolchain: toolchain.default.override { 
                  extensions = [ "rust-analyzer" ];
                  targets = [ "wasm32-unknown-unknown" ];
                }
              );
              cargo = self.rust-bin.selectLatestNightlyWith (
                toolchain: toolchain.default.override { 
                  extensions = [ "rust-analyzer" ];
                  targets = [ "wasm32-unknown-unknown" ];
                }
              );
            })
          ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        inherit (import "${crate2nix}/tools.nix" { inherit pkgs; })
          generatedCargoNix;
        
        project = import
          (generatedCargoNix {
            inherit name;
            src = ./.;
          }) { inherit pkgs; };
      in rec {
        packages.${name} = project.rootCrate.build;
        defaultPackage = packages.${name};
        packages.container = pkgs.dockerTools.buildImage {
          inherit name;
          tag = packages.${name}.version;
          created = "now";
          contents = packages.${name};
          config.Cmd = [ "${packages.${name}}/bin/little-bo-peep-leptos" ];
        };

        apps.${name} = flake-utils.lib.mkApp {
          inherit name;
          drv = packages.${name};
        };
        defaultApp = apps.${name};
        
        devShell = pkgs.devshell.mkShell {
          imports = [ (pkgs.devshell.importTOML ./devshell.toml) ];
          env = [
            {
              name = "RUST_SRC_PATH";
              value = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            }
          ];
        };
      }
    );
}