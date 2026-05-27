#!/bin/bash

cargo build --release
hyperfine --warmup 100 -r 100 'cargo run --release -- --file_path "$(head -n1 test_file_path.txt)"'