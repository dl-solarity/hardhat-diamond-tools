{ system ? builtins.currentSystem
, pkgs ? import <nixpkgs> {
    inherit system;
  }
}:
let
  downloadMap = {
    "x86_64-darwin" = "darwin_amd64";
    "x86_64-linux" = "linux_amd64";
    "aarch64-linux" = "linux_arm64";
    "aarch64-darwin" = "darwin_arm64";
  };
  sha256Map = with pkgs; {
    "x86_64-darwin" = lib.fakeSha256;
    "x86_64-linux" = lib.fakeSha256;
    "aarch64-linux" = lib.fakeSha256;
    "aarch64-darwin" = "sha256-AEzjF62Zhv9AxQ7qi+n/616iaeFZ52j+BCdWD2+Tu1A=";
  };
in
pkgs.stdenv.mkDerivation rec {
  pname = "foundry";
  version = "nightly-cc5637a979050c39b3d06bc4cc6134f0591ee8d0";
  src = pkgs.fetchzip {
    url = "https://github.com/foundry-rs/foundry/releases/download/${version}/foundry_nightly_${downloadMap.${system}}.tar.gz";
    sha256 = sha256Map.${system};
    stripRoot = false;
  };
  phases = [
    "installPhase"
  ];

  installPhase = builtins.readFile ./install.sh;
}

