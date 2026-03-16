#!/bin/sh

IS_RELEASE=true

BUILD_TYPE=""
if [ "$IS_RELEASE" == true ]; then
  BUILD_TYPE="--release"
fi

export RUST_BACKTRACE=1

cargo ndk \
-P "26" \
-o ./src/main/jniLibs \
-t "aarch64-linux-android" \
-t "armv7-linux-androideabi" \
-t "x86_64-linux-android" \
-t "i686-linux-android" \
rustc \
--lib \
--crate-type "cdylib" \
$BUILD_TYPE \
--manifest-path="../../Cargo.toml"

if [ $? -ne 0 ]; then
    echo "-> Failed to create jni libs"
    exit 1
fi
