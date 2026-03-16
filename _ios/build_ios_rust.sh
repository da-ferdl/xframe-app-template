#!/bin/sh

# If 'true' builds release variant, else debug variant
IS_RELEASE=true

# => 'LIB_NAME' variable must be set from the caller of this script
if [[ -z "$LIB_NAME" ]]; then
  echo "-> Failed to compile rust - 'LIB_NAME' variable not set or empty"
    exit 1
fi

BASE_PATH="../"

B_TYPE_PATH="debug"
BUILD_TYPE=""

if [ "$IS_RELEASE" == true ]; then
  B_TYPE_PATH="release"
  BUILD_TYPE="--release"
fi

# iOS simulator target for Intel Macs (Intel Processor)
X86_64_TRIPLE="x86_64-apple-ios"

# iOS simulator target for Apple Silicon Macs (M1, M2, ..)
ARM64_SIM_TRIPLE="aarch64-apple-ios-sim"

# iOS device target
DEVICE_TARGET="aarch64-apple-ios"

# Default simulator target defined for intel macs
SIM_TARGET=$X86_64_TRIPLE

if [[ "$(uname -s)" = Darwin && "$(uname -v)" = *ARM64* ]]; then
    # We are on a Silicon Mac - use silicon simulator target
    SIM_TARGET=$ARM64_SIM_TRIPLE
fi


CRATE_TYPE="staticlib"
MANIFEST_PATH=$BASE_PATH"Cargo.toml"

# builds the iOS common target
$HOME/.cargo/bin/cargo rustc \
--lib \
--crate-type $CRATE_TYPE \
--target $DEVICE_TARGET \
$BUILD_TYPE \
--manifest-path=$MANIFEST_PATH

if [ $? -ne 0 ]; then
    echo "-> Failed to compile rust for iOS device target"
    exit 1
fi

# builds the iOS simulator target depending if the current machine
# is intel or silicon
$HOME/.cargo/bin/cargo rustc \
--lib \
--crate-type $CRATE_TYPE \
--target $SIM_TARGET \
$BUILD_TYPE \
--manifest-path=$MANIFEST_PATH

if [ $? -ne 0 ]; then
    echo "-> Failed to compile rust for iOS simulator target"
    exit 1
fi

FRAMEWORK_NAME="${LIB_NAME}.xcframework"

if [ -d "$FRAMEWORK_NAME" ]; then rm -Rf $FRAMEWORK_NAME; fi

xcodebuild \
-create-xcframework \
-library $BASE_PATH"target/"$DEVICE_TARGET"/"$B_TYPE_PATH"/lib"$LIB_NAME".a" \
-library $BASE_PATH"target/"$SIM_TARGET"/"$B_TYPE_PATH"/lib"$LIB_NAME".a" \
-output $FRAMEWORK_NAME

if [ $? -ne 0 ]; then
    echo "-> Failed to create ${FRAMEWORK_NAME}"
    exit 1
fi


