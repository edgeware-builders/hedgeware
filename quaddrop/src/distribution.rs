use hex_literal::*;
use hedgeware_parachain_primitives::AccountId;
use hedgeware_parachain_primitives::Balance;

/// Split endowment amount for founders
pub const ENDOWMENT: Balance = 500_000_000_000_000_000_000_000;

/// Genesis allocation that will fit into the "balances" module for founders/core devs
pub fn get_dev_allocation() -> Vec<(AccountId, Balance)> {
	return vec![(
		hex!["dadea872c11cac6d115aa4fe27eb1592383f99c9a0ae5ccbcbc44ffe31530871"].into(),
		ENDOWMENT,
	), (
		hex!["647e5d643dc2ca54a4eb5e2803245f2086fd2e9dc378a559f20e3d1fb6ba707e"].into(),
		ENDOWMENT,
	), (
		hex!["e0a06290c2eee45209b18d498f361c0e7ee67586314677b509193e6aeae47c32"].into(),
		ENDOWMENT,
	), (
		hex!["24b36676d5758405d13c946b6a439e99babee964400c782dea8e1ed6393c3e1a"].into(),
		ENDOWMENT,
	), (
		hex!["90b682e600e15c90def14b0591bd63fb8d4db91f055602746bc2fccda0a7175b"].into(),
		ENDOWMENT,
	), (
		hex!["667a0a10f6c2cf1428891afda1471b993fa8e3f6ceb865a91095c674038f4140"].into(),
		ENDOWMENT,
	), (
		hex!["9edf22e19102ca8fea312c12f8e22613aaba69561c9933a93eff6ca70c65a82d"].into(),
		ENDOWMENT,
	), (
		hex!["fab1361c01606a0d54eb3fc3e99dcc9b44bc931e386d2894d3f1c846886d4769"].into(),
		ENDOWMENT,
	), (
		hex!["bc70d598387cd74611438e5c82dbe67cb0a89eedd13896a5daf80ade40739f08"].into(),
		ENDOWMENT,
	), (
		hex!["260dd712b57877f27344499f8d3f89f67efb09eab19c521e023e9c213b59ba66"].into(),
		ENDOWMENT,
	), (
		hex!["385aa775302c1b6bf59df6dbe8e4220422b6d4caf2a800d9ff9d123aa092c710"].into(),
		ENDOWMENT,
	), (
		hex!["5efcd22c50a064ffc8fa9925c9646ef060401b95194ac369bba09d9d5b97392b"].into(),
		ENDOWMENT,
	), (
		hex!["8c36ec6ee71ee248ea40b78a419770a743c6504efd5a9fee99ea6ae6da83741f"].into(),
		ENDOWMENT,
	), (
		hex!["2ed366db3edf8d5f952e978f2cd2df084d48764fdd534fca8a1b2045d99c0765"].into(),
		ENDOWMENT,
	), (
		hex!["5ad1494edc7f7168e14c70214fda868289f266f8d9d3603092b70f71021bd620"].into(),
		ENDOWMENT,
	), (
		hex!["6a327e9426cdd6a239700d3be2d296968e477c59e82268d43eba18947d7f8a56"].into(),
		ENDOWMENT,
	), (
		hex!["381888fd990ab930764b1dde63b72a492d4e48bf5ce55656cb2d8c1f8ba27750"].into(),
		ENDOWMENT,
	), (
		hex!["f8df6f0123658748bb537b97625ca3e2750756f104fa8de25f259eb4d0954677"].into(),
		ENDOWMENT,
	), (
		hex!["1e7717a66a32229eaa0d94eb39c75ba5f1620598c74cac45a545215b5da22640"].into(),
		ENDOWMENT,
	), (
		hex!["e2ff54b28b14fb38b61732cd0e7fac6cccbd533ea5452f4b1eadb341f8f82f24"].into(),
		ENDOWMENT,
	)];
}

pub fn get_edgeware_treasury_allocation() -> Vec<(AccountId, Balance)> {
	return vec![(
		hex!["885c2c7916cfa19940de1daeb126bd806d3bba65b5d6602d66916061eeda1800"].into(),
		ENDOWMENT * 10,
	)];
}

pub fn get_crowdloan_allocation() -> Vec<(AccountId, Balance)> {
	return vec![(
		hex!["6e3f21eaaced8dd1f871de62b8368db4916d18b9ba67a8250037d91ae95c8972"].into(),
		ENDOWMENT * 20,
	)];
}