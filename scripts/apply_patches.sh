#!/bin/bash
set -euo pipefail

PATCH_DIR=$(realpath "../patches")

apply_patch() {
  local repo_path="$1"
  local rel_name="$2"
  local original_dir
  original_dir=$(pwd)

  echo "Applying patch for: $rel_name"
  cd "$repo_path"

  local patch_file="${PATCH_DIR}/${rel_name//\//_}.patch"
  if [[ -f "$patch_file" ]]; then
    git apply --allow-empty "$patch_file"
  else
    echo "Warning: Patch file not found for $rel_name"
  fi

  # Apply submodule patches
  git submodule update --init --recursive
  local submodules
  submodules=$(git submodule status | awk '{ print $2 }')

  for submodule in $submodules; do
    local submodule_path="$repo_path/$submodule"
    local submodule_name
    submodule_name=$(basename "$submodule_path")
    local sub_patch_file="${PATCH_DIR}/${rel_name//\//_}_${submodule_name}.patch"

    echo "Applying patch for submodule: $submodule_name"
    cd "$submodule_path"

    if [[ -f "$sub_patch_file" ]]; then
      git apply --allow-empty "$sub_patch_file"
    else
      echo "Warning: Patch file not found for submodule $submodule_name"
    fi
  done

  cd "$original_dir"
}

apply_patch "PatternLanguage" "PatternLanguage"
