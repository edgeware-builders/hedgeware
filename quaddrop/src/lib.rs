use hex::FromHex;
use serde_json::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{Serialize, Deserialize};
use hedgeware_parachain_primitives::*;

pub mod distribution;

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

pub fn parse_allocation(local_path: String) -> Result<Allocation> {
	let balances = get_quaddrop_allocation(local_path).unwrap().balances.iter()
		.map(|b| {
			let balance = b.1.to_string().parse::<Balance>().unwrap();
			return (<[u8; 32]>::from_hex(b.0.clone()).unwrap().into(), balance);
		})
		.filter(|b| b.1 > 0)
		.chain(distribution::get_dev_allocation().clone())
		.chain(distribution::get_edgeware_treasury_allocation().clone())
		.chain(distribution::get_crowdloan_allocation().clone())
		.collect();

	Ok(Allocation {
		balances,
	})
}

#[test]
fn sum_allocation() {
	let allocation = parse_allocation("allocation/dump.json".to_string());
	let total = allocation.unwrap().balances
		.iter()
		.map(|elt| elt.1)
		.fold(0, |acc, elt| acc + elt);
	// ensure total is less than 5 million tokens with 18 decimals
	assert!(total < 5_000_000_000_000_000_000_000_000);
}