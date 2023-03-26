#!/usr/bin/env bash

info() {
  echo "::info $*" >&2
}

error() {
  echo "::error file=build.sh:: $*" >&2
}

set -eu -o pipefail

crash() {
  error "Command exited with non-zero exit code"
  exit 1
}

trap 'crash' ERR
PROJECT_ROOT="rust/build/${GITHUB_REPOSITORY}"
OUTPUT_DIR="$1"

mkdir -p "$PROJECT_ROOT"
rmdir "$PROJECT_ROOT"
ln -s "$GITHUB_WORKSPACE" "$PROJECT_ROOT"
cd "$PROJECT_ROOT"

if [ -z "${SRC_DIR+0}" ]; then
  info "No SRC_DIR is set, using repo base dir"
else
  info "Switching to src dir \"$SRC_DIR\""
  cd "$SRC_DIR"
fi

BINARIES="$(cargo read-manifest | jq -r ".targets[] | select(.kind[] | contains(\"bin\")) | .name")"

OUTPUT_LIST=""
for BINARY in $BINARIES; do
  info "Building $BINARY..."

  if [ -x "./build.sh" ]; then
    OUTPUT=$(./build.sh "${CMD_PATH}" "${OUTPUT_DIR}")
  else
    rustup target add x86_64-unknown-linux-gnu
    cargo build --release --target x86_64-unknown-linux-gnu --bin "$BINARY"
    OUTPUT=$(find "target/x86_64-unknown-linux-gnu/release/" -maxdepth 1 -type f -executable \( -name "${BINARY}" -o -name "${BINARY}.*" \) -print0 | xargs -0)
  fi

  info "$OUTPUT"

  if [ "$OUTPUT" = "" ]; then
    error "Unable to find output"
    exit 1
  fi

  info "Saving $OUTPUT..."

  # shellcheck disable=SC2086
  mv $OUTPUT "$OUTPUT_DIR" || error "Unable to copy binary"

  for f in $OUTPUT; do
    OUTPUT_LIST="$OUTPUT_LIST $(basename "$f")"
  done
done
echo "$OUTPUT_LIST"
