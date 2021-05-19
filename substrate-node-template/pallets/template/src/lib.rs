#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
mod consts;

#[frame_support::pallet]
pub mod pallet {
    use super::consts::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::{marker::PhantomData, vec::Vec};

    // Main pallet config
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // Main pallet
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    // Calling outside of pallet(Extrensics)
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new account with an initial currency
        // TODO: Multiple currency
        // TODO: Tweak Weight
        // TODO: Implement account creation
        #[pallet::weight(50_000_000)]
        pub(super) fn create_account(
            origin: OriginFor<T>,
            name: Vec<u8>,
            currency: u8,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            match currency {
                ETH_CURRENCY_CODE => {
                    Self::deposit_event(Event::AccountCreation(sender.clone(), currency))
                }
                _ => return Err(Error::<T>::CoinUnsupported.into()),
            }

            <AccountsStore<T>>::insert(&sender, (name, currency));
            Ok(().into())
        }

        /// Transfer funds from your account
        // TODO: Tweak weight
        // TODO: implement actual logic
        #[pallet::weight(70_000_000)]
        pub(super) fn transfer_funds(
            origin: OriginFor<T>,
            currency: u8,
            amount: u8,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            match currency {
                ETH_CURRENCY_CODE => {
                    // TODO: Implement logic
                }
                _ => return Err(Error::<T>::CoinUnsupported.into()),
            }

            Self::deposit_event(Event::TransferFund(sender, currency, amount));
            Ok(().into())
        }
    }

    type AccountInfo = (Vec<u8>, u8);
    // Pallet storage
    #[pallet::storage]
    #[pallet::getter(fn accounts)]
    pub(super) type AccountsStore<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, AccountInfo>;

    // Pallet events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        AccountCreation(T::AccountId, u8),  // AccountId, Currency Type
        TransferFund(T::AccountId, u8, u8), // AccountId, Currency Type, Ammount
    }

    // Errors
    #[pallet::error]
    pub enum Error<T> {
        /// Unsupported currency
        CoinUnsupported,
    }

    // Pallet hooks
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub initial_accounts: Vec<(T::AccountId, AccountInfo)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                initial_accounts: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for acc in &self.initial_accounts {
                AccountsStore::<T>::insert(&acc.0, &acc.1)
            }
        }
    }
}

#[cfg(test)]
mod tests;
