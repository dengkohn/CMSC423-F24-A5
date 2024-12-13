#!/bin/bash

cargo build --bin saligner
rm output_test.txt
target/debug/saligner ../global_test.sp global 2 5 output_test.txt > array.tsv
code array.tsv
code output_test.txt
