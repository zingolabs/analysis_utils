#![forbid(unsafe_code)]
use macros::annotated_benchmark;
use std::time::Instant;
use zingo_testutils::scenarios;

#[annotated_benchmark(unsynced_faucet_recipient_1153)]
async fn basic_annotated_benchmark_test() {
    println!("HELLO");
}
