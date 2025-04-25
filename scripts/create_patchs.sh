#!/bin/bash
set -euo pipefail

# Save patches in this directory
PATCH_DIR=$(realpath "../patches")
mkdir -p "$PATCH_DIR"

generate_patch() {
  local repo_path="$1"
  local rel_name="$2"

  echo "Generating patch for: $rel_name"
  cd "$repo_path"
  git diff > "${PATCH_DIR}/${rel_name//\//_}.patch"

  # Recursively handle submodules
  git submodule update --init --recursive
  local submodules=$(git submodule status | awk '{ print $2 }')

  for submodule in $submodules; do
    local submodule_path=$(realpath "$repo_path/$submodule")
    local submodule_name=$(basename "$submodule_path")
    echo "Generating patch for submodule: $submodule_name"
    cd "$submodule_path"
    git diff > "${PATCH_DIR}/${rel_name//\//_}_${submodule_name}.patch"
  done
}

# Kick off the top-level patch
generate_patch "$(pwd)/PatternLanguage" "PatternLanguage"
