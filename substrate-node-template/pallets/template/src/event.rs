
// Macros & imports
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
}

#[pallet::event]
#[pallet:metadata(T::AccountId = "AccountId")]
#[pallet::generate_deposit(pub (super) fn deposit_event)]
pub enum Event <T:Config> {
    
}