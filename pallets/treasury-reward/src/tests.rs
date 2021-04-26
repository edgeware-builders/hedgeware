use super::*;
use crate::mock::*;
use frame_support::{assert_ok, assert_noop};
use frame_support::{traits::{OnFinalize}};

#[test]
fn basic_setup_works() {
	// Verifies initial conditions of mock
	new_test_ext(
		Some(vec![AccountId::new([1; 32]), AccountId::new([2; 32]), AccountId::new([3; 32])]),
		Some(vec![Percent::from_percent(10), Percent::from_percent(10), Percent::from_percent(10)]),
	).execute_with(|| {
		// Initial Era and session
		let treasury_address: AccountId = TreasuryPalletId::get().into_account();
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
		assert_eq!(Balances::free_balance(treasury_address.clone()) > 0, true);
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
		let treasury_address: AccountId = TreasuryPalletId::get().into_account();
		System::set_block_number(1);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(1);
		assert_eq!(Balances::free_balance(treasury_address.clone()), 95 * DOLLARS);
		System::set_block_number(2);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(2);
		assert_eq!(Balances::free_balance(treasury_address.clone()), 190 * DOLLARS);

		<TreasuryReward>::set_current_payout(Origin::root(), 95).unwrap();
		<TreasuryReward>::set_minting_interval(Origin::root(), 2).unwrap();
		
		System::set_block_number(3);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(3);
		assert_eq!(Balances::free_balance(treasury_address.clone()), 190 * DOLLARS);
		System::set_block_number(4);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(4);
		assert_eq!(Balances::free_balance(treasury_address.clone()), 190 * DOLLARS + 95);

		<TreasuryReward>::set_current_payout(Origin::root(), 0).unwrap();

		System::set_block_number(5);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(5);
		assert_eq!(Balances::free_balance(treasury_address.clone()), 190 * DOLLARS + 95);
		System::set_block_number(6);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(6);
		assert_eq!(Balances::free_balance(treasury_address.clone()), 190 * DOLLARS + 95);

		<TreasuryReward>::set_current_payout(Origin::root(), 105).unwrap();

		System::set_block_number(7);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(7);
		assert_eq!(Balances::free_balance(treasury_address.clone()), 190 * DOLLARS + 95);
		System::set_block_number(8);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(8);
		assert_eq!(Balances::free_balance(treasury_address.clone()), 190 * DOLLARS + 200);

		<TreasuryReward>::set_minting_interval(Origin::root(), 1).unwrap();
		<TreasuryReward>::set_current_payout(Origin::root(), 10).unwrap();

		System::set_block_number(9);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(9);
		assert_eq!(Balances::free_balance(treasury_address.clone()), 190 * DOLLARS + 210);
		System::set_block_number(10);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(10);
		assert_eq!(Balances::free_balance(treasury_address.clone()), 190 * DOLLARS + 220);
	});
}

#[test]
fn add_and_remove_participants_without_dilution_augmentation() {
	new_test_ext(
		Some(vec![AccountId::new([1; 32]), AccountId::new([2; 32]), AccountId::new([3; 32])]),
		Some(vec![Percent::from_percent(10), Percent::from_percent(10), Percent::from_percent(10)]),
	).execute_with(|| {
		// Add new recipient
		let recipient = AccountId::new([4; 32]);
		assert_ok!(add_recipient(recipient.clone(), Percent::from_percent(10)));
		// Check recipient is added successfully
		let recipients = <TreasuryReward>::recipients();
		assert_eq!(
			recipients,
			vec![
				AccountId::new([1; 32]),
				AccountId::new([2; 32]),
				AccountId::new([3; 32]),
				AccountId::new([4; 32])
			]
		);
		// Check the available allocation is smaller
		let recipient_allocation = TreasuryReward::get_available_recipient_alloc();
		assert_eq!(recipient_allocation, Percent::from_percent(60));
		// Remove recipient
		assert_ok!(remove_recipient(recipient.clone()));
		let recipients = <TreasuryReward>::recipients();
		// Check recipient was removed successfully
		assert_eq!(recipients, vec![AccountId::new([1; 32]), AccountId::new([2; 32]), AccountId::new([3; 32])]);
		// Check available allocation has grown from removing when there is room
		let recipient_allocation = TreasuryReward::get_available_recipient_alloc();
		assert_eq!(recipient_allocation, Percent::from_percent(70));
	});
}

#[test]
fn add_and_remove_participant_with_dilution_and_augmentation() {
	new_test_ext(
		Some(vec![AccountId::new([1; 32])]),
		Some(vec![Percent::from_percent(100)]),
	).execute_with(|| {
		// Check the available allocation is zero
		let recipient_allocation = TreasuryReward::get_available_recipient_alloc();
		assert_eq!(recipient_allocation, Percent::from_percent(0));
		let alloc_1 = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(100));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		// Add new recipient
		let recipient = AccountId::new([2; 32]);
		assert_ok!(add_recipient(recipient.clone(), Percent::from_percent(50)));
		// Check the available allocation is still zero
		let recipient_allocation = TreasuryReward::get_available_recipient_alloc();
		assert_eq!(recipient_allocation, Percent::from_percent(0));
		// Check the individual allocations of recipients, ensure dilution occurred
		let mut alloc_1 = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(50));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		let alloc_2 = TreasuryReward::recipient_percentages(recipient.clone()).unwrap();
		assert_eq!(alloc_2.current, Percent::from_percent(50));
		assert_eq!(alloc_2.proposed, Percent::from_percent(50));
		// Remove recipient
		assert_ok!(remove_recipient(recipient.clone()));
		// Assert storage item was removed
		assert_eq!(TreasuryReward::recipient_percentages(recipient.clone()).is_none(), true);
		// Check augmented allocation is back to max for remaining participant
		alloc_1 = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(100));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
	});
}

#[test]
fn add_and_remove_many_participants() {
	new_test_ext(
		Some(vec![AccountId::new([1; 32])]),
		Some(vec![Percent::from_percent(100)]),
	).execute_with(|| {
		let recipients = vec![
			AccountId::new([2; 32]),
			AccountId::new([3; 32]),
			AccountId::new([4; 32]),
			AccountId::new([5; 32]),
			AccountId::new([6; 32]),
		];
		// Add first dilution
		assert_ok!(add_recipient(recipients[0].clone(), Percent::from_percent(10)));
		// Check the individual allocations of recipients, ensure dilution occurred
		let mut alloc_1 = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(90));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		// Add second dilution
		assert_ok!(add_recipient(recipients[1].clone(), Percent::from_percent(10)));
		// Check the individual allocations of recipients, ensure dilution occurred
		alloc_1 = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(81));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		// Add third dilution
		assert_ok!(add_recipient(recipients[2].clone(), Percent::from_percent(10)));
		// Check the individual allocations of recipients, ensure dilution occurred
		alloc_1 = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(72));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		// Add fourth dilution
		assert_ok!(add_recipient(recipients[3].clone(), Percent::from_percent(10)));
		// Check the individual allocations of recipients, ensure dilution occurred
		alloc_1 = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(65));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));
		// Add fifth dilution
		assert_ok!(add_recipient(recipients[4].clone(), Percent::from_percent(10)));
		// Check the individual allocations of recipients, ensure dilution occurred
		alloc_1 = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(59));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));

		for i in 0..recipients.len() {
			let _ = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
			assert_ok!(remove_recipient(recipients[i].clone()));
		}
		// Ensure augmentation occurred, lack of precision causes this to be lower than intended
		alloc_1 = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(97));
		assert_eq!(alloc_1.proposed, Percent::from_percent(100));		
	});
}

#[test]
fn add_and_remove_room() {
	new_test_ext(
		Some(vec![AccountId::new([1; 32])]),
		Some(vec![Percent::from_percent(90)]),
	).execute_with(|| {
		let recipient = AccountId::new([2; 32]);
		// Add first dilution
		assert_ok!(add_recipient(recipient.clone(), Percent::from_percent(20)));
		// Check the individual allocations of recipients, ensure dilution occurred
		let mut alloc_1 = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(81));
		assert_eq!(alloc_1.proposed, Percent::from_percent(90));
		let alloc_2 = TreasuryReward::recipient_percentages(recipient.clone()).unwrap();
		assert_eq!(alloc_2.current, Percent::from_percent(19));
		assert_eq!(alloc_2.proposed, Percent::from_percent(20));
		assert_ok!(remove_recipient(recipient.clone()));
		alloc_1 = TreasuryReward::recipient_percentages(AccountId::new([1; 32])).unwrap();
		assert_eq!(alloc_1.current, Percent::from_percent(90));
		assert_eq!(alloc_1.proposed, Percent::from_percent(90));
		let sum = TreasuryReward::sum_percentages(TreasuryReward::get_recipient_pcts());
		assert_eq!(sum, 90);
	});
}

#[test]
fn update_after_adding_and_diluting_with_room() {
	new_test_ext(
		Some(vec![AccountId::new([1; 32])]),
		Some(vec![Percent::from_percent(90)]),
	).execute_with(|| {
		let recipient = AccountId::new([2; 32]);
		// Add first dilution
		assert_ok!(add_recipient(recipient.clone(), Percent::from_percent(20)));
		assert_ok!(update(recipient.clone(), Percent::from_percent(30)));
		let alloc_2 = TreasuryReward::recipient_percentages(recipient.clone()).unwrap();
		assert_eq!(alloc_2.current, Percent::from_percent(28));
		assert_eq!(alloc_2.proposed, Percent::from_percent(30));

	});
}

#[test]
fn update_after_adding_and_diluting_without_room() {
	new_test_ext(
		Some(vec![AccountId::new([1; 32])]),
		Some(vec![Percent::from_percent(100)]),
	).execute_with(|| {
		let recipient = AccountId::new([2; 32]);
		// Add first dilution
		assert_ok!(add_recipient(recipient.clone(), Percent::from_percent(20)));
		assert_ok!(update(recipient.clone(), Percent::from_percent(30)));
		let alloc_2 = TreasuryReward::recipient_percentages(recipient.clone()).unwrap();
		assert_eq!(alloc_2.current, Percent::from_percent(30));
		assert_eq!(alloc_2.proposed, Percent::from_percent(30));
	});
}

#[test]
fn high_recipient_percentage_should_fail() {
	new_test_ext(
		Some(vec![AccountId::new([1; 32]), AccountId::new([2; 32]), AccountId::new([3; 32])]),
		Some(vec![Percent::from_percent(10), Percent::from_percent(10), Percent::from_percent(10)]),
	).execute_with(|| {
		assert_noop!(add_recipient(AccountId::new([4; 32]), Percent::from_percent(51)), "Invalid proposed percentage. Too large.");
	});
}

#[test]
fn payout_participants_and_treasury_successfully() {
	new_test_ext(
		Some(vec![
			AccountId::new([201; 32]),
			AccountId::new([202; 32]),
			AccountId::new([203; 32]),
		]),
		Some(vec![Percent::from_percent(10), Percent::from_percent(10), Percent::from_percent(10)]),
	).execute_with(|| {
		// Initial Era and session
		let treasury_address: AccountId = TreasuryPalletId::get().into_account();
		assert_eq!(Balances::free_balance(AccountId::new([201; 32])), 0);
		assert_eq!(Balances::free_balance(AccountId::new([202; 32])), 0);
		assert_eq!(Balances::free_balance(AccountId::new([203; 32])), 0);
		System::set_block_number(1);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(1);
		assert_eq!(Balances::free_balance(treasury_address.clone()), 8075 * DOLLARS / 100);
		assert_eq!(Balances::free_balance(AccountId::new([201; 32])), 475 * DOLLARS / 100);
		assert_eq!(Balances::free_balance(AccountId::new([202; 32])), 475 * DOLLARS / 100);
		assert_eq!(Balances::free_balance(AccountId::new([203; 32])), 475 * DOLLARS / 100);
	});
}