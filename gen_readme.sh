#!/bin/bash

# https://stackoverflow.com/a/29613573/5069285
quoteSubst() {
  IFS= read -d '' -r < <(sed -e ':a' -e '$!{N;ba' -e '}' -e 's/[&/\]/\\&/g; s/\n/\\&/g' <<<"$1")
  printf %s "${REPLY%$'\n'}"
}

cp README_template.md README.md

sed -i "s/\[diy\]/$(quoteSubst "$(cat examples/diy.rs)")/g;s/\[high-level\]/$(quoteSubst "$(cat examples/high-level.rs)")/g" README.md
