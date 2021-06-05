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

pub fn get_quaddrop_allocation() -> Result<AllocationRaw> {
	let path = Path::new("quaddrop/allocation/dump.json");
	let mut file = File::open(&path).unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	let a: AllocationRaw = serde_json::from_str(&data)?;
	return Ok(a);
}

pub fn parse_allocation() -> Result<Allocation> {
	let balances = distribution::get_allocation();
	Ok(Allocation {
		balances,
	})
}
