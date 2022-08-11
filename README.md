Polkadot Blockchain Academy 22.

Test Template for developing pallets using substrate.

Allow staking to a set of "Validators" held in storage.
Uses a reservable currency to keep track of stakes.
Switches out the active set every block, top third of total staked make it in.

cargo +nightly test -p delegated-pos

Current issues:

Use of a constant value for max validators plagues the pallet
no implementation with aura.


