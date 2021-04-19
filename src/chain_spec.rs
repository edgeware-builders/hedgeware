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

use parachain_runtime::DOLLARS;
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use rococo_parachain_primitives::{AccountId, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use lockdrop::*;
use rococo_parachain_primitives::*;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<parachain_runtime::GenesisConfig, Extensions>;

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

/// Mainnet configuration
pub fn hedgeware_rococo_testnet() -> ChainSpec {
	match ChainSpec::from_json_bytes(&include_bytes!("../res/hedgeware.chainspec.json")[..]) {
		Ok(spec) => spec,
		Err(e) => panic!("{}", e),
	}
}

type AccountPublic = <Signature as Verify>::Signer;

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
			"ss58Format": 42,
			"tokenDecimals": 18,
			"tokenSymbol": "tHEDG"
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
				id,
			)
		},
		vec![],
		None,
		None,
		properties,
		Extensions {
			relay_chain: "rococo_local_testnet".into(),
			para_id: id.into(),
		},
	)
}

pub fn hedgeware(id: ParaId) -> ChainSpec {
	let data = r#"
		{
			"ss58Format": 77,
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
				vec![],
				id,
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

fn testnet_genesis(
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> parachain_runtime::GenesisConfig {
	let allocation: Allocation = parse_allocation().unwrap();
	let balances: Vec<(AccountId, Balance)> = allocation.balances;

	let vesting: Vec<(AccountId, BlockNumber, BlockNumber, Balance)> = allocation.vesting;

	const INITIAL_BALANCE: u128 = 1_000_000 * DOLLARS;

	parachain_runtime::GenesisConfig {
		frame_system: parachain_runtime::SystemConfig {
			code: parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: parachain_runtime::BalancesConfig {
			balances: balances.into_iter()
				.chain::<Vec<(AccountId, Balance)>>(
					endowed_accounts
						.iter()
						.map(|a| (a.clone(), INITIAL_BALANCE))
						.collect()
					)
				.map(|a| a)
				.collect::<Vec<(AccountId, Balance)>>(),
		},
		pallet_vesting: parachain_runtime::VestingConfig { vesting: vesting },
		pallet_sudo: parachain_runtime::SudoConfig { key: root_key },
		parachain_info: parachain_runtime::ParachainInfoConfig { parachain_id: id },
		pallet_contracts: Default::default(),
		pallet_democracy: Default::default(),
		pallet_collective_Instance1: parachain_runtime::CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		},
		pallet_treasury: Default::default(),
		edge_treasury_reward: Default::default(),
		pallet_elections_phragmen: Default::default(),
	}
}
