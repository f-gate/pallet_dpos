use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::Hooks};
#[test]
fn test_genesis() {
	new_test_ext().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());


		let validators = Validators::<Test>::get();
		let top_dogs = ActiveSet::<Test>::get();
		assert!(validators.len() > 0);
		assert!(top_dogs.len() > 0);
	})
}

#[test]
fn test_top_dogs_are_legit_good_boys() {
	new_test_ext().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 0u64;

		for i in (100..140) {
			assert_ok!(Dpos::stake_tokens(Origin::signed(alice), validators[i].0, 100));
		}

		assert_ok!(Dpos::stake_tokens(Origin::signed(alice), validators[141].0, 100000));
		assert_ok!(Dpos::stake_tokens(Origin::signed(alice), validators[142].0, 900000));
		assert_ok!(Dpos::stake_tokens(Origin::signed(alice), validators[143].0, 200000));

		let mut top_dogs = ActiveSet::<Test>::get();
		dbg!(&top_dogs);
		assert!(top_dogs.contains(&141));
		assert!(top_dogs.contains(&142));
		assert!(top_dogs.contains(&143));

	})
}

#[test]
fn test_delegate_tokens_not_enough_token() {
	new_test_ext().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 1000u64;
		assert_noop!(Dpos::stake_tokens(Origin::signed(alice), validators[0].0, 100000000), Error::<Test>::NotEnoughFunds);
	})
}

//type MinimumStake = ConstU64<100>;
#[test]
fn test_delegate_tokens_below_minimum() {
	new_test_ext().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 0u64;
		assert_ok!(Dpos::stake_tokens(Origin::signed(alice), validators[0].0, 50));
	})
}


#[test]
fn test_delegate_tokens_enough_tokens() {
	new_test_ext().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 0u64;
		assert_ok!(Dpos::stake_tokens(Origin::signed(alice), validators[0].0, 100000));
	})
}

#[test]
fn test_delegate_tokens_not_validator() {
	new_test_ext().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 0u64;
		assert_noop!(Dpos::stake_tokens(Origin::signed(alice), 50, 1000), Error::<Test>::NotValidator);
	})
}

#[test]
fn test_delegate_tokens_validator_sent() {
	new_test_ext().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 100u64;
		assert_noop!(Dpos::stake_tokens(Origin::signed(validators[1].0), validators[0].0, 1000), Error::<Test>::CannotStakeAsValidator);
	})
}

#[test]
fn test_revoke_tokens_stake_is_zero() {
	new_test_ext().execute_with(|| {
		System::set_block_number(4u64);
		System::on_initialize(System::block_number());

		let validators = Validators::<Test>::get();
		let alice = 100u64;
		assert_noop!(Dpos::revoke_stake(Origin::signed(alice), validators[0].0), Error::<Test>::StakeIsZero);
	})
}


