#!/usr/bin/bash
set -e

if [ ! -d "$HOME/protobuf/lib" ]; then
    wget https://github.com/google/protobuf/archive/v3.5.1.tar.gz
    tar -xzvf v3.5.1.tar.gz
    cd protobuf-3.5.1 && ./autogen.sh && ./configure --prefix=$HOME/protobuf && make && make install
fi
