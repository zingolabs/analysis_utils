#![forbid(unsafe_code)]
#![cfg(feature = "local_env")]
pub mod darkside;
use std::{fs::File, path::Path};
use zingo_testutils::{self, build_fvk_client_and_capability, data};

use bip0039::Mnemonic;
use data::seeds::HOSPITAL_MUSEUM_SEED;
use json::JsonValue::{self, Null};
use tokio::time::Instant;
use zingo_testutils::scenarios;

use tracing_test::traced_test;
use zcash_client_backend::encoding::encode_payment_address;
use zcash_primitives::{
    consensus::Parameters,
    merkle_tree::write_incremental_witness,
    transaction::{fees::zip317::MINIMUM_FEE, TxId},
};
use zingo_testutils::regtest::get_cargo_manifest_dir_parent;
use zingoconfig::{ChainType, ZingoConfig};
use zingolib::{
    check_client_balances, get_base_address,
    lightclient::LightClient,
    wallet::{
        data::TransactionMetadata,
        keys::{
            extended_transparent::ExtendedPrivKey,
            unified::{Capability, WalletCapability},
        },
        LightWallet, Pool,
    },
};

#[test]
fn inside_benchmarks() {}
mod benchmarks {
    use super::*;
    #[tokio::test]
    pub async fn time_scenario_setup_teardown() {
        let cph = zingo_testutils::scenarios::chainload::unsynced_basic().await;
        drop(cph);
    }
    mod sync_1153_baseline_synctimes {
        const PREFIX: &'static str = "sync_1153_baseline_synctimes";

        use zingo_testutils::DurationAnnotation;

        use super::*;
        async fn timing_run(keyownership: &str, print_updates: bool) {
            let sync_duration;
            match keyownership {
                "keyowning" => {
                    let (_, child_process_handler, keyowning, _keyless) =
                        scenarios::chainload::unsynced_faucet_recipient_1153().await;
                    let timer_start = Instant::now();
                    keyowning.do_sync(print_updates).await.unwrap();
                    let timer_stop = Instant::now();
                    sync_duration = timer_stop.duration_since(timer_start);
                    drop(child_process_handler);
                }
                "keyless" => {
                    let (_, child_process_handler, _keyowning, keyless) =
                        scenarios::chainload::unsynced_faucet_recipient_1153().await;
                    let timer_start = Instant::now();
                    keyless.do_sync(print_updates).await.unwrap();
                    let timer_stop = Instant::now();
                    sync_duration = timer_stop.duration_since(timer_start);
                    drop(child_process_handler);
                }
                "fullviewonly" => {
                    let (_, child_process_handler, view_only_client) =
                        scenarios::chainload::unsynced_viewonlyclient_1153().await;
                    let timer_start = Instant::now();
                    view_only_client.do_sync(print_updates).await.unwrap();
                    let timer_stop = Instant::now();
                    sync_duration = timer_stop.duration_since(timer_start);
                    drop(child_process_handler);
                }
                _ => panic!(),
            }
            let annotation = DurationAnnotation::new(
                format!("{PREFIX}_{keyownership}_client_pu_{print_updates}"),
                sync_duration,
            );
            zingo_testutils::record_time(&annotation);

            assert!(sync_duration.as_secs() < 1000);
        }
        #[tokio::test]
        async fn keyless_client_pu_true() {
            timing_run("keyless", true).await;
        }
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
    }
}