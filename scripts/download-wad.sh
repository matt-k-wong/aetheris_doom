#!/usr/bin/env bash
# Download Freedoom Phase 1 WAD for out-of-the-box testing.
# Bump FREEDOOM_VERSION when upgrading to a newer release.
set -euo pipefail

FREEDOOM_VERSION="0.13.0"
FREEDOOM_ZIP="freedoom-${FREEDOOM_VERSION}.zip"
FREEDOOM_URL="https://github.com/freedoom/freedoom/releases/download/v${FREEDOOM_VERSION}/${FREEDOOM_ZIP}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
TARGET_WAD="${REPO_ROOT}/freedoom1.wad"

if [[ -f "${TARGET_WAD}" ]]; then
    echo "freedoom1.wad already exists at ${TARGET_WAD} — skipping download."
    exit 0
fi

echo "Downloading Freedoom v${FREEDOOM_VERSION} from ${FREEDOOM_URL} ..."

TMPDIR="$(mktemp -d)"
trap 'rm -rf "${TMPDIR}"' EXIT

ZIP_PATH="${TMPDIR}/${FREEDOOM_ZIP}"

if ! curl -fL --retry 3 --retry-delay 2 -o "${ZIP_PATH}" "${FREEDOOM_URL}"; then
    echo "ERROR: Failed to download ${FREEDOOM_URL}" >&2
    exit 1
fi

echo "Extracting freedoom1.wad ..."

EXTRACT_DIR="${TMPDIR}/extract"
mkdir -p "${EXTRACT_DIR}"

if ! unzip -q "${ZIP_PATH}" -d "${EXTRACT_DIR}"; then
    echo "ERROR: Failed to unzip ${ZIP_PATH}" >&2
    exit 1
fi

WAD_SRC="${EXTRACT_DIR}/freedoom-${FREEDOOM_VERSION}/freedoom1.wad"
if [[ ! -f "${WAD_SRC}" ]]; then
    echo "ERROR: Expected ${WAD_SRC} not found in archive" >&2
    exit 1
fi

cp "${WAD_SRC}" "${TARGET_WAD}"
echo "Success: freedoom1.wad installed at ${TARGET_WAD}"
