#!/bin/bash
TARGET=arm-unknown-linux-gnueabihf

export TARGET_CC=$TARGET-gcc
export TARGET_AR=$TARGET-ar
export CC_arm_unknown_linux_gnu=$TARGET-gcc
export CXX_arm_unknown_linux_gnu=$TARGET-g++
export AR_arm_unknown_linux_gnu=$TARGET-ar
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=$TARGET-gcc
export CMAKE_TOOLCHAIN_FILE_arm_unknown_linux_gnueabihf=$(pwd)/wip/arm.cmake
cargo build --release --target $TARGET --bin logger
