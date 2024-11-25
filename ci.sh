#!/usr/bin/bash

set -euxo pipefail

export RUSTFLAGS=-Dwarnings

cargo +nightly fmt -- --check

cargo clippy
