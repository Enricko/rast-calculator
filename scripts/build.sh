#!/usr/bin/env bash
# Build the calculator for the runner's own target: scripts/build.sh <target> <out-dir>
set -euo pipefail
TARGET=${1:?target} OUT=${2:?out-dir}
case "$TARGET" in
  linux-x86_64) ARCH=x86_64 ;;
  linux-aarch64) ARCH=aarch64 ;;
  *) echo "unknown target $TARGET" >&2; exit 1 ;;
esac
[ "$(uname -m)" = "$ARCH" ] || { echo "build $TARGET on a native $ARCH machine" >&2; exit 1; }
HERE=$(cd "$(dirname "$0")/.." && pwd)
VERSION=$(sed -n 's/^[[:space:]]*"version": "\([^"]*\)".*/\1/p' "$HERE/package.json" | head -n 1)
SOURCE_SHA=$(sed -n 's/^[[:space:]]*"sha256": "\([0-9a-f]*\)".*/\1/p' "$HERE/package.json" | head -n 1)
ACTUAL_SHA=$(cat "$HERE/src/main.rs" "$HERE/scripts/build.sh" "$HERE/app/index.html" | sha256sum | cut -d' ' -f1)
[ "$SOURCE_SHA" = "$ACTUAL_SHA" ] || {
  echo "package.json source.sha256 does not match the source inputs ($ACTUAL_SHA); update it" >&2
  exit 1
}
WORK=$(mktemp -d); trap 'rm -rf "$WORK"' EXIT
PKG="calculator-$VERSION"
mkdir -p "$WORK/stage/$PKG/bin"
rustc --edition 2021 -O "$HERE/src/main.rs" -o "$WORK/stage/$PKG/bin/calculator"
"$WORK/stage/$PKG/bin/calculator" --version
# The App Center app is the web UI; the CLI stays available in the terminal.
cp -R "$HERE/app" "$WORK/stage/$PKG/app"
mkdir -p "$OUT"
ARCHIVE="$OUT/$PKG-$TARGET.tar.gz"
tar --sort=name --owner=0 --group=0 --numeric-owner --mtime=@0 -C "$WORK/stage" -cf - "$PKG" | gzip -n -9 > "$ARCHIVE"
sha256sum "$ARCHIVE"
echo "archive=$ARCHIVE" >> "${GITHUB_OUTPUT:-/dev/null}"
