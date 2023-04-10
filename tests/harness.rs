use fuels::{
    accounts::predicate::Predicate,
    prelude::{abigen, Account, TxParameters, ViewOnlyAccount},
    test_helpers::{launch_custom_provider_and_get_wallets, AssetConfig, WalletsConfig},
    types::AssetId,
};

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

    abigen!(Predicate(
        name = "MyPredicateEncoder",
        abi = "out/debug/predicates-test-abi.json"
    ));

    // Once we've compiled our predicate with `forc build`, we can create a `Predicate` instance via `Predicate::load_from`. The resulting data from `encode_data` can then be set on the loaded predicate.

    // ```rust,ignore
    let predicate_data = MyPredicateEncoder::encode_data(4096, 4096);
    let code_path = "out/debug/predicates-test.bin";

    let predicate: Predicate = Predicate::load_from(code_path)
        .unwrap()
        .with_data(predicate_data)
        .with_provider(first_wallet.try_provider().unwrap().clone());

    // First wallet transfers amount to predicate.
    first_wallet
        .transfer(predicate.address(), 500, asset_id, TxParameters::default())
        .await
        .unwrap();

    // Check predicate balance.
    let balance = predicate
        .get_asset_balance(&AssetId::default())
        .await
        .unwrap();

    assert_eq!(balance, 500);

    let amount_to_unlock = 500;

    predicate
        .transfer(
            second_wallet.address(),
            amount_to_unlock,
            asset_id,
            TxParameters::default(),
        )
        .await
        .unwrap();

    // Predicate balance is zero.
    let balance = predicate
        .get_asset_balance(&AssetId::default())
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
