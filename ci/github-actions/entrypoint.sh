#!/bin/bash

sudo apt install -y libglfw3-dev xorg-dev libx11-dev libxss-dev libxxf86vm-dev libxkbfile-dev libxv-dev gcc-mingw-w64-x86-64 musl

set -eu

set_output() {
  echo "$1=$2" >> "$GITHUB_OUTPUT"
}
info() {
  echo "::info::$*"
}
warn() {
  echo "::warning file=entrypoint.sh::$*"
}
error() {
  echo "::error file=entrypoint.sh::$*"
}

# For backwards compatible also accept environment variable names, but parse all inputs in github
# action format
export RUSTTARGET="${INPUT_RUSTTARGET:-${RUSTTARGET:-}}"
EXTRA_FILES="${INPUT_EXTRA_FILES:-${EXTRA_FILES:-}}"
# SRC_DIR is handled in build.sh
ARCHIVE_TYPES="${INPUT_ARCHIVE_TYPES:-${ARCHIVE_TYPES:-}}"
ARCHIVE_NAME="${INPUT_ARCHIVE_NAME:-${ARCHIVE_NAME:-}}"
PRE_BUILD="${INPUT_PRE_BUILD:-${PRE_BUILD:-}}"
POST_BUILD="${INPUT_POST_BUILD:-${POST_BUILD:-}}"
export MINIFY="${INPUT_MINIFY:-${MINIFY:-}}"
export TOOLCHAIN_VERSION="${INPUT_TOOLCHAIN_VERSION:-${TOOLCHAIN_VERSION:-}}"
UPLOAD_MODE="${INPUT_UPLOAD_MODE:-${UPLOAD_MODE:-release}}"

if [ -z "${CMD_PATH+x}" ]; then
  export CMD_PATH=""
fi

OUTPUT_DIR="/github/workspace/output"
sudo mkdir -p "$OUTPUT_DIR"

# Link repo
PROJECT_ROOT="/rust/build/${GITHUB_REPOSITORY}"
sudo mkdir -p "$PROJECT_ROOT"
sudo rmdir "$PROJECT_ROOT"
sudo ln -s "$GITHUB_WORKSPACE" "$PROJECT_ROOT"
cd "$PROJECT_ROOT"

# Build
if ! FILE_LIST=$(ci/github-actions/build.sh "$OUTPUT_DIR"); then
  error "Build failed"
  exit 1
fi

EVENT_DATA=$(cat "$GITHUB_EVENT_PATH")
echo "$EVENT_DATA" | jq .
UPLOAD_URL=$(echo "$EVENT_DATA" | jq -r .release.upload_url)
UPLOAD_URL=${UPLOAD_URL/\{?name,label\}/}
RELEASE_NAME=$(echo "$EVENT_DATA" | jq -r .release.tag_name)
PROJECT_NAME=$(basename "$GITHUB_REPOSITORY")
NAME="${ARCHIVE_NAME:-${PROJECT_NAME}_${RELEASE_NAME}_${RUSTTARGET}}"
ARCHIVE_TYPES="${ARCHIVE_TYPES:-"zip"}"
EXTRA_FILES="${EXTRA_FILES:-""}"

if [ -z "${EXTRA_FILES+x}" ]; then
  warn "EXTRA_FILES not set"
else
  for file in $(echo -n "${EXTRA_FILES}" | tr " " "\n"); do
    sudo cp --parents "$file" "$OUTPUT_DIR"
  done
fi

cd "$OUTPUT_DIR"

if [ -n "${EXTRA_FILES+0}" ]; then
  FILE_LIST="${FILE_LIST} ${EXTRA_FILES}"
fi

FILE_LIST=$(echo "${FILE_LIST}" | awk '{$1=$1};1')

info "Packing files: $FILE_LIST"

for ARCHIVE_TYPE in $ARCHIVE_TYPES; do
  ARCHIVE="tmp.${ARCHIVE_TYPE}"

  # shellcheck disable=SC2086
  case $ARCHIVE_TYPE in
    "zip")
      sudo zip -9r $ARCHIVE ${FILE_LIST}
    ;;

    "tar"|"tar.gz"|"tar.bz2"|"tar.xz"|"tar.zst")
      sudo tar caf $ARCHIVE ${FILE_LIST}
    ;;

    *)
      error "The given archive type '${ARCHIVE_TYPE}' is not supported; please use a supported archive type."
      continue
  esac

  FILE_NAME="${NAME}.${ARCHIVE/tmp./}"
  printf "%s %s" "$(sha256sum "${ARCHIVE}" | cut -d ' ' -f 1)" "$FILE_NAME" > "${ARCHIVE}.sha256sum"
  CHECKSUM_FILE_NAME="${FILE_NAME}.sha256sum"

  sudo mv "$ARCHIVE" "$FILE_NAME"
  sudo mv "${ARCHIVE}.sha256sum" "$CHECKSUM_FILE_NAME"
  set_output "BUILT_ARCHIVE" "output/${FILE_NAME}"
  set_output "BUILT_CHECKSUM" "output/${CHECKSUM_FILE_NAME}"

  if [ "$UPLOAD_MODE" = "release" ]; then
    if [ "$UPLOAD_URL" = "null" ]; then
      warn "UPLOAD_MODE \"release\" was specified but no URL to upload to could be detected"
    else
      curl \
        --fail-with-body -sS \
        -X POST \
        --data-binary @"${FILE_NAME}" \
        -H 'Content-Type: application/octet-stream' \
        -H "Authorization: Bearer ${GITHUB_TOKEN}" \
        "${UPLOAD_URL}?name=${FILE_NAME}"

      curl \
        --fail-with-body -sS \
        -X POST \
        --data-binary @"$CHECKSUM_FILE_NAME" \
        -H 'Content-Type: text/plain' \
        -H "Authorization: Bearer ${GITHUB_TOKEN}" \
        "${UPLOAD_URL}?name=${CHECKSUM_FILE_NAME}"
    fi
  fi
done