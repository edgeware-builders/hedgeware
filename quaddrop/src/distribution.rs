use hex_literal::*;
use hedgeware_parachain_primitives::AccountId;
use hedgeware_parachain_primitives::Balance;

/// Split endowment amount for Commonwealth
pub const ENDOWMENT: Balance = 50_000_000_000_000_000_000_000;
/// Genesis allocation that will fit into the "balances" module for Commonwealth/Founders
pub fn get_allocation() -> Vec<(AccountId, Balance)> {
	return vec![(
		hex!["dadea872c11cac6d115aa4fe27eb1592383f99c9a0ae5ccbcbc44ffe31530871"].into(),
		ENDOWMENT,
	)];
}