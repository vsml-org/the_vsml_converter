#!/bin/bash
set -euo pipefail

export WGPU_BACKEND=vulkan
export RUST_BACKTRACE=full

EXAMPLES_DIR="$1"
OUT_ROOT="$2"

mkdir -p "${OUT_ROOT}"

echo "Running cargo test..."
VSML_VRT_OUTPUT_PATH="${OUT_ROOT}/cargo_test" cargo test --release -- --nocapture vrt

cargo build -p vsml_cli --release --locked

VSML_EXECUTABLE="$(pwd)/target/release/vsml"

find "${EXAMPLES_DIR}" -mindepth 1 -maxdepth 1 -type d | while read -r dir; do
  name="$(basename "$dir")"
  echo "Processing example: $name"
  mkdir -p "${OUT_ROOT}/${name}"

  # Pick the first .vsml file in the subdirectory
  vsml_file="$(find "$dir" -maxdepth 1 -type f -name '*.vsml' | head -n 1 || true)"

  if [ -z "$vsml_file" ]; then
    echo "  -> No .vsml file found in $dir"
    continue
  fi

  echo "  -> Running vsml_cli on: $vsml_file"
  "$VSML_EXECUTABLE" "$vsml_file" \
    --output "${OUT_ROOT}/${name}/spectrogram.png" \
    --overwrite \
    "--experimental-ffmpeg-output-option=-filter_complex '[0:v]fps=1[v_out];[1:a]showspectrumpic=s=1920x1080:legend=1[spec_out]' -map [v_out] ${OUT_ROOT}/${name}/%04d.png -map [spec_out]"
done
