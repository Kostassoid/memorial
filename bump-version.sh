#!/usr/bin/env bash

set -e

CURRENT_VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' memorial-cli/Cargo.toml | head -n1)

SEMVER_PATTERN='([0-9]+)\.([0-9]+)\.([0-9]+)(.*)'

[[ $CURRENT_VERSION =~ $SEMVER_PATTERN ]]

MAJOR=${BASH_REMATCH[1]}
MINOR=${BASH_REMATCH[2]}
PATCH=${BASH_REMATCH[3]}
# shellcheck disable=SC2034
SUFFIX=${BASH_REMATCH[4]}

# shellcheck disable=SC2004
case $1 in
  "major")
    MAJOR=$(($MAJOR+1))
    MINOR="0"
    PATCH="0"
    ;;
  "minor")
    MINOR=$(($MINOR+1))
    PATCH="0"
    ;;
  "patch")
    PATCH=$(($PATCH+1))
    ;;
esac

RELEASE_VERSION=$(printf "%d.%d.%d" "$MAJOR" "$MINOR" "$PATCH")

echo "Current version: v${CURRENT_VERSION}"
echo "New version: v${RELEASE_VERSION}"

TODAY=$(date +'%Y-%m-%d')

CARGO_FILES=("memorial-cli/Cargo.toml" "memorial-core/Cargo.toml" "memorial-macros/Cargo.toml")

for F in "${CARGO_FILES[@]}"; do
  sed -i "s/version = \"$CURRENT_VERSION\"/version = \"$RELEASE_VERSION\"/" "$F"
done

sed -i "s/v$CURRENT_VERSION/v$RELEASE_VERSION/g" README.md
sed -i "s/Unreleased/v$RELEASE_VERSION/" CHANGELOG.md || true
sed -i "s/ReleaseDate/$TODAY/" CHANGELOG.md || true
