use primitives::{Pair, Public};
use parachain_runtime::{
	AccountId, BalancesConfig, GenesisConfig, SudoConfig, IndicesConfig, SystemConfig, WASM_BINARY,
};
use substrate_service;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId) {
	(get_from_seed::<AccountId>(&format!("{}//stash", seed)), get_from_seed::<AccountId>(seed))
}

/// Returns the chain spec.
pub fn get_chain_spec() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		|| testnet_genesis(
			vec![
				get_authority_keys_from_seed("Alice"),
				get_authority_keys_from_seed("Bob"),
			],
			get_from_seed::<AccountId>("Alice"),
			vec![
				get_from_seed::<AccountId>("Alice"),
				get_from_seed::<AccountId>("Bob"),
				get_from_seed::<AccountId>("Charlie"),
				get_from_seed::<AccountId>("Dave"),
				get_from_seed::<AccountId>("Eve"),
				get_from_seed::<AccountId>("Ferdie"),
				get_from_seed::<AccountId>("Alice//stash"),
				get_from_seed::<AccountId>("Bob//stash"),
				get_from_seed::<AccountId>("Charlie//stash"),
				get_from_seed::<AccountId>("Dave//stash"),
				get_from_seed::<AccountId>("Eve//stash"),
				get_from_seed::<AccountId>("Ferdie//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None,
	)
}

fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
			vesting: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
	}
}
