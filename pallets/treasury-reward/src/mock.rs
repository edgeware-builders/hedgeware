use super::*;
use crate as treasury_reward;
use sp_runtime::traits::One;
use frame_support::pallet_prelude::DispatchResult;
use frame_support::{construct_runtime, parameter_types, weights::Weight, PalletId};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill, AccountId32,
};
use frame_support::traits::GenesisBuild;
use system::mocking::{MockBlock, MockUncheckedExtrinsic};
pub(crate) type Balance = u64;

// Configure a mock runtime to test the pallet.
type UncheckedExtrinsic = MockUncheckedExtrinsic<Test>;
type Block = MockBlock<Test>;
pub const DOLLARS: Balance = 1_000_000_000;
pub type AccountId = AccountId32;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		TreasuryReward: treasury_reward::{Pallet, Call, Storage, Event<T>, Config<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Config for Test {
	type AccountData = pallet_balances::AccountData<u64>;
	type AccountId = AccountId32;
	type BaseCallFilter = ();
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type Call = Call;
	type DbWeight = ();
	type Event = Event;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = Prefix;
	type SystemWeightInfo = ();
	type Version = ();
	type OnSetCode = ();
}

parameter_types! {
	pub const Prefix: u8 = 100;
	pub const ExistentialDeposit: Balance = 0;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type WeightInfo = ();
}

parameter_types! {
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
}

parameter_types! {
	pub const MinimumTreasuryPct: Percent = Percent::from_percent(50);
	pub const MaximumRecipientPct: Percent = Percent::from_percent(50);
}

impl Config for Test {
	type Event = Event;
	type Currency = Balances;
	type MinimumTreasuryPct = MinimumTreasuryPct;
	type MaximumRecipientPct = MaximumRecipientPct;
	type DefaultRewardAddress = TreasuryPalletId;
}

pub(crate) fn new_test_ext(recipients: Option<Vec<AccountId>>, pcts: Option<Vec<Percent>>) -> sp_io::TestExternalities {
	let recipients = recipients.unwrap_or_else(|| vec![
		AccountId::new([1; 32]),
		AccountId::new([2; 32]),
		AccountId::new([3; 32])
	]);
	let pcts = pcts.unwrap_or_else(|| vec![
		Percent::from_percent(10),
		Percent::from_percent(10),
		Percent::from_percent(10),
	]);

	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
				(AccountId32::new([1; 32]), 10000000000),
				(AccountId32::new([2; 32]), 10000000000),
				(AccountId32::new([3; 32]), 10000000000),
				(AccountId32::new([4; 32]), 10000000000),
				(AccountId32::new([10; 32]), 10000000000),
				(AccountId32::new([11; 32]), 10000000000),
				(AccountId32::new([20; 32]), 10000000000),
				(AccountId32::new([21; 32]), 10000000000),
				(AccountId32::new([30; 32]), 10000000000),
				(AccountId32::new([31; 32]), 10000000000),
				(AccountId32::new([40; 32]), 10000000000),
				(AccountId32::new([41; 32]), 10000000000),
				(AccountId32::new([100; 32]), 10000000000),
				(AccountId32::new([101; 32]), 10000000000),
				// This allow us to have a total_payout different from 0.
				(AccountId32::new([255; 32]), 1_000_000_000_000),
		],
	}.assimilate_storage(&mut t).unwrap();

	
	treasury_reward::GenesisConfig::<Test> {
		current_payout: 95 * DOLLARS,
		minting_interval: One::one(),
		recipients: recipients,
		recipient_percentages: pcts,
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}

pub fn add_recipient(recipient: AccountId, percent: Percent) -> DispatchResult {
	TreasuryReward::add(Origin::root(), recipient, percent)
}

pub fn remove_recipient(recipient: AccountId) -> DispatchResult {
	TreasuryReward::remove(Origin::root(), recipient)
}

pub fn update(recipient: AccountId, percent: Percent) -> DispatchResult {
	TreasuryReward::update(Origin::root(), recipient, percent)
}
