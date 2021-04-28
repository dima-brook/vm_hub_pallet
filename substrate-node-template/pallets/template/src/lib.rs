#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
mod consts;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::{
        vec::Vec,
        marker::PhantomData
    };
    use super::consts::*;

    // Main pallet config
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // Main pallet
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T> (PhantomData<T>);

    // Calling outside of pallet(Extrensics)
    #[pallet::call]
    impl<T:Config> Pallet <T> {
        /// Create a new account with an initial currency
        // TODO: Multiple currency
        // TODO: Tweak Weight
        // TODO: Implement account creation
        #[pallet::weight(50_000_000)]
        pub(super) fn create_account(origin: OriginFor<T>, name: Vec<u8>, currency: u8) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            match currency {
                ETH_CURRENCY_CODE => {
                    Self::deposit_event(Event::AccountCreation(sender.clone(), currency))
                },
                _ => return Err(Error::<T>::CoinUnsupported.into())
            }

            <AccountsStore<T>>::insert(&sender, (name, currency));
            Ok(().into())
        }

        /// Transfer funds from your account
        // TODO: Tweak weight
        // TODO: implement actual logic
        #[pallet::weight(70_000_000)]
        pub(super) fn transfer_funds(origin: OriginFor<T>, currency: u8, amount: u8) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            match currency {
                ETH_CURRENCY_CODE => {
                    // TODO: Implement logic
                }
                _ => return Err(Error::<T>::CoinUnsupported.into())
            }

            Self::deposit_event(Event::TransferFund(sender.clone(), currency, amount)); 
            Ok(().into())
        }
    }

    // Pallet storage
    #[pallet::storage]
    #[pallet::getter(fn accounts)]
    pub(super) type AccountsStore <T:Config> = StorageMap<_, Twox64Concat, T::AccountId, (Vec<u8>, u8)>;

    // Pallet events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T:Config> {
        AccountCreation(T::AccountId, u8), // AccountId, Currency Type
        TransferFund(T::AccountId, u8, u8) // AccountId, Currency Type, Ammount
    }

    // Errors
	#[pallet::error]
	pub enum Error<T> {
        /// Unsupported currency
        CoinUnsupported,
	}

    // Pallet hooks
    #[pallet::hooks]
    impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T>{}
}

// TODO
/*#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;*/
