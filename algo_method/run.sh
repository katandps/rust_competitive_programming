#!/bin/bash
cd `dirname $0`

RUST_BACKTRACE=1 cargo run --release < sample.txt