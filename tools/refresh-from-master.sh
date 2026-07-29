#!/usr/bin/env bash
# refresh-from-master.sh — refresh this binding's generated surface against the latest MEOS API,
# end to end, with one command:
#
#   tools/refresh-from-master.sh
#
# It runs the shared MobilityDB/MEOS-API refresh-binding.sh over this repo — deriving the catalog
# and libmeos from the latest MobilityDB master and regenerating this binding's surface. The
# per-binding last leg is in tools/refresh.conf.
#
# All of refresh-binding.sh's options pass through, e.g.:
#   tools/refresh-from-master.sh --mdb ~/src/MobilityDB   # refresh against a local MobilityDB branch
#   tools/refresh-from-master.sh --skip-tests             # regenerate + build, skip the tests
#
# MEOSAPI=<path> uses an existing MEOS-API checkout (any branch); otherwise MEOS-API master is
# cloned into the work dir. WORK_DIR overrides the scratch location (default <repo>/.meos-chain).
set -euo pipefail

HERE="$(cd "$(dirname "$0")/.." && pwd)"
WORK="${WORK_DIR:-$HERE/.meos-chain}"
MEOSAPI="${MEOSAPI:-}"

if [ -z "$MEOSAPI" ]; then
  MEOSAPI="$WORK/MEOS-API"
  mkdir -p "$WORK"
  if [ -d "$MEOSAPI/.git" ]; then
    git -C "$MEOSAPI" fetch --quiet https://github.com/MobilityDB/MEOS-API master
    git -C "$MEOSAPI" checkout --quiet FETCH_HEAD
  else
    git clone --quiet https://github.com/MobilityDB/MEOS-API "$MEOSAPI"
  fi
fi

exec "$MEOSAPI/tools/refresh-binding.sh" \
  --binding "$HERE" --meos-api "$MEOSAPI" --work-dir "$WORK" "$@"
