#!/bin/bash
set -euo pipefail

export WGPU_BACKEND=vulkan
export RUST_BACKTRACE=full

cargo build -p vsml_cli --release --locked

VSML_EXECUTABLE="$(pwd)/target/release/vsml"

EXAMPLES_DIR="$1"
OUT_ROOT="$2"

mkdir -p "${OUT_ROOT}"

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

  out_mp4="/tmp/out.mp4"

  echo "  -> Running vsml_cli on: $vsml_file"
  "$VSML_EXECUTABLE" "$vsml_file" --output "$out_mp4" --overwrite

  ffmpeg -hide_banner -loglevel error -y -i "$out_mp4" -vf fps=1 "${OUT_ROOT}/${name}/%04d.png"
  ffmpeg -i "$out_mp4" -lavfi "showspectrumpic=s=1920x1080:legend=1" -y "${OUT_ROOT}/${name}/spectrogram.png"
done
