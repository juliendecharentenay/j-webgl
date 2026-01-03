#!/bin/bash

export OLD_VERSION=0.0.1
export NEW_VERSION=0.0.2

export VERSION=v$NEW_VERSION

sed -s -i -e "s/^version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" \
  j-webgl/Cargo.toml \
  j-webgl-macro-make-renderer/Cargo.toml

git add \
  j-webgl/Cargo.toml \
  j-webgl-macro-make-renderer/Cargo.toml
git commit -m "Pre-release update version number"
git push

git tag -a $VERSION -m "Version $NEW_VERSION"
git push origin $VERSION

cargo publish -p j-webgl
cargo publish -p j-webgl-macro-make-renderer
