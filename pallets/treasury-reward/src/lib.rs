#![cfg_attr(not(feature = "std"), no_std)]

use codec::*;
use sp_std::prelude::*;
use sp_runtime::traits::{Saturating, Zero};
use sp_runtime::{Percent, RuntimeDebug};
use frame_system::{Config as SystemConfig};
use frame_support::traits::Currency;

#[derive(Encode, Decode, Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
pub struct RecipientAllocation {
	pub proposed: Percent,
	pub current: Percent,
}

pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_treasury::Config + pallet_balances::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Origin: From<<Self as SystemConfig>::Origin>;

		/// The overarching call type; we assume sibling chains use the same type.
		type Call: From<Call<Self>> + Encode;

		/* Other dependencies */

		/// The currency type
		type Currency: Currency<Self::AccountId>;

		/* module-specific types */

		/// Minimum fraction of a treasury reward that goes to the Treasury account itself
		type MinimumTreasuryPct: Get<Percent>;
		/// Maximum fraction of a treasury reward that goes to an individual non-Treasury recipient itself
		type MaximumRecipientPct: Get<Percent>;
	}

	// How often the reward should occur
	#[pallet::storage]
	#[pallet::getter(fn minting_interval)]
	pub(super) type MintingInterval<T: Config> = StorageValue<
		_,
		T::BlockNumber,
	>;

	// The total amount that is paid out
	#[pallet::storage]
	#[pallet::getter(fn current_payout)]
	pub(super) type CurrentPayout<T: Config> = StorageValue<
		_,
		BalanceOf<T>,
	>;

	// Treasury reward recipients
	#[pallet::storage]
	#[pallet::getter(fn recipients)]
	pub(super) type Recipients<T: Config> = StorageValue<
		_,
		Vec<T::AccountId>,
	>;

	/// Treasury reward percentages mapping
	#[pallet::storage]
	#[pallet::getter(fn recipient_percentages)]
	pub(super) type RecipientPercentages<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		RecipientAllocation,
		OptionQuery,
	>;

	/*

		pub enum Event<T> where <T as frame_system::Config>::BlockNumber,
							<T as frame_system::Config>::AccountId,
							Balance = <T as pallet_balances::Config>::Balance,
							Payout = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance 
	*/

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::BlockNumber = "BlockNumber")]
	pub enum Event<T: Config> {
		TreasuryMinting(Balance, BlockNumber, AccountId),
		RecipientAdded(AccountId, Percent),
		RecipientRemoved(AccountId),
		MintingIntervalUpdate(BlockNumber),
		RewardPayoutUpdate(Payout),
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		/*

		/// Mint money for the treasury and recipient pool!
		fn on_finalize(_n: T::BlockNumber) {
			if <frame_system::Pallet<T>>::block_number() % Self::minting_interval() == Zero::zero() {
				let reward = Self::current_payout();
				// get up front treasury reward from minimum amount that is always allocated
				let mut treasury_reward = T::MinimumTreasuryPct::get() * reward;
				// up front allocation being split between recipients, any leftover goes to Treasury
				let leftover_recipient_alloc = Self::get_leftover(T::MinimumTreasuryPct::get());
				// up front reward that gets divided between recipients; the recipients current
				// allocation percentage denotes their fraction of the leftover_recipient_allocation
				let leftover_recipients_reward = leftover_recipient_alloc * reward;
				let recipients = Self::recipients();
				let mut allocated_to_recipients: BalanceOf<T> = 0u32.into();
				for i in 0..recipients.len() {
					if let Some(alloc) = Self::recipient_percentages(recipients[i].clone()) {
						// calculate fraction for recipient i
						let reward_i = alloc.current * leftover_recipients_reward;
						// reward the recipient
						<T as Config>::Currency::deposit_creating(
							&recipients[i].clone(),
							reward_i.clone(),
						);
						// emit event of payout
						Self::deposit_event(Event::TreasuryMinting(
							<pallet_balances::Pallet<T>>::free_balance(recipients[i].clone()),
							<frame_system::Pallet<T>>::block_number(),
							recipients[i].clone())
						);
						// track currently allocated amount to recipients
						allocated_to_recipients = allocated_to_recipients + reward_i;
					}
				}

				// update treasury reward with any leftover reward deducted by what was allocated
				// or ensure that if no recipients exist, to provide entire reward to the treasury
				if recipients.len() == 0 {
					treasury_reward = reward;
				} else {
					treasury_reward = treasury_reward + leftover_recipients_reward - allocated_to_recipients;
				}

				// allocate reward to the Treasury
				<T as Config>::Currency::deposit_creating(
					&<pallet_treasury::Pallet<T>>::account_id(),
					treasury_reward,
				);
				// emit event of payout
				Self::deposit_event(Event::TreasuryMinting(
					<pallet_balances::Pallet<T>>::free_balance(<pallet_treasury::Pallet<T>>::account_id()),
					<frame_system::Pallet<T>>::block_number(),
					<pallet_treasury::Pallet<T>>::account_id())
				);
			}
		}

		*/

	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Updates the minting interval of the treasury reward process
		#[pallet::weight(5_000_000)]
		fn set_minting_interval(origin: OriginFor<T>, interval: T::BlockNumber) -> DispatchResult {
			ensure_root(origin)?;
			<MintingInterval<T>>::put(interval);
			Self::deposit_event(Event::MintingIntervalUpdate(interval));
			Ok(())
		}

		/*

		/// Adds a new recipient to the recipients list and assigns them
		/// the submitted percentage of the leftover treasury reward.
		/// If there is no leftover allocation, the other recipients'
		/// reward percentages will be diluted.
		#[pallet::weight(5_000_000)]
		fn add(origin: OriginFor<T>, recipient: T::AccountId, pct: Percent) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(pct.deconstruct() <= T::MaximumRecipientPct::get().deconstruct(), "Invalid proposed percentage. Too large.");
			ensure!(!Self::recipients().contains(&recipient), "Duplicate recipients not allowed");
			let leftover_allocation = Self::get_available_recipient_alloc();
			// Dilute current allocations by overflowed percentage
			if pct.deconstruct() > leftover_allocation.deconstruct() {
				let diff = pct.saturating_sub(leftover_allocation);
				let first_portion = pct.saturating_sub(diff);
				let second_portion = pct.saturating_sub(first_portion);
				// Add new recipient with diff first, this needs to be diluted with the leftover
				Self::add_recipient(recipient.clone(), pct, first_portion);
				ensure!(Self::sum_percentages(Self::get_recipient_pcts()) <= 100, "Invalid percentage calculation");
				Self::dilute_percentages(diff);
				Self::add_to_allocation(recipient, second_portion);
			} else {
				// Add new recipient
				Self::add_recipient(recipient, pct, pct);
			}
			ensure!(Self::sum_percentages(Self::get_recipient_pcts()) <= 100, "Invalid percentage calculation");
			Ok(())
		}

		/// Removes an existing recipient from the active list and dilutes
		/// all remaining participants current percentages by that deleted amount.
		/// Dilution should only occur up until the proposed percentages each
		/// active participant was added to the set with.
		#[pallet::weight(5_000_000)]
		fn remove(origin: OriginFor<T>, recipient: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(Self::recipients().contains(&recipient), "Recipient doesn't exist");
			// Get removed recipient percentrage and calculate augmented percentages.
			let pct = <RecipientPercentages<T>>::get(recipient.clone()).unwrap();
			// Remove recipient from pool and the mapping to their allocation
			Self::remove_recipient(recipient.clone());
			// Calculation occurs over updated set of recipients since we put it back.
			Self::augment_percentages(pct.proposed);
			ensure!(Self::sum_percentages(Self::get_recipient_pcts()) <= 100, "Invalid percentage calculation");
			Ok(())
		}

		/// Updates an existing recipients allocation by removing and adding
		/// them into the set. This will cause a dilution and inflation of the
		/// set and does lose precision in the process.
		#[pallet::weight(5_000_000)]
		fn update(origin: OriginFor<T>, recipient: T::AccountId, pct: Percent) -> DispatchResult {
			ensure_root(origin.clone())?;
			ensure!(pct.deconstruct() <= T::MaximumRecipientPct::get().deconstruct(), "Invalid proposed percentage. Too large.");
			Self::remove(origin.clone(), recipient.clone()).map_err(|_| Error::<T>::FailedToRemove)?;
			Self::add(origin, recipient, pct).map_err(|_| Error::<T>::FailedToAdd)?;
			ensure!(Self::sum_percentages(Self::get_recipient_pcts()) <= 100, "Invalid percentage calculation");
			Ok(())
		}

		/// Updates the minting interval of the treasury reward process
		#[pallet::weight(5_000_000)]
		fn set_minting_interval(origin: OriginFor<T>, interval: T::BlockNumber) -> DispatchResult {
			ensure_root(origin)?;
			<MintingInterval<T>>::put(interval);
			Self::deposit_event(Event::MintingIntervalUpdate(interval));
			Ok(())
		}

		/// Updates the current payout of the treasury reward process
		#[pallet::weight(5_000_000)]
		fn set_current_payout(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
			ensure_root(origin)?;
			<CurrentPayout<T>>::put(amount);
			Self::deposit_event(Event::RewardPayoutUpdate(amount));
			Ok(())
		}

		*/

		
	}
}

impl<T: Config> Pallet<T> {

	fn get_recipient_pcts() -> Vec<Percent> {
		let recipients = Self::recipients();
		let mut pcts = vec![];
		for i in 0..recipients.len() {
			if let Some(alloc) = Self::recipient_percentages(recipients[i].clone()) {
				pcts.push(alloc.current);
			}
		}

		return pcts;
	}

	/// Calculates the difference between 100 percent and a provided percentage 
	fn get_leftover(pct: Percent) -> Percent {
		Percent::from_percent(100).saturating_sub(pct)
	}

}


