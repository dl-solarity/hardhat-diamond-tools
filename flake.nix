{
  description = "A very basic flake";

  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay/c88b28944129eeff5e819bdc21248dc07eb0625d";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        foundryPkg = pkgs.callPackage ./nix/foundry { inherit pkgs system; };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = with pkgs; lib.cleanSourceWith {
          src = ./.; # The original, unfiltered source
          filter = path: type:
            (lib.hasSuffix "\.js" path) ||
            # Default filter from crane (allow .rs files)
            (craneLib.filterCargoSources path type)
          ;
        };

        commonArgs = {
          inherit src;
          pname = "diamond-tools";
          nativeBuildInputs = with pkgs; [
            # in future add other deps
          ] ++ lib.optional stdenv.isDarwin [
            libiconv
          ];
        };

        nativeArtifacts = craneLib.buildDepsOnly (commonArgs // {
          doCheck = true;
        });

        cli = craneLib.buildPackage (commonArgs // rec {
          cargoArtifacts = nativeArtifacts;
          pname = "diamond-tools-cli";
          cargoExtraArgs = "--package=${pname}";
        });

        wasmArgs = commonArgs // rec {
          pname = "diamond-tools-plugin";
          cargoExtraArgs = "--package=${pname}";
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";

          buildInputs = with pkgs; [
            wasm-pack
            wasm-bindgen-cli
            nodejs
            cargo-generate
            cargo-expand
            binaryen
          ];
        };

        wasmArtifacts = craneLib.buildDepsOnly (wasmArgs // {
          doCheck = false;
        });

        plugin = craneLib.buildPackage (wasmArgs // {
          cargoArtifacts = wasmArtifacts;
        });
      in
      rec {
        checks = {
          # Checks that packages are build at all
          inherit cli;

          cli-doc = craneLib.cargoDoc (commonArgs // {
            cargoArtifacts = nativeArtifacts;
          });

          cli-clippy = craneLib.cargoClippy (commonArgs // {
            cargoArtifacts = nativeArtifacts;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          });

          cli-fmt = craneLib.cargoFmt (commonArgs // {
            inherit src;
          });

          plugin-clippy = craneLib.cargoClippy (wasmArgs // {
            cargoArtifacts = wasmArtifacts;
            cargoClippyExtraArgs = "-- -D warnings";
          });

          plugin-fmt = craneLib.cargoFmt (wasmArgs // {
            inherit src;
          });

          # TODO(Velnbur): add `cargo audit` check
        };

        packages = {
          inherit plugin;
          default = cli;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = cli;
        };

        formatter = pkgs.nixpkgs-fmt;

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks.${system};

          nativeBuildInputs = [
            rustToolchain
          ] ++ commonArgs.nativeBuildInputs;

          buildInputs = with pkgs; [
            rust-analyzer
            nixfmt
            rnix-lsp
            foundryPkg
            nodePackages.typescript-language-server
          ] ++ wasmArgs.buildInputs;

          shellHook = ''
            # For rust-analyzer 'hover' tooltips to work.
            export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
          '';
        };
      }
    );
}
