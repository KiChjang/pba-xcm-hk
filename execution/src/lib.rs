// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Exercises for the XCVM lecture.
//!
//! Run tests with `RUST_LOG=xcm cargo test -- --nocapture` to get all the execution logs.
//!
//! You're going to want to look at the available instructions for these exercises:
//! https://github.com/paritytech/xcm-format#5-the-xcvm-instruction-set
//! https://github.com/paritytech/polkadot-sdk/blob/481165d92297d7dfd5eaf9c7f442441761fc0a12/polkadot/xcm/src/v3/mod.rs#L430

#[cfg(test)]
mod tests {
	use codec::Encode;
	use frame_support::{assert_ok, weights::Weight};
	use test_log::test;
	use xcm::{latest::prelude::*, VersionedXcm};
	use xcm_simulator_pba::{
		para_account_id, parachain, parachain_xcm_executed_successfully, relay_chain, MockNet,
		ParaA, ParachainPalletBalances, ParachainPalletXcm, Relay, RelayChainPalletXcm, TestExt,
		ALICE, BOB, INITIAL_BALANCE,
	};

	/// Example
	///
	/// In this example, we want to showcase how to execute XCM programs.
	/// To do this, we use pallet-xcm's `execute` extrinsic.
	#[test]
	fn execute_withdraw_asset() {
		// Execute a new testnet
		MockNet::reset();

		// Asset MultiLocation to use:
		// Parachain Pallet Balances tied to: MultiLocation { parents: 1, interior: Here }
		let withdraw_amount = 100u128;
		let asset_location = MultiLocation { parents: 1, interior: Here }; // Could've done MultiLocation::parent() as well.

		ParaA::execute_with(|| {
			// The message we will execute.
			// This is using the XCM builder API, which lets you chain function calls and then call `.build()` to get your XCM.
			// The other way is to just call the XCM struct constructor and put the instructions in.
			let message: Xcm<parachain::RuntimeCall> = Xcm::builder_unsafe()
				.withdraw_asset((asset_location, withdraw_amount).into())
				.build();

			// Execute the message
			// First parameter is origin
			// Second parameter is versioned message (we are using V3)
			// Third parameter is the max weight (just make sure it's enough)
			assert_ok!(ParachainPalletXcm::execute(
				parachain::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::V3(message.into())),
				Weight::from_parts(100_000_000_000, 100_000_000_000)
			));

			// Assert XCM message executed successfully
			assert!(parachain_xcm_executed_successfully());

			// Ensure the balance has been withdrawn
			assert_eq!(
				ParachainPalletBalances::free_balance(ALICE),
				INITIAL_BALANCE - withdraw_amount
			);
		});
	}

	/// Background
	/// Remember that XCM uses the same Weight concept as FRAME.
	/// Weight is two-dimensional.
	/// The first dimension is time. It's measured in picoseconds (1e12 picoseconds make a second).
	/// The second dimension is space (PoV size). It's measured in kibibytes (https://en.wikipedia.org/wiki/Byte#Units_based_on_powers_of_2).
	///
	/// Configuration
	/// In parachain.rs inside xcm-simulator-pba, the XCM executor is configured.
	/// No need to understand all the configuration items there.
	/// Pay attention to the `TokensPerSecond` type, it's a tuple that defines:
	/// - The asset to buy weight
	/// - The cost of one second of execution time (Weight's first dimension)
	/// - The cost of one MiB (mibibyte) of PoV size (Weight's second dimension)
	/// To buy some weight, you have to pay for execution time + PoV size.
	///
	/// Task
	/// Create a `BuyExecution` Instruction that buys 1 second of execution time and 1 MiB of PoV size.
	/// Make sure you use `Limited` as the weight limit (https://github.com/paritytech/polkadot-sdk/blob/481165d92297d7dfd5eaf9c7f442441761fc0a12/polkadot/xcm/xcm-executor/src/lib.rs#L835).
	/// 1. Who is buying the execution?
	/// 2. How to verify we executed this instruction correctly? Does it work on its own?
	/// 3. What happens if you pay less? What happens if you pay more?
	#[test]
	fn execute_buy_execution() {
		MockNet::reset();

		let weight = Weight::from_parts(1_000_000_000_000u64, 1024 * 1024);

		// TODO: Modify message
		let message: Xcm<()> = Xcm::builder_unsafe().build();

		ParaA::execute_with(|| {
			assert_ok!(ParachainPalletXcm::execute(
				parachain::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::V3(message.into())),
				Weight::from_parts(100_000_000_000, 100_000_000_000)
			));

			// TODO: Add assertions
		});
	}

	#[test]
	fn execute_send_funds_to_bob() {
		MockNet::reset();

		// Task
		// Send 100 tokens from Alice to Bob from a parachain locally and verify it.
		// 1.) Where to send it from?
		// 2.) how to verify Bob received the funds?

		let withdraw_amount = 100u128;

		// TODO: Modify message
		let message: Xcm<()> = Xcm::builder_unsafe().build();

		ParaA::execute_with(|| {
			assert_ok!(ParachainPalletXcm::execute(
				parachain::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::V3(message.into())),
				Weight::from_parts(100_000_000_000, 100_000_000_000)
			));

			// TODO: Add assertions
		});
	}

	#[test]
	fn execute_transact_message() {
		MockNet::reset();

		// Task
		// Create a call which we can dispatch locally
		// 1.) What kinds of calls do we have to choose from?
		//      (Hint system pallet might be useful here but any call can do (Which we can verify))
		// Hint for encoding: https://github.com/paritytech/substrate/blob/master/frame/democracy/src/tests.rs#L223
		// or https://github.com/paritytech/xcm-docs/blob/0e4a5fb58b9421288f4fc5e6def9c5d8b6b3f065/examples/src/3_transact/mod.rs#L17
		// 2.) How can we verify the call was dispatched correctly? Hint: events could be a way :)

		// TODO: Create encoded call

		// TODO: Modify message
		let message: Xcm<parachain::RuntimeCall> = Xcm::builder_unsafe().build();

		ParaA::execute_with(|| {
			// TODO: Execute message and add assertions
		});
	}

	#[test]
	fn execute_origin_mutating_instruction() {
		MockNet::reset();

		// Task
		// 1.) Modify the origin register with the DescendOrigin instruction
		// 2.) Clear the origin of the register
		// 3.) To evaluate: Does it work if you append a WithdrawAsset after origin mutations?

		ParaA::execute_with(|| {
			// Insert Here the appropriate code to execute the XCM message asked for.
		});
	}

	#[test]
	fn execute_transact_on_own_dispatchable() {
		MockNet::reset();

		// Task
		// 1.) Add your own dispatchable in the `parachain` module via your own custom pallet or in
		// an existing pallet
		// 2.) Construct the call for your new dispatchable
		// 3.) Send a Transact instruction for your new dispatchable
		// 4.) Verify the call was dispatched

		ParaA::execute_with(|| {
			// Insert Here the appropriate code to execute the XCM message asked for.
		});
	}

	#[test]
	fn execute_initiate_teleport_to_para_a() {
		/* ------------------------------------------------------------------------- */
		// In this section we do some initialization for our XCM exercise
		// 1.) Create a new Test net scenario
		MockNet::reset();

		// 2.) Declare an amount to withdraw and teleport
		let withdraw_amount = 100;

		// 3.) Create 2-tuple with item 1 being the junction/s to send from and item 2 the
		// withdraw_amount let withdraw_assets_from = $CREATE_TUPLE_HERE;

		/* ------------------------------------------------------------------------ */
		// In this next section we focus on constructing the instructions necessary create our XCM
		// message 1.) Takes some assets and place in the holding register.
		// First we need to use an Instruction which can withdraw assets and place them in the
		// holding register let instruction_1: Instruction<parachain::RuntimeCall> =
		// $PLACE_CORRECT_INSTRUCTION_HERE

		// 2.) Inform a destination chain that we are teleporting the previously withdrawn assets
		// Second we need to use an Instruction which informs the other chain about the assets being
		// teleported let instruction_2: Instruction<parachain::RuntimeCall> =
		// $PLACE_CORRECT_INSTRUCTION_HERE

		// 4.) XCM Message for parachain
		// Create the XCM message for the given instructions above
		// let message = $INSERT_CONSTRUCTED_XCM_MESSAGE_HERE

		Relay::execute_with(|| {
			// 5.) Execute send_xcm from the XCM pallet
			//     assert_ok!(
			//         // RelayChainPalletXcm::execute(
			//            /* $INSERT_CORRECT_PARAMS_TO_EXECUTE_XCM */
			//         // )
			//     );
		});

		ParaA::execute_with(|| {
			// let expected_message_received = $INSERT_EXPECTED_MESSAGE_RECEIVED_BY_PARA_A
			// assert_eq!(parachain::MsgQueue::received_dmp(), expected_message_received);
		})
	}

	#[test]
	fn execute_initiate_reserve_withdraw_to_para_a() {
		/* ------------------------------------------------------------------------- */
		// In this section we do some initialization for our XCM exercise
		// 1.) Create a new Test net scenario
		MockNet::reset();

		// 2.) Declare an amount to withdraw and teleport
		let withdraw_amount = 100;

		// 3.) Create 2-tuple with item 1 being the junction/s to send from and item 2 the
		// withdraw_amount let withdraw_assets_from = $CREATE_TUPLE_HERE;

		/* ------------------------------------------------------------------------ */
		// In this next section we focus on constructing the instructions necessary create our XCM
		// message 1.) Takes some assets and place in the holding register.
		// First we need to use an Instruction which can withdraw assets and place them in the
		// holding register let instruction_1: Instruction<parachain::RuntimeCall> =
		// $PLACE_CORRECT_INSTRUCTION_HERE

		// 2.) Inform a destination chain that we are placing the previously withdrawn reserve
		// assets Second we need to use an Instruction which informs the other chain about the
		// assets being teleported let instruction_2: Instruction<parachain::RuntimeCall> =
		// $PLACE_CORRECT_INSTRUCTION_HERE

		// 4.) XCM Message for parachain
		// Create the XCM message for the given instructions above
		// let message = $INSERT_CONSTRUCTED_XCM_MESSAGE_HERE

		Relay::execute_with(|| {
			// 5.) Execute send_xcm from the XCM pallet
			//     assert_ok!(
			//         // RelayChainPalletXcm::execute(
			//            /* $INSERT_CORRECT_PARAMS_TO_EXECUTE_XCM */
			//         // )
			//     );
		});

		ParaA::execute_with(|| {
			// let expected_message_received = $INSERT_EXPECTED_MESSAGE_RECEIVED_BY_PARA_A
			// assert_eq!(parachain::MsgQueue::received_dmp(), expected_message_received);
		})
	}

	#[test]
	fn send_ump_withdraw_deposit_alice() {
		/* ------------------------------------------------------------------------- */
		// In this section we do some initialization for our XCM exercise
		// 1.) Create a new Test net scenario
		MockNet::reset();

		// 2.) Declare an amount to send to ALICE's account on the relaychain
		let withdraw_amount = 100;

		// 3.) Create 2-tuple with item 1 being the junction/s to send from and item 2 the
		// withdraw_amount let withdraw_assets_from = $CREATE_TUPLE_HERE;

		/* ------------------------------------------------------------------------ */
		// In this next section we focus on constructing the instructions necessary create our XCM
		// message 1.) Takes some assets and place in the holding register.
		// First we need to use an Instruction which can withdraw assets and place them in the
		// holding register let instruction_1: Instruction<parachain::RuntimeCall> =
		// $PLACE_CORRECT_INSTRUCTION_HERE HINT: when a message is sent from ParaA to the relay, the
		// para A multilocation origin is converted to:
		// let para_a_account = xcm_simulator_pba::para_account_id(Para_A_para_id)

		// 2.) Purchase execution for this message to be executed appropriately on the destination
		// chain Second we need to use an Instruction which can obtain execution some execution
		// let instruction_2: Instruction<parachain::RuntimeCall> = $PLACE_CORRECT_INSTRUCTION_HERE

		// 3.) Drain assets from the holding register and specify to whome receives this asset
		// Lastly we need an instruction which can take assets from the holding register and give
		// them to ALICE let instruction_3: Instruction<parachain::RuntimeCall> =
		// $PLACE_CORRECT_INSTRUCTION_HERE

		// 4.) XCM Message for parachain
		// Create the XCM message for the given instructions above
		// let message = $INSERT_CONSTRUCTED_XCM_MESSAGE_HERE

		ParaA::execute_with(|| {
			// 5.) Execute send_xcm from the XCM pallet
			//     assert_ok!(
			//         // ParachainPalletXcm::send_xcm(
			/* $INSERT_CORRECT_PARAMS_TO_SEND_XCM */
			//         // )
			//     );
		});

		Relay::execute_with(|| {
			// TODO: Uncomment assertions
			// assert_eq!(
			// 	relay_chain::Balances::free_balance(ALICE),
			// 	INITIAL_BALANCE + withdraw_amount
			// );
			// assert_eq!(
			// 	relay_chain::Balances::free_balance(para_account_id(1)),
			// 	INITIAL_BALANCE - withdraw_amount
			// );
		})
	}

	#[test]
	fn withdraw_and_query_holding_xcmp() {
		/* ----------------------------------------------------- */
		// Now do it on your own!!!
		// 1.) Construct XCM Instructions
		// 2.) Send correct XCM Message from Para_A to Relay
		// 3.) Verify correct message was received and state was updated correctly on destination
		// 4.) Verify a response was received by the request originating chain

		MockNet::reset();

		let withdraw_amount = 100;

		/* Fill in here */
	}

	#[test]
	fn test_automatic_versioning_on_runtime_upgrade_with_relay() {
		MockNet::reset();

		ParaA::execute_with(|| {
			// Set version 1 to parachain
			// HINT: you can use parachain::XcmVersioner
		});

		Relay::execute_with(|| {
			// This sets the default version in the relay to 2, for not known destinations
			assert_ok!(RelayChainPalletXcm::force_default_xcm_version(
				relay_chain::RuntimeOrigin::root(),
				Some(2)
			));

			// Wrap version, which sets VersionedStorage
			// This is necessary because the mock router does not use wrap_version, but
			// this is not necessary in prod
			// This triggers note_unknown_version in pallet-xcm
			// And also version negotiation
			assert_ok!(<RelayChainPalletXcm as xcm::WrapVersion>::wrap_version(
				&Parachain(1).into(),
				Xcm::<()>(vec![])
			));

			// Let's advance the relay. This should trigger the subscription message
			relay_chain::relay_roll_to(2);

			// queries should have been updated
			// we received a query back from the para indicating of the version
			assert!(RelayChainPalletXcm::query(0).is_some());
		});

		/*  FILL in class
		let expected_supported_version: relay_chain::RuntimeEvent =
			pallet_xcm::Event::SupportedVersionChanged(
				// parachain multilocation as seen by the relay,
				// version that the relay received?
			)
			.into();

		Relay::execute_with(|| {
			// Assert that the events vector contains the version change
			assert!(relay_chain::relay_events().contains(&expected_supported_version));
		});*/

		/*  FILL in class
		let expected_version_notified: parachain::RuntimeEvent =
			pallet_xcm::Event::VersionChangeNotified(
				// Relay as seen by the para,
				// version that the para received?
			)
			.into();*/

		// ParaA changes version to 2, and calls on_runtime_upgrade. This should notify the targets
		// of the new version change
		ParaA::execute_with(|| {
			// Set version 2
			// Do runtime upgrade
			parachain::on_runtime_upgrade();
			// Initialize block, to call on_initialize and notify targets
			parachain::para_roll_to(2);
			// Expect the event in the parachain
			// assert!(parachain::para_events().contains(&expected_version_notified));
		});

		/*  FILL in class
		// This event should have been seen in the relay
		let expected_supported_version_after: relay_chain::RuntimeEvent =
			pallet_xcm::Event::SupportedVersionChanged(
				// Para multilocation as seen by the relay,
				// version that the relay received?
			)
			.into();

		Relay::execute_with(|| {
			// Assert that the events vector contains the new version change
			assert!(relay_chain::relay_events().contains(&expected_supported_version_after));
		});*/
	}
}
