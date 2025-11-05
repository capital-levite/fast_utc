#!/bin/bash

cargo bench --bench timestamp_bench_chrono -- --warm-up-time 0.2 --measurement-time 0.5
cargo bench --bench timestamp_bench_coarsetime -- --warm-up-time 0.2 --measurement-time 0.5
