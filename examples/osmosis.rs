use localic_utils::{ConfigChainBuilder, TestContextBuilder, OSMOSIS_CHAIN_NAME};
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
        .with_chain(ConfigChainBuilder::default_osmosis().build()?)
        .build()?;

    // Create some tokens on osmosis
    ctx.build_tx_create_tokenfactory_token()
        .with_chain_name(OSMOSIS_CHAIN_NAME)
        .with_subdenom("bruhtoken")
        .send()?;
    let bruhtoken =
        ctx.get_tokenfactory_denom("osmo1kuf2kxwuv2p8k3gnpja7mzf05zvep0cysqyf2a", "bruhtoken");

    // Create an osmosis pool
    ctx.build_tx_create_osmo_pool()
        .with_weight("uosmo", 1)
        .with_weight(&bruhtoken, 1)
        .with_initial_deposit("uosmo", 1)
        .with_initial_deposit(&bruhtoken, 1)
        .send()?;

    Ok(())
}
