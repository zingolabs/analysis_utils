# TL;DR Follow these steps to run a benchmark

0. git clone zingolib and checkout the version you want to benchmark

1. Follow the zingo-testutils README at `docs/TEST-REGTEST.md` to set up regtest mode in the version of the code that you want to benchmark.
2. run: cargo nextest run $BENCHMARK

   $BENCHMARKS are marked as #[annotated_benchmark]s in:

  ./tests/benchmarks.rs

  e.g.

  keyowning_client_pu_false


  So you can invoke that benchmark with:

  `cargo nextest run keyowning_client_pu_false`

3.  Step 2\. produces (or appends to a file in tests/times) named *_sync_duration_annotation.json

4. THIS ONLY KINDA WORKS FOR SPECIFIC CASES You can pass a duration_annotation file to `cargo run` to produce a plot of different kinds of annotations.


