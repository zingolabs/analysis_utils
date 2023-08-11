#![forbid(unsafe_code)]

use macros::annotated_benchmark;

use tokio::time::Instant;
use zingo_testutils::scenarios;
#[annotated_benchmark(unsynced_faucet_recipient_1153)]
async fn keyless_client_pu_false() {
    keyless.do_sync(false).await.unwrap();
}
#[annotated_benchmark(unsynced_faucet_recipient_1153)]
async fn keyless_client_pu_true() {
    keyless.do_sync(true).await.unwrap();
}
/*
#[annotated_benchmark("fun", "ham", 1)]
#[tokio::test]
async fn keyless_client_pu_false() {
    timing_run("keyless", false).await;
}
#[tokio::test]
async fn keyowning_client_pu_true() {
    timing_run("keyowning", true).await;
}
#[tokio::test]
async fn keyowning_client_pu_false() {
    timing_run("keyowning", false).await;
}
#[tokio::test]
async fn fullviewonly_client_pu_true() {
    timing_run("fullviewonly", true).await;
}
#[tokio::test]
async fn fullviewonly_client_pu_false() {
    timing_run("fullviewonly", false).await;
}
*/
