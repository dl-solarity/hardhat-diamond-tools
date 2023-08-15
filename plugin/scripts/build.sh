#! /bin/bash
#
# Build script for plugin.

set -e

# if PLUGIN_OUT_DIR is not set, set it to `pkg`
if [ -z "$PLUGIN_OUT_DIR" ]; then
  PLUGIN_OUT_DIR=pkg
fi
echo "PLUGIN_OUT_DIR: $PLUGIN_OUT_DIR"

# if PLUGIN_RELEASE is not set, set it to `false`
if [ -z "$PLUGIN_RELEASE" ]; then
  PLUGIN_RELEASE=false
fi

# if PLUGIN_WASM_FILE is not set, set it to defualt
if [ -z "$PLUGIN_WASM_FILE" ]; then
    folder="debug"
    if [ "$PLUGIN_RELEASE" = true ] ; then
      folder="release"
    fi
    PLUGIN_WASM_FILE=../target/wasm32-unknown-unknown/$folder/diamond_tools_plugin.wasm
fi
echo "PLUGIN_WASM_FILE: $PLUGIN_WASM_FILE"

# if PLUGIN_SRC_DIR is not set, set it to `.`
if [ -z "$PLUGIN_SRC_DIR" ]; then
  PLUGIN_SRC_DIR=$(pwd)
fi

cargo_params=""
if [ "$PLUGIN_RELEASE" = true ] ; then
  cargo_params="--release"
fi

# build wasm
# NOTE: cargo build --release 
cargo build \
  --package=diamond-tools-plugin \
  --target wasm32-unknown-unknown \
  --message-format json-render-diagnostics \
  -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target wasm32-unknown-unknown \
  $cargo_params

# build js files for wasm
wasm-bindgen \
    --target nodejs \
    --out-dir $PLUGIN_OUT_DIR \
    $PLUGIN_WASM_FILE

# optimize wasm using `wasm-opt`
if [ "$PLUGIN_RELEASE" = true ] ; then
  wasm-opt \
    -Oz \
    -o $PLUGIN_OUT_DIR/diamond_tools_plugin_bg.wasm \
    $PLUGIN_OUT_DIR/diamond_tools_plugin_bg.wasm
fi

# copy `index.js` template to `pkg`
cat > $PLUGIN_OUT_DIR/index.js << EOF
const { run } = require("./diamond_tools_plugin");
run();
EOF

# copy `README.md` to `pkg`
cp $PLUGIN_SRC_DIR/README.md $PLUGIN_OUT_DIR

# copy `readme`, `name`, `description, `version`, `license` from `package.json` to `pkg/package.json`
cat $PLUGIN_SRC_DIR/package.json | jq '. | {readme, name, description, version, license, repository}' > $PLUGIN_OUT_DIR/package.json

# add `main` field to `pkg/package.json` using jq
cat $PLUGIN_OUT_DIR/package.json | jq '. + {main: "index.js"}' > $PLUGIN_OUT_DIR/package.json.tmp
mv $PLUGIN_OUT_DIR/package.json.tmp $PLUGIN_OUT_DIR/package.json