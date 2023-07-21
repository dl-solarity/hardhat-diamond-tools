{ pkgs, craneLib, ... }:
craneLib.buildPackage {
  src = craneLib.cleanCargoSource (craneLib.path ../.);

  cargoExtraArgs = "--package=diamond-merge-cli";

  buildInputs = [
    # add your dependencies here
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    pkgs.libiconv
  ];
}
