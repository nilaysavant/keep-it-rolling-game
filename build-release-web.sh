#!/bin/bash

echo "trunk build in release mode..."
trunk build --release
echo "trunk build in release mode... done!"

echo "cd into dist..."
cd dist

echo "fix paths (from absolute to relative) in index.html..."
sed -i -e 's/href="/href="./g' index.html
sed -i -e "s/'\//'.\//g" index.html
echo "fix paths (from absolute to relative) in index.html... done!"
