// Copyright Parity Technologies (UK) Ltd.
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

//!
//! # References
//!
//! This crate was auto generated by FRAMY CLI <https://crates.io/crates/framy>.  
//! Please report bugs to <https://github.com/ggwpez/framy/issues>.

//! # ActivityRuntimeXcm
//!
//! proposed exercise for lesson 7.3
//!
//! ## Overview

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod benchmarking;
mod mock;
mod tests;

pub mod weights;

use sp_runtime::traits::TryConvert;

use xcm::prelude::*;

/// Whatever type is needed for claiming assets
// TODO: Modify as needed
pub struct Ticket;

type Location = MultiLocation;
type Assets = MultiAssets;

pub trait WeightInfo {
	fn trap_assets() -> Weight;
	fn claim_assets() -> Weight;
}

impl WeightInfo for () {
	fn trap_assets() -> Weight {
		Weight::zero()
	}

	fn claim_assets() -> Weight {
		Weight::zero()
	}
}

pub struct TestWeightInfo;
impl WeightInfo for TestWeightInfo {
	fn trap_assets() -> Weight {
		Weight::from_parts(1_000_000, 1_000_000)
	}

	fn claim_assets() -> Weight {
		Weight::from_parts(1_000_000, 1_000_000)
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type of the runtime.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Something to execute an XCM message.
		type XcmExecutor: ExecuteXcm<<Self as frame_system::Config>::RuntimeCall>;

		type LocationConverter: TryConvert<<Self as frame_system::Config>::RuntimeOrigin, Location>;

		/// Weight information for all calls of this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The origin could not be converted to an XCM location
		BadOrigin,
	}

	#[pallet::call(weight(<T as Config>::WeightInfo))]
	impl<T: Config> Pallet<T> {
		/* ------------------------------------------------------------------------- */
		// Proposed activity:
		// Modify the following extrinsics to
		// 1.) Create a dispatchable that creates an XCM. Such XCM results in some funds being
		// trapped after the message exeecution.
		//
		// 2.) Create a dispatchable that craetes an XCM which claims some trapped funds as a
		// result of the execution of the message.
		//
		// Usually trapping funds is not a desired outcome.
		// In the first XCM, what would you do to avoid trapping the assets?

		/// Composes and executes an XCM that traps some funds
		#[pallet::call_index(0)]
		pub fn trap_assets(origin: OriginFor<T>, _assets: Assets) -> DispatchResult {
			// TODO: Build the XCM
			let message = Xcm(vec![
				// Instructions
			]);

			Self::do_execute_xcm(origin, message)?;

			Ok(())
		}

		/// Composes and executes and XCM that claims some trapped funds
		#[pallet::call_index(1)]
		pub fn claim_assets(
			origin: OriginFor<T>,
			_assets: Assets,
			_ticket: Location,
		) -> DispatchResult {
			// TODO: Build the XCM
			let message = Xcm(vec![
				// Instructions
			]);

			Self::do_execute_xcm(origin, message)?;

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Helper function to execute an XCM
		/// It will work if the pallet is configured correctly
		pub(crate) fn do_execute_xcm(
			origin: OriginFor<T>,
			message: Xcm<<T as frame_system::Config>::RuntimeCall>,
		) -> Result<(), Error<T>> {
			let mut hash = message.using_encoded(sp_io::hashing::blake2_256);
			let origin_location =
				T::LocationConverter::try_convert(origin).map_err(|_| Error::BadOrigin)?;

			T::XcmExecutor::prepare_and_execute(
				origin_location,
				message,
				&mut hash,
				// Dummy weights - focus is on coding extrinsics that build XCMs
				Weight::from_parts(2_000_000_000_000, 2_000_000_000_000),
				Weight::zero(),
			);

			Ok(())
		}
	}
}