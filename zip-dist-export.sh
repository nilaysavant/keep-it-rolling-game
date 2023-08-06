#!/bin/bash

# setup exports dir for web
mkdir -p exports/web-wasm

cd dist # move into dist dir
# Compress exported web dist into a *.zip file with date time
export_path="../exports/web-wasm/web-export-$(date +"%Y-%m-%d_%H-%M-%S").zip"
echo "compressing to zip file: $export_path"
zip -r $export_path *
echo "compressing to zip file: $export_path ...done!"
