use zingo_testutils::build_fvk_client_and_capability;
use zingo_testutils::build_fvks_from_wallet_capability;
use zingo_testutils::data::seeds::HOSPITAL_MUSEUM_SEED;
use zingo_testutils::regtest::ChildProcessHandler;
use zingo_testutils::regtest::RegtestManager;
use zingo_testutils::scenarios::setup::ScenarioBuilder;
use zingolib::lightclient::LightClient;
pub async fn unsynced_viewonlyclient_1153() -> (
    RegtestManager,
    ChildProcessHandler,
    LightClient,
    LightClient,
) {
    let mut sb = ScenarioBuilder::new_load_1153_saplingcb_regtest_chain();
    let zingo_config = zingolib::load_clientconfig(
        sb.client_builder.server_id.clone(),
        Some(sb.client_builder.zingo_datadir.clone()),
        zingoconfig::ChainType::Regtest,
        true,
    )
    .unwrap();
    // Create a lightclient to extract a capability from.
    let original_recipient = sb.client_builder.build_new_faucet(0, false).await;
    // Extract viewing keys
    let wallet_capability = original_recipient
        .wallet
        .wallet_capability()
        .read()
        .await
        .clone();
    // Delete the client after getting the capability.
    // Extract the orchard fvk
    let [o_fvk, s_fvk, t_fvk] = build_fvks_from_wallet_capability(&wallet_capability);
    let (viewing_client, _) =
        build_fvk_client_and_capability(&[&o_fvk, &s_fvk, &t_fvk], &zingo_config).await;
    (
        sb.regtest_manager,
        sb.child_process_handler.unwrap(),
        original_recipient,
        viewing_client,
    )
}
pub async fn unsynced_faucet_recipient_1153() -> (
    RegtestManager,
    ChildProcessHandler,
    LightClient,
    LightClient,
) {
    let mut sb = ScenarioBuilder::new_load_1153_saplingcb_regtest_chain();
    //(Some(REGSAP_ADDR_FROM_ABANDONART.to_string()), None);
    let faucet = sb.client_builder.build_new_faucet(0, false).await;
    let recipient = sb
        .client_builder
        .build_newseed_client(HOSPITAL_MUSEUM_SEED.to_string(), 0, false)
        .await;
    (
        sb.regtest_manager,
        sb.child_process_handler.unwrap(),
        faucet,
        recipient,
    )
}
