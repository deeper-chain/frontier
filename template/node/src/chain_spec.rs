use frontier_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, EVMConfig, EthereumConfig, GenesisConfig, GrandpaConfig,
	Signature, SudoConfig, SystemConfig, WASM_BINARY,
};
use hex_literal::hex;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public, H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					authority_keys_from_seed("Alice"),
					authority_keys_from_seed("Bob"),
				],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities
				.iter()
				.map(|x| (x.1.clone(), 1))
				.collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		evm: EVMConfig {
			
            account_pairs: {
                let mut map = BTreeMap::new();
                // Alice's deeper chain address: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
                // H160 address of Alice mapping eth account
                // eth address: 0x7a5b2024e179b312B924Ff02F4c27b5DF5326601
                // eth privete key: 0xb52e6d24f6caacc1961d3cedf04ed3a11a7f4a27a6ce85eeea5dbea6c694f53a

                map.insert(
                    H160::from_str("7a5b2024e179b312B924Ff02F4c27b5DF5326601")
                        .expect("internal H160 is valid; qed"),
                    hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").into(),
                );
                // Bob's deeper chain address: 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
                // H160 address of Bob mapping eth account
                // eth address: 0x120CF1Df8D02f6b1Aa4F2Dc9BF8FD7Cec63d8581
                // eth privete key: 0xd2d7f189142e7a0468f97e5f16ef7762bf199a4c6c31b3e38fbf43f38f7d8f30

                map.insert(
                    H160::from_str("120CF1Df8D02f6b1Aa4F2Dc9BF8FD7Cec63d8581")
                        .expect("internal H160 is valid; qed"),
                    hex!("8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48").into(),
                );
                // Dave's deeper chain address: 0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20
                // H160 address of Dave mapping eth account
                // eth address: 0x843AFB0DC3aD56696800C0d61C76Ac2A147AD48C
                // eth privete key: 0xc32a1b133e8164ce6f63441090c29bd41e8ad0af1bf307c49ff3d40b1916db03
                map.insert(
                    H160::from_str("843AFB0DC3aD56696800C0d61C76Ac2A147AD48C")
                        .expect("internal H160 is valid; qed"),
                    hex!("306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20").into(),
                );
                // Eve deeper chain address: 0xe659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e
                // H160 address of Eve mapping eth account
                // eth address: 0xe1bA6c4568D7ae1b87B9fF59eeB1d1Ff3c0C4f5B
                // eth privete key: 0x828f8cdc56b6a78c5e9698900a00d7212780892da3486062d70092e3e9f6a37e
                map.insert(
                    H160::from_str("e1bA6c4568D7ae1b87B9fF59eeB1d1Ff3c0C4f5B")
                        .expect("internal H160 is valid; qed"),
                    hex!("e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e").into(),
                );
                map
            },
            accounts: {
                let mut map = BTreeMap::new();
                map.insert(
                    // H160 address of Alice eth account
                    H160::from_str("7a5b2024e179b312B924Ff02F4c27b5DF5326601")
                        .expect("internal H160 is valid; qed"),
                    pallet_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map.insert(
                    // H160 address of Bob eth account
                    H160::from_str("120CF1Df8D02f6b1Aa4F2Dc9BF8FD7Cec63d8581")
                        .expect("internal H160 is valid; qed"),
                    pallet_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );

                map.insert(
                    // H160 address of dave eth account
                    H160::from_str("843AFB0DC3aD56696800C0d61C76Ac2A147AD48C")
                        .expect("internal H160 is valid; qed"),
                    pallet_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map.insert(
                    // H160 address of eve eth account
                    H160::from_str("e1bA6c4568D7ae1b87B9fF59eeB1d1Ff3c0C4f5B")
                        .expect("internal H160 is valid; qed"),
                    pallet_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map
            },
		},
		ethereum: EthereumConfig {},
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
	}
}
