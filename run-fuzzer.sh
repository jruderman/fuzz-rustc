#!/bin/bash

set -e
set -x

if [ ! -d rust ]; then
    #git clone https://github.com/dwrensha/rust.git --branch fuzz

    # Building the old version above @ 4d06717396b28aff7a36ad8521e80868e8569f76 with 2022-10-13 nightly didn't work for me
    # Since I don't know how to use git,
    # Just apply the changes (plus one more) directly to the current version
    git clone https://github.com/rust-lang/rust.git
    (cd rust && patch -p1 < ../rust-changes.diff)
fi

rustup override set nightly

# Include some debugging info in case we need to use rust-lldb.
# Sadly, {debugunfo=2 + opt>0 + cov} causes llvm to crash while compiling our target
export RUSTFLAGS="$RUSTFLAGS -C debuginfo=1"

# - enable coverage instrumentation
export RUSTFLAGS="$RUSTFLAGS -C passes=sancov-module -C llvm-args=-sanitizer-coverage-level=4"
export RUSTFLAGS="$RUSTFLAGS -C llvm-args=-sanitizer-coverage-trace-compares"
export RUSTFLAGS="$RUSTFLAGS -C llvm-args=-sanitizer-coverage-inline-8bit-counters"
export RUSTFLAGS="$RUSTFLAGS -C llvm-args=-sanitizer-coverage-pc-table"

# - enable compilation of rustc_private crates
export RUSTFLAGS="$RUSTFLAGS -Z force-unstable-if-unmarked"

# - enable debug assertions
export RUSTFLAGS="$RUSTFLAGS -C debug-assertions=on"

#export RUSTFLAGS="$RUSTFLAGS -Z sanitizer=address"

# Create seed directory if it does not exist. Add example files here.
mkdir -p seeds

# Create corpus directory which the fuzzer will fill with interesting inputs.
mkdir -p corpus

# Create artifact output directory.
mkdir -p artifacts

# Create directory for inputs suggested by the compiler.
# These can be added in bulk to corpus/seeds in a future run or merge.
mkdir -p shelved

# Detect the target.
if [ "$(uname -s)" == "Darwin" ]; then
    export TARGET="x86_64-apple-darwin"
elif [ "$(uname -s)" == "Linux" ]; then
    export TARGET="x86_64-unknown-linux-gnu"
else
    echo "Sorry, currently only Mac OS and Linux are supported"
    exit 1
fi

# This variable tells the new rustc (i.e. the one we're compiling) what version it
# should consider itself to have. This is used when loading precompiled libstd and other
# crates. If the rustc version recorded in those crates' metadata does not match this,
# then the compiler quits early.
export CFG_VERSION=`rustc --version | cut -f2- -d ' '`

# Usually we can use the precompiled libstd from rustup.
TOOLCHAIN_ROOT=${RUSTUP_BASE:-$HOME/.rustup}/toolchains/nightly-$TARGET

# If a metadata change has landed on master and is not yet in a nightly release,
# we may need to compile our own libstd. `./x.py build --stage 1` should suffice.
# If fuzzing immediately fails on an empty input, this is a good thing to try.
# (Note that you will also need to change CFG_VERSION, above.)
#TOOLCHAIN_ROOT=$HOME/src/rust/build/x86_64-unknown-linux-gnu/stage1/

# Custom environment variable indicating where to look for the precompiled libstd.
export FUZZ_RUSTC_LIBRARY_DIR=$TOOLCHAIN_ROOT/lib/rustlib/$TARGET/lib

# Set some environment variables that are needed when building the rustc source code.
export CFG_COMPILER_HOST_TRIPLE=$TARGET
export CFG_RELEASE_CHANNEL=nightly
export CFG_RELEASE=unknown
export REAL_LIBRARY_PATH_VAR=foobar

# Any writable location will do for this one.
export RUSTC_ERROR_METADATA_DST=/tmp/rustc_error_metadata

export RUSTC_INSTALL_BINDIR=/tmp/rustc_install_bindir

# Please crash less
export RUST_MIN_STACK=20000000

# The --target flag is important because it prevents build.rs scripts from being built with
# the above-specified RUSTFLAGS.
cargo run --release --verbose --target $TARGET --bin "fuzz_target" -- -artifact_prefix=artifacts/ ${@:1} `pwd`/corpus `pwd`/seeds

# An invocation like this can reduce a corpus:
# Make sure NOT to pass -only_ascii=1 for this
#mkdir new-corpus
#cargo run --release --verbose --target $TARGET --bin "fuzz_target" -- -artifact_prefix=artifacts/ ${@:1} -merge=1 `pwd`/new-corpus `pwd`/corpus `pwd`/seeds

# An invocation like this can minimize a crash:
#cargo run --release --verbose --target $TARGET --bin "fuzz_target" -- -minimize_crash=1 ${@:1}
