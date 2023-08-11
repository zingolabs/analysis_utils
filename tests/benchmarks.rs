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
#[annotated_benchmark(unsynced_faucet_recipient_1153)]
async fn keyowning_client_pu_true() {
    keyowning.do_sync(true).await.unwrap();
}
#[annotated_benchmark(unsynced_faucet_recipient_1153)]
async fn keyowning_client_pu_false() {
    keyowning.do_sync(false).await.unwrap();
}
#[annotated_benchmark(unsynced_viewonlyclient_1153)]
async fn fullviewonly_client_pu_true() {
    keyowning.do_sync(true).await.unwrap();
}
#[annotated_benchmark(unsynced_viewonlyclient_1153)]
async fn fullviewonly_client_pu_false() {
    keyowning.do_sync(false).await.unwrap();
}
