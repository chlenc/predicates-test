use fuels::{
    prelude::abigen,
    test_helpers::{launch_custom_provider_and_get_wallets, AssetConfig, WalletsConfig},
    types::AssetId,
};

abigen!(Predicate(
    name = "Predicate",
    abi = "out/debug/predicates-test-abi.json"
));
#[tokio::test]
async fn just_test() {
    let asset_id = AssetId::default();
    let wallets_config = WalletsConfig::new_multiple_assets(
        2,
        vec![AssetConfig {
            id: asset_id,
            num_coins: 1,
            coin_amount: 1_000,
        }],
    );

    let wallets = &launch_custom_provider_and_get_wallets(wallets_config, None, None).await;

    let first_wallet = &wallets[0];
    let second_wallet = &wallets[1];

    let predicate = Predicate::load_from("out/debug/predicates-test.bin").unwrap();

    // First wallet transfers amount to predicate.
    predicate
        .receive(first_wallet, 500, asset_id, None)
        .await
        .unwrap();

    // Check predicate balance.
    let balance = first_wallet
        .get_provider()
        .unwrap()
        .get_asset_balance(predicate.address(), asset_id)
        .await
        .unwrap();

    assert_eq!(balance, 500);

    // We use the Predicate's `encode_data()` to encode the data we want to
    // send to the predicate. This is a builder pattern and the function
    // returns a new predicate.
    let amount_to_unlock = 500;

    predicate
        .encode_data(4096_u64, 4096_u64)
        .spend(second_wallet, amount_to_unlock, asset_id, None)
        .await
        .unwrap();

    // Predicate balance is zero.
    let balance = first_wallet
        .get_provider()
        .unwrap()
        .get_asset_balance(predicate.address(), AssetId::default())
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
