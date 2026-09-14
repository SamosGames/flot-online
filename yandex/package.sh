#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
dist_dir="$repo_root/client/dist"
archive="$repo_root/yandex/yandex-games.zip"

env NO_COLOR=false make -C "$repo_root/client" release
rm -f "$archive"
(cd "$dist_dir" && zip -qr "$archive" .)
printf 'Created %s\n' "$archive"
