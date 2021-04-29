#![cfg_attr(not(feature = "std"), no_std)]
mod coins;
mod consts;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::coins::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::{convert::*, marker::PhantomData, vec::Vec};

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

            let raw_info = (name, currency);
            AccountInfo::<T>::try_from(raw_info.clone())?;

            <AccountsStore<T>>::insert(&sender, raw_info);
            Ok(().into())
        }

        /// Transfer funds from your account
        // TODO: Tweak weight
        // TODO: implement actual logic
        #[pallet::weight(70_000_000)]
        pub(super) fn transfer_funds(
            origin: OriginFor<T>,
            currency: u8,
            amount: u128,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            SupportedCoin::<T>::try_from(currency)?;

            Self::deposit_event(Event::TransferFund(sender, currency, amount));
            Ok(().into())
        }
    }

    type RawCurrencyType = u8;
    type RawAccountInfo = (Vec<u8>, RawCurrencyType);
    // Pallet storage
    #[pallet::storage]
    #[pallet::getter(fn accounts)]
    pub(super) type AccountsStore<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, RawAccountInfo>;

    // Pallet events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        AccountCreation(T::AccountId, RawAccountInfo), // AccountId, Currency Type
        TransferFund(T::AccountId, RawCurrencyType, u128), // AccountId, Currency Type, Ammount
    }

    // Errors
    #[derive(PartialEq)]
    #[pallet::error]
    pub enum Error<T> {
        /// Unsupported currency
        CoinUnsupported,
        /// Empty Name during creation
        NameEmpty,
    }

    // Pallet hooks
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub initial_accounts: Vec<(T::AccountId, RawAccountInfo)>,
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
