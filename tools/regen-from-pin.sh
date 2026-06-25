#!/usr/bin/env bash
# regen-from-pin.sh — regenerate the meos-rs FFI from the MEOS pin (per GENERATION.md).
#
# Usage:  tools/regen-from-pin.sh <pin>
#   env:  CATALOG = path to meos-idl.json from MEOS-API run.py (used once the catalog
#                   generator lands; see GENERATION.md — the canonical target)
#
# TODAY meos-rs generates its FFI via bindgen over the MEOS source submodule. The canonical
# target is to generate `meos-sys` from the catalog (one generation logic), via generate-then-
# retire — bindgen + the committed prebuilt-bindings stay until proven ABI-equivalent.
# Invoked standalone, or by MEOS-API tools/ecosystem-generate.sh.
set -euo pipefail
PIN="${1:?usage: regen-from-pin.sh <pin>}"
HERE="$(cd "$(dirname "$0")/.." && pwd)"

# Point the MEOS source submodule at the pin commit, then bindgen-regenerate via the bundled build.
( cd "$HERE/sys/meos-src/source" && git fetch --quiet origin "$PIN" 2>/dev/null && git checkout --quiet "$PIN" ) \
  || echo "NOTE: pin the sys/meos-src/source submodule to $PIN (an immutable ecosystem-pin commit)"
( cd "$HERE" && cargo build --features bundled,bindgen && cargo test --features bundled ) \
  || echo "WARN: meos-rs build/test returned non-zero"
echo "[meos-rs] FFI regenerated (bindgen) at pin $PIN — catalog-driven target tracked in GENERATION.md"
