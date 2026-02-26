#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

WORLD_ROOMS="$TMP_DIR/world.rooms.keys"
WORLD_OBJECTS="$TMP_DIR/world.objects.keys"

cd "$ROOT_DIR"

if ! command -v rg >/dev/null 2>&1; then
  echo "error: ripgrep (rg) is required" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required" >&2
  exit 1
fi

rg -o 'Room::new\("[^"]+"' src/game/world.rs \
  | sed -E 's/Room::new\("([^"]+)"/\1/' \
  | sort -u > "$WORLD_ROOMS"

rg -o 'Object::new\("[^"]+"' src/game/world.rs \
  | sed -E 's/Object::new\("([^"]+)"/\1/' \
  | sort -u > "$WORLD_OBJECTS"

echo "i18n alignment report"
echo "project: $ROOT_DIR"
echo
echo "world unique counts:"
echo "  rooms:   $(wc -l < "$WORLD_ROOMS" | tr -d ' ')"
echo "  objects: $(wc -l < "$WORLD_OBJECTS" | tr -d ' ')"
echo

for lang in en it es; do
  LANG_ROOMS="$TMP_DIR/$lang.rooms.keys"
  LANG_OBJECTS="$TMP_DIR/$lang.objects.keys"

  jq -r '.rooms | keys[]' "data/i18n/$lang.json" | sort > "$LANG_ROOMS"
  jq -r '.objects | keys[]' "data/i18n/$lang.json" | sort > "$LANG_OBJECTS"

  ROOMS_MISSING="$(comm -23 "$WORLD_ROOMS" "$LANG_ROOMS" | wc -l | tr -d ' ')"
  ROOMS_EXTRA="$(comm -13 "$WORLD_ROOMS" "$LANG_ROOMS" | wc -l | tr -d ' ')"
  OBJECTS_MISSING="$(comm -23 "$WORLD_OBJECTS" "$LANG_OBJECTS" | wc -l | tr -d ' ')"
  OBJECTS_EXTRA="$(comm -13 "$WORLD_OBJECTS" "$LANG_OBJECTS" | wc -l | tr -d ' ')"

  echo "[$lang]"
  echo "  rooms missing in i18n (present in world): $ROOMS_MISSING"
  echo "  rooms extra in i18n (not in world):       $ROOMS_EXTRA"
  echo "  objects missing in i18n (present in world): $OBJECTS_MISSING"
  echo "  objects extra in i18n (not in world):       $OBJECTS_EXTRA"
  echo
done
