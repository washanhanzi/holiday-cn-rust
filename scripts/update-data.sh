#!/usr/bin/env bash
set -euo pipefail
old=$(git -C holiday-cn rev-parse HEAD)
git submodule update --init --remote holiday-cn
new=$(git -C holiday-cn rev-parse HEAD)
if git -C holiday-cn diff --quiet "$old" "$new" -- '*.json'; then
  git -C holiday-cn checkout --detach "$old"
  echo "changed=false" >> "$GITHUB_OUTPUT"
else
  echo "changed=true" >> "$GITHUB_OUTPUT"
fi
