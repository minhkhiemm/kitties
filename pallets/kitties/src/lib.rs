#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		sp_runtime::traits::{Hash, Zero},
		dispatch::{DispatchResultWithPostInfo, DispatchResult},
		traits::{Currency, ExistenceRequirement, Randomness},
		pallet_prelude::*
	};
	use frame_system::pallet_prelude::*;

	// TODO Part II Struct for holding kitty information

	// TODO Part II Enum and implementation to handle Gender type in Kitty struct

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::error]
	pub enum Error<T> {
		// TODO part II
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// TODO Part III
	}

	// ACTION: Storage item to keep a count of all existing Kitties.
	#[pallet::storage]
	#[pallet::getter(fn kitty_cnt)]
	/// Keeps track of the number of Kitties in existence
	pub(super) type KittyCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

	// TODO Part II: Remaining storage items.

	// TODO Part III: Our pallet's genesis configuration.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO Part III: create kitty

		// TODO Part III: set_price

		// TODO Part III: transfer

		// TODO Part III: buy_kitty

		// TODO Part III: breed_kitty
	}

	// TODO Part II: helper function for Kitty struct

	impl<T: Config> Pallet<T> {
		// TODO Part III: helper functions for dispatchable functions

		// TODO: increment_nonce, random_hash, mint, transfer_from
	}
}
