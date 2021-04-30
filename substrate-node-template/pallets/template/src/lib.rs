#![cfg_attr(not(feature = "std"), no_std)]
mod coins;
mod consts;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::coins::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::{convert::*, marker::PhantomData, str, vec::Vec};
    use xp_compiler::{consts::calls::*, deserialize::XpCallJson};

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
    // TODO: Tweak weights
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new account with an initial currency
        #[pallet::weight(50_000_000)]
        pub(super) fn create_account(
            origin: OriginFor<T>,
            language: u8,
            address: u128,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            // Language sanity check
            let lang = SupportedCoin::<T>::try_from(language)?;

            // Compile the script with our common interface
            // create_account(address)
            let addr_s = address.to_string();
            let call = XpCallJson::new(lang.into(), CREATE_ACC, vec![&addr_s]);
            let res = call.compile().map_err(Error::<T>::from)?;

            // Save the script to store so that it can be retrieved
            <AccountsStore<T>>::insert(&sender, res.as_bytes());

            // Trigger event
            Self::deposit_event(Event::AccountCreation(sender, language));

            Ok(().into())
        }

        /// Transfer funds from your account
        #[pallet::weight(70_000_000)]
        pub(super) fn transfer_funds(
            origin: OriginFor<T>,
            language: u8,
            receiver: u128,
            amount: u64,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let lang = SupportedCoin::<T>::try_from(language)?;

            // transfer_amount(receiver, amount)
            let addr_s = receiver.to_string();
            let am_s = amount.to_string();
            let call = XpCallJson::new(lang.into(), TRANSFER_AMOUNT, vec![&addr_s, &am_s]);
            let res = call.compile().map_err(Error::<T>::from)?;

            <AccountsStore<T>>::insert(&sender, res.as_bytes());
            Self::deposit_event(Event::TransferFund(sender, language, receiver, amount));
            Ok(().into())
        }
    }

    type RawScriptData = Vec<u8>;
    // Pallet storage
    #[pallet::storage]
    #[pallet::getter(fn accounts)]
    pub(super) type AccountsStore<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, RawScriptData>;

    // Pallet events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        AccountCreation(T::AccountId, u8), // AccountId, Currency Type
        TransferFund(T::AccountId, u8, u128, u64), // AccountId, Currency Type, Receiver, Ammount
    }

    // Errors
    #[derive(PartialEq)]
    #[pallet::error]
    pub enum Error<T> {
        /// Unsupported currency
        CoinUnsupported,
        /// Empty Name during creation
        NameEmpty,
        /// Invalid Script Arguments
        InvalidArgs,
        /// Unsupported Language
        UnsupportedLang,
        /// Unsupported Function
        UnsupportedCall,
    }

    // Convert xp_compiler::errors::CompileError to Error
    impl<T: Config> From<xp_compiler::errors::CompileError> for Error<T> {
        fn from(val: xp_compiler::errors::CompileError) -> Self {
            use xp_compiler::errors::CompileError;

            match val {
                CompileError::UnsupportedLang(_) => Self::UnsupportedLang,
                CompileError::UnsupportedCall(_) => Self::UnsupportedCall,
                CompileError::InvalidArgs => Self::InvalidArgs,
            }
        }
    }

    // Pallet hooks
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub initial_accounts: Vec<(T::AccountId, RawScriptData)>,
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
