#!/usr/bin/env bash
set -e

check_protoc_version () {
  this_version=$(protoc --version)
  return $([ "libprotoc $PROTOC_VERSION" = "$this_version" ])
}

if check_protoc_version; then
    echo "[SUCCESS] protobuf version $PROTOC_VERSION detected."
    exit
fi

echo "[INFO] Installing protobuf $PROTOC_VERSION to $HOME/protobuf..."

cd "$TMPDIR"
wget "https://github.com/protocolbuffers/protobuf/releases/download/v$PROTOC_VERSION/protoc-$PROTOC_VERSION-linux-x86_64.zip"
unzip -o -d "$HOME/protobuf" "protoc-$PROTOC_VERSION-linux-x86_64.zip"

echo "[SUCCESS] Done."
