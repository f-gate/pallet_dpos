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
		traits::{
			Currency, ReservableCurrency,
		},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{Block, Extrinsic, Zero, Saturating};
	use sp_std::vec::Vec;
	use pallet_session::{SessionManager, SessionHandler};
	
	type SessionIndex = u32;
	type BalanceOf<T> =
		<<T as Config>::MyToken as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_session::Config  {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MyToken: ReservableCurrency<Self::AccountId>;
		type ForceOrigin: EnsureOrigin<Self::Origin>;
		type MinimumStake: Get<BalanceOf<Self>>;
		type BlocksTillSwap: Get<Self::BlockNumber>;
	}

	

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

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
	pub type AccountHasStakedTo<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<T::AccountId, ConstU32<100>>,
		ValueQuery,
	>;

	/// voted on by governance to delegate votes.
	#[pallet::storage]
	pub type IsValidator<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (), ValueQuery>;

	#[pallet::storage]
	pub type ActiveSet<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, ConstU32<100>>, ValueQuery>;

	///storage value of all the validators.
	#[pallet::storage]
	pub type Validators<T: Config> =
		StorageValue<_, BoundedVec<(T::AccountId, BalanceOf<T>), ConstU32<100>>, ValueQuery>;

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

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub init_validators: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {init_validators: Vec::new()}
		}
	}
	///Take the genisis build and push validators into storage
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for auth in &self.init_validators {
				IsValidator::<T>::insert(auth.clone(), ());
			}

			//set all validators inititally to authorities
			ActiveSet::<T>::set(self.init_validators.clone().try_into().unwrap());

			//turn to tuple with value 0 to insert to Validator:Stake
			let tuple_vec: Vec<(T::AccountId,  BalanceOf<T>)> =
				self.init_validators.iter().map(|auth| (auth.clone(), Default::default())).collect();

			let bounded_vec: BoundedVec<(T::AccountId,  BalanceOf<T>), ConstU32<100>> =
				tuple_vec.try_into().unwrap();
			Validators::<T>::set(bounded_vec);
		}
	}

	///on init we get all the validators, sort by stake, grab top one third the account ids.
	///push into ActiveSet
	///currently no rewards for being in active stake
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		//todo: add BlocksTillSwap
		fn on_initialize(current_block_num: T::BlockNumber) -> Weight {
			let mut weight= 0u64;
			let weight_multiple= 10000u64;

			if current_block_num % T::BlocksTillSwap::get() == Zero::zero() {
				
				let mut validators = Validators::<T>::get();
				weight = validators.len() as u64 * weight_multiple;

				validators.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
				
				let middleman: Vec<T::AccountId> =
					validators[((validators.len() * 2) / 3)..validators.len()]
					.	iter()
						.map(|item| item.0.clone())
						.collect();

				if let Ok(n) = middleman.try_into() {
					ActiveSet::<T>::kill();
					ActiveSet::<T>::set(n);
				}
			}

			weight
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyValidator,
		BadAuraKey,
		BelowMinimumAmount,
		CannotStakeAsValidator,
		NotValidator,
		NotEnoughFunds,
		NoValidators,
		StakeIsZero,
		TooManyValidators,
		ValidatorMaxStake,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Delegate amount of tokens to a user who is a known delegate.
		/// Known delegators can only delegate to validators. - To stop long delegation attack.
		//todo: insert into 
		#[pallet::weight(10000)]
		pub fn stake_tokens(
			origin: OriginFor<T>,
			validator: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// The recipient is Delegatable (either voted and known or a validator).
			ensure!(IsValidator::<T>::contains_key(&validator), Error::<T>::NotValidator);
			// The origin is not already a delegate.
			ensure!(!IsValidator::<T>::contains_key(&sender), Error::<T>::CannotStakeAsValidator);

			// The sender has enough funds.
			
			ensure!(T::MyToken::can_reserve(&sender, amount), Error::<T>::NotEnoughFunds);
//			ensure!(T::MinimumStake::get() > amount, Error::<T>::BelowMinimumAmount);

			T::MyToken::reserve(&sender, amount.into())
				.expect("ensure reserve amount has been called. qed");
			//Send equal amount also to DelegatedTokens
			if let Ok(n) = StakedTokens::<T>::try_get(&validator, &sender) {
				//If exists just add
				StakedTokens::<T>::insert(&validator, &sender, amount.saturating_add(n));
			} else {
				//Add to delegated list for OriginID
				StakedTokens::<T>::insert(&validator, &sender, amount);
				ensure!(
					AccountHasStakedTo::<T>::try_append(&sender, &validator).is_ok(),
					Error::<T>::TooManyValidators
				)
			}

			//update validators staked amount todo: rafactor nicer solution
			let validator_totals_new: Vec<(T::AccountId,  BalanceOf<T>)> = Validators::<T>::take()
				.into_iter()
				.map(|val| {
					if val.0 == validator {
						 val.1.saturating_add(amount.into());
					}
					val
				}).collect();
			let bounded: BoundedVec<(T::AccountId,  BalanceOf<T>), ConstU32<100>> = validator_totals_new.try_into().unwrap();
			Validators::<T>::set(bounded);


			Self::deposit_event(Event::HasStaked(sender));
			Ok(())
		}

		///Will revoke the entire stake from a validator for the origin
		#[pallet::weight(10000)]
		pub fn revoke_stake(origin: OriginFor<T>, validator: T::AccountId) -> DispatchResult {
			//todo extra ensures
			let sender = ensure_signed(origin)?;

			// Ensure the account id is a validator
			ensure!(IsValidator::<T>::contains_key(&validator), Error::<T>::NotValidator);

			// Ensure the stake is not zero
			let stake = StakedTokens::<T>::take(&validator, &sender);
			ensure!(stake > Zero::zero(), Error::<T>::StakeIsZero);
			
			// Unreserve all tokens from that validator user pair
			T::MyToken::unreserve(&sender, stake.into());

			// Update the accounts staked to
			let vals_new: Vec<T::AccountId> = AccountHasStakedTo::<T>::take(&sender)
				.into_iter()
				.filter(|id| !(id == &validator))
				.collect();

			let bounded: BoundedVec<T::AccountId, ConstU32<100>> = vals_new.try_into().unwrap();
			AccountHasStakedTo::<T>::insert(&sender, bounded);

			// Update the validators list
			let validator_totals_new: Vec<(T::AccountId,  BalanceOf<T>)> = Validators::<T>::take()	
			.into_iter()
			.filter(|id| !(id.0 == validator))
			.collect();
			let bounded_vec_res: BoundedVec<(T::AccountId, BalanceOf<T>), ConstU32<100>> = validator_totals_new.try_into().expect("reducing, wont be out of bounds.");
			Validators::<T>::set(bounded_vec_res);

			Self::deposit_event(Event::StakeRemoved(sender));
			Ok(())
		}

		/// Add a validator to the staking system given a valid aura key
		#[pallet::weight(10000)]
		pub fn add_validator(origin: OriginFor<T>, validator: T::AccountId) -> DispatchResult {
			let sender = ensure_root(origin)?;
			//ensure they are not already a validator
			ensure!(!IsValidator::<T>::contains_key(&validator), Error::<T>::AlreadyValidator);
			IsValidator::<T>::insert(&validator, ());
			Self::deposit_event(Event::ValidatorAdded(validator));
			Ok(())
		}

		///Remove validator if is validator
		#[pallet::weight(10000)]
		pub fn remove_validator(origin: OriginFor<T>, validator: T::AccountId) -> DispatchResult {
			let sender = ensure_root(origin)?;
			ensure!(IsValidator::<T>::contains_key(&validator), Error::<T>::NotValidator);
			IsValidator::<T>::remove(&validator);
			Self::deposit_event(Event::ValidatorRemoved(validator));
			Ok(())
		}
	}

    pub struct SessionManagerDpos<T>(sp_std::marker::PhantomData<T>);

	impl<T: Config> SessionManager<T::ValidatorId> for SessionManagerDpos<T> { 
        // Every call this will return the newly updated set of validators
        fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
            let active_set = ActiveSet::<T>::get();
            if active_set.len() == 0 {
                return None
            }

            // Grab all the accountids of validators and turn them into ValidatorIds
            let vec_val_id: Vec<T::ValidatorId> = active_set.into_iter().map(|account_id| {
                let res  = account_id.try_into();
                if let Ok(n) = res {
                    return Some(n)
                } else {
                    return None
                }
            }).flatten().collect::<Vec<T::ValidatorId>>();
			Some(vec_val_id)
        }

        fn end_session(end_index: SessionIndex) {

        }

        fn start_session(start_index: SessionIndex){
            todo!()
        }

        fn new_session_genesis(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> { 
            Self::new_session(0)
        }

    }

}
