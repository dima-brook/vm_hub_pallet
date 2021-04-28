#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::marker::PhantomData;

    // Main pallet config
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // Main pallet
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet <T> (PhantomData <T>);

    // Calling outside of pallet
    #[pallet::call]
    impl <T:Config> Pallet <T> {

    }

    // Pallet storage
    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub type Storage <T:Config> = StorageValue<_, StorageT>;
    type StorageT = u32;

    // Pallet events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event <T:Config> {
        
    }


    // Pallet logic
    #[pallet::hooks]
    impl <T:Config> Hooks <BlockNumberFor<T>> for Pallet<T>{

    }
}

// TODO
/*#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;*/
