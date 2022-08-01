use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn delegate_tokens_not_enough_token() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		let alice = 0u32;
		let bob = 3u32;
		let _ = Dpos::add_validator(Origin::signed(1), true);

		//todo setup test

		//<Test as crate::Config>::Currency::func_name();
	})
}

#[test]
fn test_add_validator_double_add() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		let alice = 10u64;
		assert_ok!(Dpos::add_validator(Origin::signed(alice.clone()), true));
		assert_noop!(
			Dpos::add_validator(Origin::signed(alice), true),
			Error::<Test>::AlreadyValidator
		);
	})
}

#[test]
fn test_remove_validator() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		let alice = 10u64;
		Dpos::add_validator(Origin::signed(alice.clone()), true);
		assert_ok!(Dpos::remove_validator(Origin::signed(alice.clone())));
		let err = IsValidator::<Test>::try_get(alice.clone());
		assert_eq!(err, Err(()))
	})
}

#[test]
fn test_remove_validator_double() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		let alice = 10u64;
		assert_ok!(Dpos::add_validator(Origin::signed(alice.clone()), true));
		assert_ok!(Dpos::remove_validator(Origin::signed(alice.clone())));
		assert_noop!(Dpos::remove_validator(Origin::signed(alice)), Error::<Test>::NotValidator);
	})
}
