Polkadot Blockchain Academy 22.

Test Template for developing pallets using substrate.

Allow staking to a set of "Validators" held in storage.
Uses a reservable currency to keep track of stakes.
Switches out the active set every block, top third of total staked make it in.

pain is pleasure right?
diamonds are made under extreme pressure, so is spagetti code.


cargo +nightly test -p delegated-pos

Current issues:

Use of a constant value for max validators plagues the pallet

broken functionality on stake_tokens() due to a mismatch of types, ie 
another hardcoded u32, therefore cannot update the stake of each user..

genesis config test fails which means all tests using validators will fail.

chainspec is throwing error.

poorly tested.

poorly documented.

no implementation with aura.