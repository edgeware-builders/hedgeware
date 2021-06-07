use hex::FromHex;
use serde_json::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{Serialize, Deserialize};
use hedgeware_parachain_primitives::*;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

pub mod distribution;

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Allocation {
	pub balances: Vec<(AccountId, Balance)>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AllocationRaw {
	balances: Vec<(String, String)>,
}

pub fn get_quaddrop_allocation(local_path: String) -> Result<AllocationRaw> {
	let path = Path::new(&local_path);
	let mut file = File::open(&path).unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	let a: AllocationRaw = serde_json::from_str(&data)?;
	return Ok(a);
}

pub fn get_dev_accounts() -> Vec<(AccountId, Balance)> {
	pub const ENDOWMENT: Balance = 1_000_000_000_000_000_000_000;

	return vec![(
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		ENDOWMENT,
	), (
		get_account_id_from_seed::<sr25519::Public>("Bob"),
		ENDOWMENT,
	), (
		get_account_id_from_seed::<sr25519::Public>("Charlie"),
		ENDOWMENT,
	), (
		get_account_id_from_seed::<sr25519::Public>("Dave"),
		ENDOWMENT,
	), (
		get_account_id_from_seed::<sr25519::Public>("Eve"),
		ENDOWMENT,
	), (
		get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		ENDOWMENT,
	), (
		get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
		ENDOWMENT,
	), (
		get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		ENDOWMENT,
	), (
		get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
		ENDOWMENT,
	), (
		get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
		ENDOWMENT,
	), (
		get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
		ENDOWMENT,
	), (
		get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		ENDOWMENT,
	)];
}

pub fn parse_allocation(local_path: String, dev: bool) -> Result<Allocation> {
	let balances = get_quaddrop_allocation(local_path).unwrap().balances.iter()
		.map(|b| {
			let balance = b.1.to_string().parse::<Balance>().unwrap();
			return (<[u8; 32]>::from_hex(b.0.clone()).unwrap().into(), balance);
		})
		.filter(|b| b.1 > 0)
		.chain(distribution::get_dev_allocation().clone())
		.chain(distribution::get_edgeware_treasury_allocation().clone())
		.chain(distribution::get_crowdloan_allocation().clone())
		.chain(if dev { get_dev_accounts() } else { vec![] })
		.collect();

	Ok(Allocation {
		balances,
	})
}

#[test]
fn sum_allocation() {
	let allocation = parse_allocation("allocation/dump.json".to_string(), false);
	let total = allocation.unwrap().balances
		.iter()
		.map(|elt| elt.1)
		.fold(0, |acc, elt| acc + elt);
	// ensure total is less than 5 million tokens with 18 decimals
	assert!(total < 5_000_000_000_000_000_000_000_000);
}