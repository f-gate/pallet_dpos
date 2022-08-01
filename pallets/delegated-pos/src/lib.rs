#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
	};
	use sp_runtime::traits::Zero;

	use frame_support::traits::tokens::Balance;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	type BalanceOf<T> =
		<<T as Config>::MyToken as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MyToken: ReservableCurrency<Self::AccountId>;

		///type ReservationFee: Get<<<Self as Config>::MyToken as Currency<<Self as
		/// Config>::AccountId>>::Balance>;
		type ForceOrigin: EnsureOrigin<Self::Origin>;

		//The minimun amount one can delegate to avoid spam attacks
		//type MinDelegateAmount: Balance;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	//#[pallet::genesis_config]
	//pub struct GenesisConfig<T: Config> {
	//  pub staked_tokens: StorageDoubleMap,
	//  pub max_members: Option<u32>,
	//}
	//
	//#[pallet::genesis_build]
	//impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
	//fn build(&self) {
	//// self.max
	//	 }
	//}

	///Key (1 , 2): (Delegated in question. Account in question)
	///Value: Amount staked.
	#[pallet::storage]
	pub type StakedTokens<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::AccountId,
		BalanceOf<T>,
		ValueQuery,
	>;

	///List of people someone has Staked to.
	///Key: (Account in question.)
	///Value:(List of Validators in question)
	#[pallet::storage]
	pub type StakedList<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<T::AccountId, ConstU32<100>>,
		ValueQuery,
	>;

	/// voted on by governance to delegate votes.
	#[pallet::storage]
	pub type IsValidator<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (), ValueQuery>;

	///storage value of all the validators.
	#[pallet::storage]
	pub type Validators<T: Config> =
		StorageValue<_, BoundedVec<(T::AccountId, u8), ConstU32<100>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		StakeRemoved(T::AccountId),
		HasStaked(T::AccountId),
		HasDelegated(T::AccountId),
		ValidatorAdded(T::AccountId),
		ValidatorRemoved(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NotEnoughFunds,
		ValidatorMaxStake,
		BelowMinimumAmount,
		NotValidator,
		StakeIsZero,
		AlreadyValidator,
		BadAuraKey,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Delegate amount of tokens to a user who is a known delegate.
		/// Known delegators can only delegate to validators. - To stop long delegation attack.
		#[pallet::weight(10000)]
		pub fn stake_tokens(
			origin: OriginFor<T>,
			validator: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			// Ensure that : Sender is legit
			// The recipient is Delegatable (either voted and known or a validator).
			// The sender has enough funds.
			// The origin is not already a delegate.
			let sender = ensure_signed(origin)?;
			ensure!(IsValidator::<T>::contains_key(&validator), Error::<T>::NotValidator);
			ensure!(!IsValidator::<T>::contains_key(&sender), Error::<T>::NotValidator);

			//TODO: URGENT HOW TO COMPARE BALANCES
			ensure!(amount > amount, Error::<T>::BelowMinimumAmount);
			ensure!(T::MyToken::can_reserve(&sender, amount), Error::<T>::NotEnoughFunds);

			T::MyToken::reserve(&sender, amount.into())
				.expect("ensure reserve amount has been called. qed");
			//Send equal amount also to DelegatedTokens
			if let Ok(n) = StakedTokens::<T>::try_get(&validator, &sender) {
				//If exists just add
				StakedTokens::<T>::insert(&validator, &sender, n + amount);
			} else {
				//Add to delegated list for OriginID
				StakedTokens::<T>::insert(&validator, &sender, amount);
				//TODO: CHECK MAX LENGTH
				StakedList::<T>::try_append(&sender, &validator)
					.expect("max len has been checked. qed");
			}

			Self::deposit_event(Event::HasStaked(sender));
			Ok(())
		}

		///Will revoke the entire stake from a validator for the origin
		#[pallet::weight(10000)]
		pub fn revoke_stake(origin: OriginFor<T>, validator: T::AccountId) -> DispatchResult {
			//todo extra ensures
			let sender = ensure_signed(origin)?;
			let stake = StakedTokens::<T>::take(&validator, &sender);

			ensure!(stake > Zero::zero(), Error::<T>::StakeIsZero);
			T::MyToken::unreserve(&sender, stake.into());

			let vals_new: Vec<T::AccountId> = StakedList::<T>::take(&sender)
				.into_iter()
				.filter(|id| id == &validator)
				.collect();

			let bounded: BoundedVec<T::AccountId, ConstU32<100>> = vals_new.try_into().unwrap();
			StakedList::<T>::insert(&sender, bounded);

			Self::deposit_event(Event::StakeRemoved(sender));
			Ok(())
		}

		//TODO: Aura implementation.
		/// Add a validator to the staking system given a valid aura key
		#[pallet::weight(10000)]
		pub fn add_validator(origin: OriginFor<T>, aura_key: bool) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			//TODO: CHECK AURA_KEY
			ensure!(aura_key, Error::<T>::BadAuraKey);

			ensure!(!IsValidator::<T>::contains_key(&sender), Error::<T>::AlreadyValidator);
			IsValidator::<T>::insert(&sender, ());
			Self::deposit_event(Event::ValidatorAdded(sender));
			Ok(())
		}

		///Remove validator if is validator
		#[pallet::weight(10000)]
		pub fn remove_validator(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(IsValidator::<T>::contains_key(&sender), Error::<T>::NotValidator);
			IsValidator::<T>::remove(&sender);
			Self::deposit_event(Event::ValidatorRemoved(sender));
			Ok(())
		}
	}
}
