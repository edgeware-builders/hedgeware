// Copyright 2018-2020 Commonwealth Labs, Inc.
// This file is part of Edgeware.

// Edgeware is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Edgeware is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use sp_core::{H256};
use frame_system::RawOrigin;
use frame_support::dispatch::DispatchResult;
use frame_support::{assert_ok, assert_err, parameter_types, impl_outer_origin, impl_outer_event};
use frame_support::{traits::{OnFinalize}};

use sp_runtime::{
	Permill, ModuleId,
	testing::{Header}, Percent,
	traits::{BlakeTwo256, IdentityLookup, One},
};

use crate::GenesisConfig;

impl_outer_origin! {
	pub enum Origin for Test where system = frame_system {}
}

mod edge_treasury_reward {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum Event for Test {
		frame_system<T>,
		pallet_balances<T>,
		pallet_treasury<T>,
		edge_treasury_reward<T>,
	}
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = u64;
	type Call = ();
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: u64 = 1;
	pub const SpendPeriod: u64 = 2;
	pub const Burn: Permill = Permill::from_percent(50);
	pub const TipCountdown: u64 = 1;
	pub const TipFindersFee: Percent = Percent::from_percent(20);
	pub const TipReportDepositBase: u64 = 1;
	pub const TipReportDepositPerByte: u64 = 1;
	pub const TreasuryModuleId: ModuleId = ModuleId(*b"py/trsry");
}
impl pallet_treasury::Config for Test {
	type ModuleId = TreasuryModuleId;
	type Currency = Balances;
	type ApproveOrigin = frame_system::EnsureRoot<u64>;
	type RejectOrigin = frame_system::EnsureRoot<u64>;
	type Event = Event;
	type OnSlash = ();
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = ();
	type WeightInfo = ();
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
}

pub type Balances = pallet_balances::Module<Test>;
pub type System = frame_system::Module<Test>;
pub type Treasury = pallet_treasury::Module<Test>;
pub type TreasuryReward = Module<Test>;

pub(crate) fn new_test_ext(recipients: Option<Vec<u64>>, pcts: Option<Vec<Percent>>) -> sp_io::TestExternalities {
	let recipients = recipients.unwrap_or_else(|| vec![1, 2, 3]);
	let pcts = pcts.unwrap_or_else(|| vec![
		Percent::from_percent(10),
		Percent::from_percent(10),
		Percent::from_percent(10),
	]);

	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
				(1, 10000000000),
				(2, 10000000000),
				(3, 10000000000),
				(4, 10000000000),
				(10, 10000000000),
				(11, 10000000000),
				(20, 10000000000),
				(21, 10000000000),
				(30, 10000000000),
				(31, 10000000000),
				(40, 10000000000),
				(41, 10000000000),
				(100, 10000000000),
				(101, 10000000000),
				// This allow us to have a total_payout different from 0.
				(999, 1_000_000_000_000),
		],
	}.assimilate_storage(&mut t).unwrap();
	GenesisConfig::<Test> {
		current_payout: 9500000,
		minting_interval: One::one(),
		recipients: recipients,
		recipient_percentages: pcts,
	}.assimilate_storage(&mut t).unwrap();

	t.into()
}

fn add_recipient(recipient: u64, percent: Percent) -> DispatchResult {
	TreasuryReward::add(RawOrigin::Root.into(), recipient, percent)
}

fn remove_recipient(recipient: u64) -> DispatchResult {
	TreasuryReward::remove(RawOrigin::Root.into(), recipient)
}

fn update(recipient: u64, percent: Percent) -> DispatchResult {
	TreasuryReward::update(RawOrigin::Root.into(), recipient, percent)
}


#[test]
fn basic_setup_works() {
	// Verifies initial conditions of mock
	new_test_ext(
		Some(vec![1, 2, 3]),
		Some(vec![Percent::from_percent(10), Percent::from_percent(10), Percent::from_percent(10)]),
	).execute_with(|| {
		// Initial Era and session
		let treasury_address = Treasury::account_id();
		System::set_block_number(1);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(1);
		System::set_block_number(2);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(2);
		System::set_block_number(100);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(101);
		System::set_block_number(101);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(102);
		System::set_block_number(102);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(103);
		System::set_block_number(103);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(104);
		assert_eq!(Balances::free_balance(treasury_address) > 0, true);
	});
}

#[test]
fn setting_treasury_block_reward () {
	// Verifies initial conditions of mock
	new_test_ext(
		Some(vec![]),
		Some(vec![]),
	).execute_with(|| {
		// Initial Era and session
		let treasury_address = Treasury::account_id();
		System::set_block_number(1);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(1);
		assert_eq!(Balances::free_balance(treasury_address), 9500000);
		System::set_block_number(2);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(2);
		assert_eq!(Balances::free_balance(treasury_address), 19000000);

		<TreasuryReward>::set_current_payout(Origin::root(), 95).unwrap();
		<TreasuryReward>::set_minting_interval(Origin::root(), 2).unwrap();
		
		System::set_block_number(3);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(3);
		assert_eq!(Balances::free_balance(treasury_address), 19000000);
		System::set_block_number(4);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(4);
		assert_eq!(Balances::free_balance(treasury_address), 19000095);

		<TreasuryReward>::set_current_payout(Origin::root(), 0).unwrap();

		System::set_block_number(5);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(5);
		assert_eq!(Balances::free_balance(treasury_address), 19000095);
		System::set_block_number(6);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(6);
		assert_eq!(Balances::free_balance(treasury_address), 19000095);

		<TreasuryReward>::set_current_payout(Origin::root(), 105).unwrap();

		System::set_block_number(7);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(7);
		assert_eq!(Balances::free_balance(treasury_address), 19000095);
		System::set_block_number(8);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(8);
		assert_eq!(Balances::free_balance(treasury_address), 19000200);

		<TreasuryReward>::set_minting_interval(Origin::root(), 1).unwrap();
		<TreasuryReward>::set_current_payout(Origin::root(), 10).unwrap();

		System::set_block_number(9);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(9);
		assert_eq!(Balances::free_balance(treasury_address), 19000210);
		System::set_block_number(10);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(10);
		assert_eq!(Balances::free_balance(treasury_address), 19000220);
	});
}

#[test]
fn add_and_remove_participants_without_dilution_augmentation() {
	new_test_ext(
		Some(vec![1, 2, 3]),
		Some(vec![Percent::from_percent(10), Percent::from_percent(10), Percent::from_percent(10)]),
	).execute_with(|| {
		// Add new recipient
		let recipient = 4;
		assert_ok!(add_recipient(recipient, Percent::from_percent(10)));
		// Check recipient is added successfully
		let recipients = <TreasuryReward>::recipients();
		assert_eq!(recipients, vec![1, 2, 3, 4]);
		// Check the available allocation is smaller
		let recipient_allocation = TreasuryReward::get_available_recipient_alloc();
		assert_eq!(recipient_allocation, Percent::from_percent(60));
		// Remove recipient
		assert_ok!(remove_recipient(recipient));
		let recipients = <TreasuryReward>::recipients();
		// Check recipient was removed successfully
		assert_eq!(recipients, vec![1, 2, 3]);
		// Check available allocation has grown from removing when there is room
		let recipient_allocation = TreasuryReward::get_available_recipient_alloc();
		assert_eq!(recipient_allocation, Percent::from_percent(70));
	});
}

#[test]
fn add_and_remove_participant_with_dilution_and_augmentation() {
	new_test_ext(
		Some(vec![1]),
		Some(vec![Percent::from_percent(100)]),
	).execute_with(|| {
		// Check the available allocation is zero
		let recipient_allocation = TreasuryReward::get_available_recipient_alloc();
		assert_eq!(recipient_allocation, Percent::from_percent(0));
		let alloc_1 = TreasuryReward::recipient_percentages(1).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(100));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		// Add new recipient
		let recipient = 2;
		assert_ok!(add_recipient(recipient, Percent::from_percent(50)));
		// Check the available allocation is still zero
		let recipient_allocation = TreasuryReward::get_available_recipient_alloc();
		assert_eq!(recipient_allocation, Percent::from_percent(0));
		// Check the individual allocations of recipients, ensure dilution occurred
		let mut alloc_1 = TreasuryReward::recipient_percentages(1).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(50));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		let alloc_2 = TreasuryReward::recipient_percentages(recipient).unwrap();
		assert_eq!(alloc_2.current, Percent::from_percent(50));
		assert_eq!(alloc_2.proposed, Percent::from_percent(50));
		// Remove recipient
		assert_ok!(remove_recipient(recipient));
		// Assert storage item was removed
		assert_eq!(TreasuryReward::recipient_percentages(recipient).is_none(), true);
		// Check augmented allocation is back to max for remaining participant
		alloc_1 = TreasuryReward::recipient_percentages(1).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(100));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
	});
}

#[test]
fn add_and_remove_many_participants() {
	new_test_ext(
		Some(vec![1]),
		Some(vec![Percent::from_percent(100)]),
	).execute_with(|| {
		let recipients = vec![2, 3, 4, 5, 6];
		// Add first dilution
		assert_ok!(add_recipient(recipients[0], Percent::from_percent(10)));
		// Check the individual allocations of recipients, ensure dilution occurred
		let mut alloc_1 = TreasuryReward::recipient_percentages(1).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(90));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		// Add second dilution
		assert_ok!(add_recipient(recipients[1], Percent::from_percent(10)));
		// Check the individual allocations of recipients, ensure dilution occurred
		alloc_1 = TreasuryReward::recipient_percentages(1).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(81));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		// Add third dilution
		assert_ok!(add_recipient(recipients[2], Percent::from_percent(10)));
		// Check the individual allocations of recipients, ensure dilution occurred
		alloc_1 = TreasuryReward::recipient_percentages(1).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(72));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		// Add fourth dilution
		assert_ok!(add_recipient(recipients[3], Percent::from_percent(10)));
		// Check the individual allocations of recipients, ensure dilution occurred
		alloc_1 = TreasuryReward::recipient_percentages(1).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(65));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		// Add fifth dilution
		assert_ok!(add_recipient(recipients[4], Percent::from_percent(10)));
		// Check the individual allocations of recipients, ensure dilution occurred
		alloc_1 = TreasuryReward::recipient_percentages(1).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(59));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));

		for i in 0..recipients.len() {
			let _ = TreasuryReward::recipient_percentages(1).unwrap();
			assert_ok!(remove_recipient(recipients[i]));
		}
		// Ensure augmentation occurred, lack of precision causes this to be lower than intended
		alloc_1 = TreasuryReward::recipient_percentages(1).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(97));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));		
	});
}

#[test]
fn add_and_remove_room() {
	new_test_ext(
		Some(vec![1]),
		Some(vec![Percent::from_percent(90)]),
	).execute_with(|| {
		let recipient = 2;
		// Add first dilution
		assert_ok!(add_recipient(recipient, Percent::from_percent(20)));
		// Check the individual allocations of recipients, ensure dilution occurred
		let mut alloc_1 = TreasuryReward::recipient_percentages(1).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(81));
		assert_eq!(alloc_1.proposed, Percent::from_percent(90));
		let alloc_2 = TreasuryReward::recipient_percentages(recipient).unwrap();
		assert_eq!(alloc_2.current, Percent::from_percent(19));
		assert_eq!(alloc_2.proposed, Percent::from_percent(20));
		assert_ok!(remove_recipient(recipient));
		alloc_1 = TreasuryReward::recipient_percentages(1).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(90));
		assert_eq!(alloc_1.proposed, Percent::from_percent(90));
		let sum = TreasuryReward::sum_percentages(TreasuryReward::get_recipient_pcts());
		assert_eq!(sum, 90);
	});
}

#[test]
fn update_after_adding_and_diluting_with_room() {
	new_test_ext(
		Some(vec![1]),
		Some(vec![Percent::from_percent(90)]),
	).execute_with(|| {
		let recipient = 2;
		// Add first dilution
		assert_ok!(add_recipient(recipient, Percent::from_percent(20)));
		assert_ok!(update(recipient, Percent::from_percent(30)));
		let alloc_2 = TreasuryReward::recipient_percentages(recipient).unwrap();
		assert_eq!(alloc_2.current, Percent::from_percent(28));
		assert_eq!(alloc_2.proposed, Percent::from_percent(30));

	});
}

#[test]
fn update_after_adding_and_diluting_without_room() {
	new_test_ext(
		Some(vec![1]),
		Some(vec![Percent::from_percent(100)]),
	).execute_with(|| {
		let recipient = 2;
		// Add first dilution
		assert_ok!(add_recipient(recipient, Percent::from_percent(20)));
		assert_ok!(update(recipient, Percent::from_percent(30)));
		let alloc_2 = TreasuryReward::recipient_percentages(recipient).unwrap();
		assert_eq!(alloc_2.current, Percent::from_percent(30));
		assert_eq!(alloc_2.proposed, Percent::from_percent(30));
	});
}

#[test]
fn high_recipient_percentage_should_fail() {
	new_test_ext(
		Some(vec![1, 2, 3]),
		Some(vec![Percent::from_percent(10), Percent::from_percent(10), Percent::from_percent(10)]),
	).execute_with(|| {
		assert_err!(add_recipient(4, Percent::from_percent(51)), "Invalid proposed percentage. Too large.");
	});
}

#[test]
fn payout_participants_and_treasury_successfully() {
	new_test_ext(
		Some(vec![1000, 1001, 1002]),
		Some(vec![Percent::from_percent(10), Percent::from_percent(10), Percent::from_percent(10)]),
	).execute_with(|| {
		// Initial Era and session
		let treasury_address = Treasury::account_id();
		assert_eq!(Balances::free_balance(1000) == 0, true);
		assert_eq!(Balances::free_balance(1001) == 0, true);
		assert_eq!(Balances::free_balance(1002) == 0, true);
		System::set_block_number(1);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(1);
		assert_eq!(Balances::free_balance(treasury_address), 8075000);
		assert_eq!(Balances::free_balance(1000), 475000);
		assert_eq!(Balances::free_balance(1001), 475000);
		assert_eq!(Balances::free_balance(1002), 475000);
	});
}