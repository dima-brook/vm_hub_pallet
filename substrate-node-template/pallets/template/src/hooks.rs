
// Macros & imports
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
}

// Logic for the pallet
#[pallet::hooks]
impl <T:Config> Hooks <BlockNumberFor<T>> for Pallet<T>{

}