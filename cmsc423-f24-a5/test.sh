#!/bin/bash
cargo build --bin saligner
target/debug/saligner ../fitting_test.sp 2 5 output_test.txt
