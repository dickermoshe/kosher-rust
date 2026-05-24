#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

cd "$repo_root"

export CARGO_TERM_COLOR="${CARGO_TERM_COLOR:-always}"
export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"

check_package() {
  local package="$1"
  local feature_args="${2:-}"
  local -a cmd=(cargo check -p "$package" --all-targets --verbose)

  if [[ -n "$feature_args" ]]; then
    local -a args
    read -r -a args <<< "$feature_args"
    cmd+=("${args[@]}")
  fi

  echo
  echo "+ ${cmd[*]}"
  "${cmd[@]}"
}

standard_packages=(
  astronomical-calculator
  hebrew-holiday-calendar
  limudim-calendar
)

standard_feature_combinations=(
  ""
  "--no-default-features"
  "--no-default-features --features defmt"
  "--all-features"
)

for package in "${standard_packages[@]}"; do
  for features in "${standard_feature_combinations[@]}"; do
    check_package "$package" "$features"
  done
done

limudim_wasm_feature_combinations=(
  ""
  "--no-default-features"
)

for features in "${limudim_wasm_feature_combinations[@]}"; do
  check_package limudim-wasm "$features"
done

zmanim_feature_combinations=(
  ""
  "--no-default-features"
  "--no-default-features --features __test-spa-refraction"
  "--no-default-features --features defmt"
  "--no-default-features --features defmt,__test-spa-refraction"
  "--all-features"
)

for features in "${zmanim_feature_combinations[@]}"; do
  check_package zmanim-calculator "$features"
done
