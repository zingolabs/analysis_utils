# TL;DR Follow these steps to run a benchmark

0. git clone zingolib and checkout the version you want to benchmark
01. point ./Cargo.toml at it.
02. symlink zcashd, zcash-cli, and lightwalletd to analysis_utils/regtest/bin/

1. Follow the zingo-testutils README at `docs/TEST-REGTEST.md` to set up regtest mode in the version of the code that you want to benchmark.
2. run: cargo nextest run $BENCHMARK

   $BENCHMARKS are marked as #[annotated_benchmark]s in:

  ./tests/benchmarks.rs

  e.g.

  keyowning_client_pu_false


  So you can invoke that benchmark with:

  `cargo nextest run keyowning_client_pu_false`

  IMPORTANT NOTE: Benchmarks must be run individually. If they are run in parallel, they will compete for system resources

3.  Step 2\. produces (or appends to a file in tests/times) named {ZINGOLIB_VERSION}_sync_duration_annotation.json

    The current set of supported tests are 

      `keyless_client_pu_false`
      
      `fullviewonly_client_pu_false`
      
      `keyowning_client_pu_false`
      
    After running all three of the above tests, you can pass the output duration_annotation file to `cargo run` to produce a plot of different kinds of annotations.


