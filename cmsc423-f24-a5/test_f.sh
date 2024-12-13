#!/bin/bash

cargo build --bin saligner
rm output_test.txt
rm array.tsv
target/debug/saligner ../fitting_test.sp fitting 2 5 output_test.txt
code output_test.txt
