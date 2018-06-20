#!/usr/bin/bash
set -e

PROTOC_VERSION=$(cat PROTOC_VERSION)

check_protoc_version () {
    this_version=`protoc --version`
    return `[ "$PROTOC_VERSION" = "$this_version" ]`
}

if check_protoc_version; then
    echo $PROTOC_VERSION detected.
    exit
fi

wget https://github.com/google/protobuf/archive/v3.5.1.tar.gz
tar -xzvf v3.5.1.tar.gz
cd protobuf-3.5.1 && ./autogen.sh && ./configure --prefix=$HOME/protobuf && make && make install
