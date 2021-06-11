// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.


use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use hedgeware_parachain_runtime::{AuraId};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify, One};
use hedgeware_parachain_primitives::{AccountId, Signature};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<hedgeware_parachain_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Mainnet configuration
pub fn hedgeware_rococo_testnet() -> ChainSpec {
	match ChainSpec::from_json_bytes(&include_bytes!("../res/hedgeware.chainspec.json")[..]) {
		Ok(spec) => spec,
		Err(e) => panic!("{}", e),
	}
}

pub fn hedgeware(id: ParaId) -> ChainSpec {
	let data = r#"
		{
			"ss58Format": 777,
			"tokenDecimals": 18,
			"tokenSymbol": "tHEDG"
		}"#;
	let properties = serde_json::from_str(data).unwrap();
	ChainSpec::from_genesis(
		"Hedgeware",
		"hedgeware",
		ChainType::Live,
		move || {
			testnet_genesis(
				hex!["0000000000000000000000000000000000000000000000000000000000000000"].into(),
				vec![
					get_from_seed::<AuraId>("Alice"),
					get_from_seed::<AuraId>("Bob"),
				],
				id,
				true,
			)
		},
		Vec::new(),
		None,
		None,
		properties,
		Extensions {
			relay_chain: "rococo".into(),
			para_id: id.into(),
		},
	)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn get_chain_spec(id: ParaId) -> ChainSpec {
	let data = r#"
		{
			"ss58Format": 777,
			"tokenDecimals": 18,
			"tokenSymbol": "HEDG"
		}"#;
	let properties = serde_json::from_str(data).unwrap();

	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_from_seed::<AuraId>("Alice"),
					get_from_seed::<AuraId>("Bob"),
				],
				id,
				true,
			)
		},
		vec![],
		None,
		None,
		properties,
		Extensions {
			relay_chain: "rococo-local".into(),
			para_id: id.into(),
		},
	)
}

fn testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<AuraId>,
	id: ParaId,
	dev_accounts: bool,
) -> hedgeware_parachain_runtime::GenesisConfig {
	let balances = quaddrop::parse_allocation(
		"quaddrop/allocation/dump.json".to_string(),
		dev_accounts
	).unwrap().balances;

	hedgeware_parachain_runtime::GenesisConfig {
		frame_system: hedgeware_parachain_runtime::SystemConfig {
			code: hedgeware_parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_aura: hedgeware_parachain_runtime::AuraConfig {
			authorities: initial_authorities,
		},
		pallet_balances: hedgeware_parachain_runtime::BalancesConfig {
			balances: balances.clone(),
		},
		pallet_democracy: hedgeware_parachain_runtime::DemocracyConfig::default(),
		pallet_collective_Instance1: hedgeware_parachain_runtime::CouncilConfig::default(),
		pallet_treasury: Default::default(),
		pallet_elections_phragmen: Default::default(),
		// pallet_vesting: hedgeware_parachain_runtime::VestingConfig::default(),
		treasury_reward: hedgeware_parachain_runtime::TreasuryRewardConfig{
			current_payout: Default::default(),
				minting_interval: One::one(),
			recipients: Default::default(),
			recipient_percentages: Default::default(),
		},
		pallet_evm: Default::default(),
		pallet_ethereum: Default::default(),
		pallet_sudo: hedgeware_parachain_runtime::SudoConfig { key: root_key },
		parachain_info: hedgeware_parachain_runtime::ParachainInfoConfig { parachain_id: id },
		cumulus_pallet_aura_ext: Default::default(),
    cumulus_pallet_parachain_system: Default::default(),
	}
}
