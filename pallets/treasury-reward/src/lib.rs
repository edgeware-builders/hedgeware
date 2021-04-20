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
		#[pallet::constant]
		type MinimumTreasuryPct: Get<Percent>;

		/// Maximum fraction of a treasury reward that goes to an individual non-Treasury recipient itself
		#[pallet::constant]
		type MaximumRecipientPct: Get<Percent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::BlockNumber = "BlockNumber", T::AccountId = "AccountId", BalanceOf<T> = "Balance")]
	pub enum Event<T: Config> {
		TreasuryMinting(BalanceOf<T>, T::BlockNumber, T::AccountId),
		RecipientAdded(T::AccountId, Percent),
		RecipientRemoved(T::AccountId),
		MintingIntervalUpdate(T::BlockNumber),
		RewardPayoutUpdate(BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		FailedToAdd,
		FailedToRemove,
	}

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
		ValueQuery,
	>;

	/// Treasury reward percentages mapping
	#[pallet::storage]
	#[pallet::getter(fn recipient_percentages)]
	pub(super) type RecipientPercentages<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		RecipientAllocation,
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub recipients: Vec<T::AccountId>,
		pub recipient_percentages: Vec<Percent>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		// type default or default provided for fields
		fn default() -> Self {
			Self {
				recipients: Default::default(),
				recipient_percentages: Default::default(),
			}
		}
	}
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// The add_extra_genesis build logic
			<Pallet<T>>::initialize_recipients(self.recipients.clone(), self.recipient_percentages.clone());
		}
	}
}

impl<T: Config> Pallet<T> {

	pub fn initialize_recipients(recipients: Vec<T::AccountId>, pcts: Vec<Percent>) {
		assert!(recipients.len() == pcts.len(), "There must be a one-to-one mapping between recipients and percentages");
		<Recipients<T>>::put(recipients.clone());
		// Sum all percentages to ensure they're bounded by 100
		let sum = Self::sum_percentages(pcts.clone());
		assert!(sum <= 100, "Percentages must sum to at most 100");
		for i in 0..recipients.clone().len() {
			<RecipientPercentages<T>>::insert(recipients[i].clone(), RecipientAllocation {
				current: pcts[i],
				proposed: pcts[i],
			});
		}	
	}

	fn dilute_percentages(new_pct: Percent) {
		let recipients = Self::recipients();
		let dilution_frac = Self::get_leftover(new_pct);
		// multiply all percentages by dilution fraction
		for i in 0..recipients.len() {
			if let Some(mut alloc) = Self::recipient_percentages(recipients[i].clone()) {
				alloc.current = alloc.current.saturating_mul(dilution_frac);
				<RecipientPercentages<T>>::insert(recipients[i].clone(), alloc);
			}
		}
	}

	fn augment_percentages(old_pct: Percent) {
		let recipients = Self::recipients();
		let augment_frac = Self::get_leftover(old_pct);
		// divide all percetages by augment fraction
		for i in 0..recipients.len() {
			if let Some(mut alloc) = Self::recipient_percentages(recipients[i].clone()) {
				alloc.current = alloc.current / augment_frac;
				// Ensure augmenting never leads to higher than proposed allocation 
				if alloc.current.deconstruct() > alloc.proposed.deconstruct() {
					alloc.current = alloc.proposed;
				}
				<RecipientPercentages<T>>::insert(recipients[i].clone(), alloc);
			}
		}
	}

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

	/// Sums a vector of percentages
	fn sum_percentages(pcts: Vec<Percent>) -> u8 {
		let mut pct = 0;
		for i in 0..pcts.len() {
			pct += pcts[i].deconstruct();
		}

		return pct;
	}

	/// Calculates the difference between 100 percent and a provided percentage 
	fn get_leftover(pct: Percent) -> Percent {
		Percent::from_percent(100).saturating_sub(pct)
	}

	/// Calculates the remaining, leftover percentage that can be allocated
	/// to any set of recipients without diluting all the other recipients
	/// allocation
	fn get_available_recipient_alloc() -> Percent {
		let recipients = Self::recipients();
		let mut pct_sum = Percent::from_percent(0);
		for i in 0..recipients.len() {
			if let Some(alloc) = Self::recipient_percentages(recipients[i].clone()) {
				pct_sum = pct_sum.saturating_add(alloc.current);
			}
		}

		return Self::get_leftover(pct_sum);
	}

	/// Helper function to add a recipient into the module's storage
	fn add_recipient(recipient: T::AccountId, proposed_pct: Percent, current_pct: Percent) {
		let mut recipients = Self::recipients();
		// Add the new recipient to the pool
		recipients.push(recipient.clone());
		<Recipients<T>>::put(recipients);
		// Add the recipients percentage
		<RecipientPercentages<T>>::insert(recipient.clone(), RecipientAllocation {
			current: current_pct,
			proposed: proposed_pct,
		});
		Self::deposit_event(Event::RecipientAdded(recipient, proposed_pct));
	}

	/// Helper function to remove a recipient from the module's storage
	fn remove_recipient(recipient: T::AccountId) {
		let mut recipients = Self::recipients();
		// Find recipient index and remove them
		let index = recipients.iter().position(|x| *x == recipient).unwrap();
		recipients.remove(index);
		// Put recipients back
		<Recipients<T>>::put(recipients.clone());
		// Remove the removed recipient's percentage from the map
		<RecipientPercentages<T>>::remove(recipient.clone());
		Self::deposit_event(Event::RecipientRemoved(recipient));
	}

	/// Adds a percentage increase to a recipients allocation
	fn add_to_allocation(recipient: T::AccountId, pct: Percent) {
		if let Some(mut alloc) = Self::recipient_percentages(recipient.clone()) {
			alloc.current = alloc.current.saturating_add(pct);
			<RecipientPercentages<T>>::insert(recipient, alloc);
		}
	}
}


