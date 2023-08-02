{ pkgs, craneLib, ... }:
craneLib.buildPackage {
  src = craneLib.cleanCargoSource (craneLib.path ../.);

  cargoExtraArgs = "--package=diamond-tools-cli";

  buildInputs = [
    # add your dependencies here
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    pkgs.libiconv
  ];
}
