#! /bin/bash
#
# After build script for plugin.

# copy `index.js` to `pkg`
cp index.js pkg

# change `main` in package.json to `index.js`
sed -i 's/"main": "diamond_tools_plugin.js"/"main": "index.js"/g' pkg/package.json