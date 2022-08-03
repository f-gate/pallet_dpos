use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::Hooks};
#[test]
fn test_genesis() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let top_dogs = ActiveSet::<Test>::get();
		assert!(validators.len() > 0);
		assert!(top_dogs.len() > 0);
	})
}

#[test]
fn test_delegate_tokens_not_enough_token() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 1000u64;
		assert_noop!(Dpos::stake_tokens(Origin::signed(alice), validators[0].0, 10000), Error::<Test>::NotEnoughFunds);
	})
}

//type MinimumStake = ConstU64<500>;
#[test]
fn test_delegate_tokens_below_minimum() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 0u64;
		assert_ok!(Dpos::stake_tokens(Origin::signed(alice), validators[0].0, 400));
	})
}



#[test]
fn test_delegate_tokens_enough_tokens() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 0u64;
		assert_ok!(Dpos::stake_tokens(Origin::signed(alice), validators[0].0, 1000));
	})
}

#[test]
fn test_delegate_tokens_not_validator() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 0u64;
		assert_noop!(Dpos::stake_tokens(Origin::signed(alice), 50, 1000), Error::<Test>::NotValidator);
	})
}

#[test]
fn test_delegate_tokens_validator_sent() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 100u64;
		assert_noop!(Dpos::stake_tokens(Origin::signed(alice), validators[1].0, 1000), Error::<Test>::NotValidator);
	})
}

#[test]
fn test_revoke_tokens_stake_is_zero() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 100u64;
		assert_noop!(Dpos::revoke_stake(Origin::signed(alice), validators[0].0), Error::<Test>::StakeIsZero);
	})
}


