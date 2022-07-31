use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;

#[test]
fn delegate_tokens_not_enough_token(){
	sp_io::TestExternalities::new_empty().execute_with(|| {
		let alice_key = 10u64
		let bob_key = 20u64
		<T>::IsDelegatable.insert(alice_key, ());

		delegated_pos::delegate_tokens(bob_key, alice_key, )


		(origin: OriginFor<T>, delegate: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
        /* for i in 0u64..10u64 {

            Item10::<T>::insert(i, u128::from(i * 100u64));
            Item11::<T>::insert(i, u128::from(i * 100u64));
        }
        // cannot call iter for 10 because it cannot returns the keys
        let all_10: Vec<_> = Item10::<T>::iter_values().collect();
        let all_11: Vec<_> = Item11::<T>::iter().collect();
        println!("{:?}\n{:?}", all_10, all_11);

        assert!(false) */
    })
	unimplemented!();
}

#[test]
fn delegate_tokens_is_not_delegate(){
	unimplemented!();
}

#[test]
fn delegate_tokens_is_not_delegate(){
	unimplemented!();
}

#[test]
fn delegate_tokens_above_minimum(){
	unimplemented!();
}


#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}
