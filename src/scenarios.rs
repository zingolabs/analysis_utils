use zingo_testutils::data::seeds::HOSPITAL_MUSEUM_SEED;
use zingo_testutils::regtest::ChildProcessHandler;
use zingo_testutils::regtest::RegtestManager;
use zingo_testutils::scenarios::setup::ScenarioBuilder;
use zingo_testutils::{build_fvk_client, build_fvks_from_wallet_capability};
use zingoconfig::RegtestNetwork;
use zingolib::lightclient::LightClient;

pub async fn unsynced_viewonlyclient_1153() -> (
    RegtestManager,
    ChildProcessHandler,
    LightClient,
    LightClient,
) {
    let regtest_network = RegtestNetwork::all_upgrades_active();
    let mut sb = ScenarioBuilder::new_load_1153_saplingcb_regtest_chain(&regtest_network).await;
    let zingo_config = zingolib::load_clientconfig(
        sb.client_builder.server_id.clone(),
        Some(sb.client_builder.zingo_datadir.clone()),
        zingoconfig::ChainType::Regtest(regtest_network),
        true,
    )
    .unwrap();
    // Create a lightclient to extract a capability from.
    let original_recipient = sb.client_builder.build_faucet(false, regtest_network).await;
    // Extract viewing keys
    let wallet_capability = original_recipient.wallet.wallet_capability().clone();
    // Delete the client after getting the capability.
    // Extract the orchard fvk
    let [o_fvk, s_fvk, t_fvk] = build_fvks_from_wallet_capability(&wallet_capability);
    let viewing_client = build_fvk_client(&[&o_fvk, &s_fvk, &t_fvk], &zingo_config).await;
    (
        sb.regtest_manager,
        sb.child_process_handler.unwrap(),
        viewing_client,
        original_recipient,
    )
}
pub async fn unsynced_faucet_recipient_1153() -> (
    RegtestManager,
    ChildProcessHandler,
    LightClient,
    LightClient,
) {
    let regtest_network = RegtestNetwork::all_upgrades_active();
    let mut sb = ScenarioBuilder::new_load_1153_saplingcb_regtest_chain(&regtest_network).await;
    //(Some(REGSAP_ADDR_FROM_ABANDONART.to_string()), None);
    let faucet = sb.client_builder.build_faucet(false, regtest_network).await;
    let recipient = sb
        .client_builder
        .build_client(HOSPITAL_MUSEUM_SEED.to_string(), 0, false, regtest_network)
        .await;
    (
        sb.regtest_manager,
        sb.child_process_handler.unwrap(),
        faucet,
        recipient,
    )
}
