use cosmwasm_std::Decimal;
use localic_utils::{types::contract::MinAmount, ConfigChainBuilder, TestContextBuilder};
use std::error::Error;

/// Demonstrates using localic-utils for neutron.
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a testcontext
    let mut ctx = TestContextBuilder::default()
        .with_unwrap_raw_logs(true)
        .with_api_url("http://localhost:42069/")
        .with_artifacts_dir("contracts")
        .with_chain(ConfigChainBuilder::default_neutron().build()?)
        .build()?;

    // Upload contracts
    ctx.build_tx_upload_contracts().send()?;

    // Create a token in the tokenfactory
    ctx.build_tx_create_tokenfactory_token()
        .with_subdenom("bruhtoken")
        .send()?;
    ctx.build_tx_create_tokenfactory_token()
        .with_subdenom("amoguscoin")
        .send()?;

    let bruhtoken = ctx.get_tokenfactory_denom(
        "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
        "bruhtoken",
    );
    let amoguscoin = ctx.get_tokenfactory_denom(
        "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
        "amoguscoin",
    );

    // Deploy valence auctions
    ctx.build_tx_create_auctions_manager()
        .with_min_auction_amount(&[(
            &String::from("untrn"),
            MinAmount {
                send: "0".into(),
                start_auction: "0".into(),
            },
        )])
        .with_server_addr("neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg")
        .send()?;
    ctx.build_tx_create_price_oracle().send()?;
    ctx.build_tx_manual_oracle_price_update()
        .with_offer_asset("untrn")
        .with_ask_asset(amoguscoin.as_str())
        .with_price(Decimal::percent(10))
        .send()?;
    ctx.build_tx_update_auction_oracle().send()?;

    ctx.build_tx_mint_tokenfactory_token()
        .with_denom(bruhtoken.as_str())
        .with_amount(10000000000)
        .send()?;
    ctx.build_tx_mint_tokenfactory_token()
        .with_denom(amoguscoin.as_str())
        .with_amount(10000000000)
        .send()?;

    ctx.build_tx_create_auction()
        .with_offer_asset("untrn")
        .with_ask_asset(bruhtoken.as_str())
        .with_amount_offer_asset(10000)
        .send()?;
    ctx.build_tx_create_auction()
        .with_offer_asset("untrn")
        .with_ask_asset(amoguscoin.as_str())
        .with_amount_offer_asset(10000)
        .send()?;

    ctx.get_auction((
        "untrn",
        ctx.get_tokenfactory_denom(
            "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
            "bruhtoken",
        ),
    ))?;
    ctx.get_auction((
        "untrn",
        ctx.get_tokenfactory_denom(
            "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
            "amoguscoin",
        ),
    ))?;

    ctx.build_tx_create_token_registry()
        .with_owner("neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg")
        .send()?;
    ctx.build_tx_create_factory()
        .with_owner("neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg")
        .send()?;
    ctx.build_tx_create_pool()
        .with_denom_a("untrn")
        .with_denom_b(amoguscoin.clone())
        .send()?;
    ctx.build_tx_create_pool()
        .with_denom_a("untrn")
        .with_denom_b(bruhtoken)
        .send()?;

    let pool = ctx.get_astroport_pool(
        "untrn",
        ctx.get_tokenfactory_denom(
            "neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg",
            "amoguscoin",
        ),
    )?;

    assert!(pool
        .query_value(&serde_json::json!({
            "pair": {}
        }))
        .get("data")
        .and_then(|data| data.get("asset_infos"))
        .is_some());

    ctx.build_tx_fund_auction()
        .with_offer_asset("untrn")
        .with_ask_asset(amoguscoin.as_str())
        .with_amount_offer_asset(10000)
        .send()?;

    ctx.build_tx_start_auction()
        .with_offer_asset("untrn")
        .with_ask_asset(amoguscoin.as_str())
        .with_end_block_delta(1000000)
        .send()?;

    ctx.build_tx_fund_pool()
        .with_denom_a("untrn")
        .with_denom_b(amoguscoin)
        .with_amount_denom_a(10000)
        .with_amount_denom_b(10000)
        .with_slippage_tolerance(Decimal::percent(50))
        .with_liq_token_receiver("neutron1kuf2kxwuv2p8k3gnpja7mzf05zvep0cyuy7mxg")
        .send()?;

    Ok(())
}
