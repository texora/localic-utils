use super::super::{
    super::{
        error::Error,
        types::contract::{
            AuctionStrategy, ChainHaltConfig, DeployedContractInfo, MinAmount,
            PriceFreshnessStrategy,
        },
        AUCTIONS_MANAGER_CONTRACT_NAME, AUCTION_CONTRACT_NAME, NEUTRON_CHAIN_ID,
    },
    test_context::TestContext,
};
use localic_std::modules::cosmwasm::CosmWasm;

impl TestContext {
    /// Creates an auction manager on Neutron, updating the autions manager
    /// code id and address in the TestContext.
    pub fn tx_create_auctions_manager(
        &mut self,
        sender_key: &str,
        min_auction_amount: impl AsRef<[(String, MinAmount)]>,
        server_addr: impl AsRef<str>,
    ) -> Result<(), Error> {
        let mut contract_a: CosmWasm = self.get_contract(AUCTIONS_MANAGER_CONTRACT_NAME)?;
        let neutron = self.get_chain(NEUTRON_CHAIN_ID);

        let auction_code_id =
            neutron
                .contract_codes
                .get(AUCTION_CONTRACT_NAME)
                .ok_or(Error::Misc(format!(
                    "contract '{AUCTION_CONTRACT_NAME}' is missing"
                )))?;

        let contract = contract_a.instantiate(
            sender_key,
            serde_json::json!({
                "auction_code_id": auction_code_id,
                "min_auction_amount": min_auction_amount.as_ref(),
                "server_addr": server_addr.as_ref(),
            })
            .to_string()
            .as_str(),
            AUCTIONS_MANAGER_CONTRACT_NAME,
            None,
            "",
        )?;

        self.auctions_manager = Some(DeployedContractInfo {
            code_id: contract_a.code_id.ok_or(Error::Misc(format!(
                "contract '{AUCTIONS_MANAGER_CONTRACT_NAME}' has no code ID"
            )))?,
            address: contract.address.clone(),
            artifact_path: contract_a.file_path.ok_or(Error::Misc(format!(
                "contract '{AUCTIONS_MANAGER_CONTRACT_NAME}' has no file path"
            )))?,
        });

        let chain = self.get_mut_chain(NEUTRON_CHAIN_ID);

        chain
            .contract_addrs
            .entry(AUCTIONS_MANAGER_CONTRACT_NAME.to_owned())
            .or_default()
            .push(contract.address);

        Ok(())
    }

    /// Creates an auction on Neutron. Requires that an auction manager has already been deployed.
    pub fn tx_create_auction<'a, TDenomA: AsRef<str>, TDenomB: AsRef<str>>(
        &mut self,
        sender_key: &str,
        pair: (TDenomA, TDenomB),
        auction_strategy: AuctionStrategy,
        chain_halt_config: ChainHaltConfig,
        price_freshness_strategy: PriceFreshnessStrategy,
        label: impl AsRef<str>,
        amount_denom_a: u128,
    ) -> Result<(), Error> {
        // The auctions manager for this deployment
        let contract_a = self.get_auctions_manager()?;
        let denom_a = pair.0.as_ref();

        let receipt = contract_a.execute(
            sender_key,
            serde_json::json!(
            {
                "admin": {
                    "new_auction": {
                        "msg": {
                            "pair": (pair.0.as_ref(), pair.1.as_ref()),
                            "auction_strategy": auction_strategy,
                            "chain_halt_config": chain_halt_config,
                            "price_freshness_strategy": price_freshness_strategy
                        },
                        "label": label.as_ref(),
                    },
            }})
            .to_string()
            .as_str(),
            format!("--amount {amount_denom_a}{denom_a} --gas 2000000").as_str(),
        )?;

        log::debug!(
            "submitted tx creating auction ({}, {}) {:?}",
            pair.0.as_ref(),
            pair.1.as_ref(),
            receipt
        );

        Ok(())
    }
}
