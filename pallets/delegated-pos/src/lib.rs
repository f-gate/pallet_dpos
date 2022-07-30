#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::traits:: {
		Currency,
		ReservableCurrency,
	};

	use frame_support::traits::tokens::Balance;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	type BalanceOf<T> = <<T as Config>::MyToken as Currency<<T as frame_system::Config>::AccountId>>::Balance;


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MyToken: ReservableCurrency<Self::AccountId>;

		///type ReservationFee: Get<<<Self as Config>::MyToken as Currency<<Self as Config>::AccountId>>::Balance>;
		type ForceOrigin: EnsureOrigin<Self::Origin>;

		///The minimun amount one can delegate to avoid spam attacks
		type MinDelegateAmount: Balance;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	
	///Prefix: Delegate
	///Key (1 , 2): (Delegated in question. Account in question)
	///Value: Amount delegated.
	#[pallet::storage]
	pub type DelegatedTokens<T: Config> = StorageDoubleMap<_,Blake2_128Concat, T::AccountId,
															 Blake2_128Concat, T::AccountId, u64, ValueQuery>;
	/// Key: DelegateID, UnitType
	/// must have either a BABE session key or voted on by governance.
	#[pallet::storage]
	pub type IsDelegatable<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (), ValueQuery>;
	#[pallet::storage]
	pub type Nominators<T: Config> = StorageValue<_, Blake2_128Concat, (), ValueQuery>;
	
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		StakeChange(T::AccountId),
		HasDelegated(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NotEnoughFunds,
		ValidatorMaxStake,
		BelowMinimumAmount,
		NotDelegatable,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Delegate amount of tokens to a user who is a known delegate or validator.
		/// Known delegators can only delegate to validators.
		#[pallet::weight(10000)]
		pub fn delegate_tokens(origin: OriginFor<T>, delegate: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
			// Ensure that :
			// Sender is legit
			// The recipient is Delegatable (either voted and known or a validator). 
			// The sender has enough funds.
			// The recipient is delegatable (either voted on or a validator).
			let sender = ensure_signed(origin)?;
			
			ensure!(IsDelegatable::<T>::contains_key(&delegate), Error::<T>::NotDelegatable);

			ensure!(amount > <T>::MinDelegateAmount, Error::<T>::BelowMinimumAmount);
			
			//ensure!(T::MyToken::can_reserve(origin::Origin::AccoundId, amount), Error::<T>::NotEnoughFunds);
			
			//T::MyToken::reserve(&sender, amount.into()).expect("ensure reserve amount has been called. qed");

			 //Send equal amount also to DelegatedTokens
			 //todo: find out what to store this as so that we can do DELEGATEDID -> SENDERID = amount.
			 //DelegatedTokens::<T>::try_mutate(origin, delegate)

			
			Self::deposit_event(Event::HasDelegated(sender));
			Ok(())
		} 

		#[pallet::weight(10000)]
		pub fn revoke_delegation(origin: OriginFor<T>, delegate: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
			unimplemented!();
		}

		#[pallet::weight(10000)]
		pub fn revoke_delegation_all(origin: OriginFor<T>) -> DispatchResult {
			unimplemented!();
		}

		#[pallet::weight(10000)]
		pub fn auto_delegate_validators(origin: OriginFor<T>) -> DispatchResult {
			unimplemented!();
		}

		#[pallet::weight(10000)]
		pub fn make_delegatable(origin: OriginFor<T>) -> DispatchResult {
			unimplemented!();
		}
		
		#[pallet::weight(10000)]
		pub fn revoke_delegatable(origin: OriginFor<T>) -> DispatchResult {
			unimplemented!();
		}

		/* #[pallet::weight(1_000)]
		pub fn create_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
			// proof: BoundedVec<u8, T::HashLimit>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has not already been claimed.
			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			T::MyToken::reserve(&sender, 1000u32.into())?;

			// Get the block number from the FRAME System pallet.
			let current_block = <frame_system::Pallet<T>>::block_number();

			// Store the proof with the sender and block number.
			Proofs::<T>::insert(&proof, (&sender, current_block));

			// Emit an event that the claim was created.
			Self::deposit_event(Event::ClaimCreated(sender, proof));
			Ok(())
		}
 */
	}
}

