#!/bin/bash

# uses rustfmt to check all rust files

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# https://www.shellcheck.net/wiki/SC2044

find "$SCRIPT_DIR/.." -type f -name '*.rs' -print0 | while IFS= read -r -d '' file
do
  file_path=$(realpath "$file")
  # echo "$file_path"
  rustfmt --check "$file_path"
done
