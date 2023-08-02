#! /usr/bin/env bash

mkdir -p $out/bin

apps=( chisel cast forge anvil )

for app in "${apps[@]}"
do
  echo "Installing $app from $src"

  cp $src/$app -t $out/bin/
  chmod +x $out/bin/$app
done
