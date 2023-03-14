use fuels::{
    prelude::{abigen, TxParameters},
    test_helpers::{launch_custom_provider_and_get_wallets, AssetConfig, Config, WalletsConfig},
    types::AssetId,
};

abigen!(Predicate(
    name = "Predicate",
    abi = "out/debug/predicates-test-abi.json"
));
#[tokio::test]
async fn just_test() {
    let provider_config = Config {
        utxo_validation: true,
        ..Config::local_node()
    };

    let wallets_config = WalletsConfig::new_multiple_assets(
        2,
        vec![AssetConfig {
            id: AssetId::default(),
            num_coins: 1,
            coin_amount: 1_000,
        }],
    );

    let wallets =
        &launch_custom_provider_and_get_wallets(wallets_config, Some(provider_config), None).await;

    let first_wallet = &wallets[0];
    let second_wallet = &wallets[1];

    let predicate = Predicate::load_from("out/debug/predicates-test.bin").unwrap();

    let predicate_code = predicate.code();
    let predicate_address = predicate.address();

    // First wallet transfers amount to predicate.
    let _result = first_wallet
        .transfer(
            predicate_address,
            500,
            AssetId::default(),
            TxParameters::default(),
        )
        .await
        .unwrap();

    // Check predicate balance.
    let balance = first_wallet
        .get_provider()
        .unwrap()
        .get_asset_balance(predicate_address, AssetId::default())
        .await
        .unwrap();

    assert_eq!(balance, 500);

    // We use the Predicate's `encode_data()` to encode the data we want to
    // send to the predicate.

    let predicate_data: Vec<u8> = predicate.encode_data(42_u32, 42_u64);

    let amount_to_unlock = 500;

    let _result = second_wallet
        .spend_predicate(
            predicate_address,
            predicate_code,
            amount_to_unlock,
            AssetId::default(),
            second_wallet.address(),
            Some(predicate_data),
            TxParameters::default(),
        )
        .await
        .unwrap();

    // Predicate balance is zero.
    let balance = first_wallet
        .get_provider()
        .unwrap()
        .get_asset_balance(predicate_address, AssetId::default())
        .await
        .unwrap();

    assert_eq!(balance, 0);

    // Second wallet balance is updated.
    let balance = second_wallet
        .get_asset_balance(&AssetId::default())
        .await
        .unwrap();
    assert_eq!(balance, 1500);
}
