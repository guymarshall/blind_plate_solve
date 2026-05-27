#!/bin/bash

cargo run --release -- --file_path "$(head -n1 test_file_path.txt)"