{ pkgs ? import <nixpkgs> { } }:

with pkgs; mkShell {
  nativeBuildInputs = [
    cargo
    cargo-make
  ] ++ (lib.optionals stdenv.isDarwin [
    libiconv
  ]);

  buildInputs = [
    rustfmt
    rust-analyzer
    clippy
  ];

  RUST_BACKTRACE = 0;
}

