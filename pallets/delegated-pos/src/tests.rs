use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_genesis() {
	sp_io::TestExternalities::new_empty().execute_with(|| {
		let validators = Validators::<Test>::get();
		let top_dogs = ActiveSet::<Test>::get();

		assert!(validators.len() > 0);
		assert!(top_dogs.len() > 0);
		
		//Dpos::stake_tokens(Origin::signed(alice.clone()), );
		//origin: OriginFor<T>,
		//validator: T::AccountId,
		//amount: BalanceOf<T>,
	})
}



//#[test]
//fn delegate_tokens_not_enough_token() {
//	sp_io::TestExternalities::new_empty().execute_with(|| {
//
//	})
//}
//
//#[test]
//fn test_add_validator_double_add() {
//	sp_io::TestExternalities::new_empty().execute_with(|| {
//		let alice = 10u64;
//		assert_ok!(Dpos::add_validator(Origin::signed(alice.clone()), true));
//		assert_noop!(
//			Dpos::add_validator(Origin::signed(alice), true),
//			Error::<Test>::AlreadyValidator
//		);
//	})
//}
//
//
//
//
//#[test]
//fn test_remove_validator() {
//	sp_io::TestExternalities::new_empty().execute_with(|| {
//		let alice = 10u64;
//		Dpos::add_validator(Origin::signed(alice.clone()), true);
//		assert_ok!(Dpos::remove_validator(Origin::signed(alice.clone())));
//		let err = IsValidator::<Test>::try_get(alice.clone());
//		assert_eq!(err, Err(()))
//	})
//}
//
//#[test]
//fn test_remove_validator_double() {
//	sp_io::TestExternalities::new_empty().execute_with(|| {
//		let alice = 10u64;
//		assert_ok!(Dpos::add_validator(Origin::signed(alice.clone()), true));
//		assert_ok!(Dpos::remove_validator(Origin::signed(alice.clone())));
//		assert_noop!(Dpos::remove_validator(Origin::signed(alice)), Error::<Test>::NotValidator);
//	})
//}
