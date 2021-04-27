
// Macros & imports
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
}

// Placeholder Storage
#[pallet::pallet]
#[pallet::generate_store(pub(super) trait Store)]
pub struct Pallet <T> (PhantomData <T>)

type StorageT = u32
#[pallet::storage]
#[pallet::getter(fn something)]
pub Storage <T:Config> = StorageValue<_, StorageT>


// Functions to be called outside the pallet
#[pallet::call]
impl <T:Config> Pallet <T> {

}