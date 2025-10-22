#!/bin/bash

mkdir build 2>/dev/null

_RELEASE_NAME="$(uname -s  | tr '[:upper:]' '[:lower:]')-$(arch)"
_BUILD_DIR=build/hnode-miner-$_RELEASE_NAME

rm -Rf $_BUILD_DIR
mkdir $_BUILD_DIR

mkdir $_BUILD_DIR/hypernodePL
mkdir $_BUILD_DIR/work

cp hnode_miner $_BUILD_DIR/

cp hnode_miner $_BUILD_DIR/hnode_miner
cp hypernodePL/hypernodePLFunctions.h $_BUILD_DIR/hypernodePL/hypernodePLFunctions.h
cp hypernodePL/libhypernodePLFunctions.a $_BUILD_DIR/hypernodePL/libhypernodePLFunctions.a

cd build
tar -cf hnode-miner-$_RELEASE_NAME.tgz hnode-miner-$_RELEASE_NAME
