// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
use frame_support::sp_runtime::traits::Zero;
use frame_support::traits::fungible;
use frame_support::traits::fungibles;
pub use pallet::*;
mod liquidity_pools;

// Define type aliases for easier access
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type AssetIdOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
    <T as frame_system::Config>::AccountId,
>>::AssetId;

pub type BalanceOf<T> = <<T as Config>::NativeBalance as fungible::Inspect<
    <T as frame_system::Config>::AccountId,
>>::Balance;

pub type AssetBalanceOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
    <T as frame_system::Config>::AccountId,
>>::Balance;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
#[cfg(test)]
mod tests;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type NativeBalance: fungible::Inspect<Self::AccountId>
            + fungible::Mutate<Self::AccountId>
            + fungible::hold::Inspect<Self::AccountId>
            + fungible::hold::Mutate<Self::AccountId>
            + fungible::freeze::Inspect<Self::AccountId>
            + fungible::freeze::Mutate<Self::AccountId>;
        type Fungibles: fungibles::Inspect<Self::AccountId>
            + fungibles::Mutate<Self::AccountId>
            + fungibles::hold::Inspect<Self::AccountId>
            + fungibles::hold::Mutate<Self::AccountId>
            + fungibles::freeze::Inspect<Self::AccountId>
            + fungibles::freeze::Mutate<Self::AccountId>
            + storage::types::EncodeLikeTuple<Self::AccountId>;
    }

    /// A storage item for this pallet.
    #[pallet::storage]
    pub type LiquidityPoolBalance<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// A storage map for this pallet.
    #[pallet::storage]
    pub type LiquidityPools<T: Config> =
        StorageMap<_, Blake2_128Concat, AssetIdOf<T>, (AssetIdOf<T>, AssetIdOf<T>)>;

    /// Events that functions in this pallet can emit.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        LiquidityPoolCreated(AccountIdOf<T>, (AssetIdOf<T>, AssetIdOf<T>)),
    }

    /// Errors that can be returned by this pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// Insufficient liquidity available in the pool.
        InsufficientLiquidity,

        /// Insufficient reserves available in the pool for the requested operation.
        InsufficientReserves,

        /// Adding liquidity overflows.
        LiquidityOverflow,

        /// Adding reserves overflows.
        ReserveOverflow,

        InvalidAssetIn,
        InvalidAssetOut,
        InsufficientAmountOut,
        ArithmeticOverflow,
        DivisionByZero,
        LiquidityPoolAlreadyExists,
    }

    /// The pallet's dispatchable functions ([`Call`]s).
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub fn create_liquidity_pool(
            origin: OriginFor<T>,
            asset_a: AssetIdOf<T>,
            asset_b: AssetIdOf<T>,
            liquidity_token: AssetIdOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let trading_pair: (<<T as Config>::Fungibles as Inspect<<T as Config>::AccountId>>::AssetId, <<T as Config>::Fungibles as Inspect<<T as Config>::AccountId>>::AssetId) = (asset_a, asset_b);
            ensure!(
                !LiquidityPools::<T>::contains_key(trading_pair),
                Error::<T>::LiquidityPoolAlreadyExists
            );

            let liquidity_pool = crate::liquidity_pools::LiquidityPool {
                assets: trading_pair,
                reserves: (Zero::zero(), Zero::zero()),
                total_liquidity: Zero::zero(),
                liquidity_token,
            };

            LiquidityPools::<T>::insert(trading_pair, liquidity_pool);

            Self::deposit_event(Event::LiquidityPoolCreated(who, trading_pair));

            Ok(())
        }
    }

    /// The pallet's internal functions.
    impl<T: Config> Pallet<T> {
        /* Internally Callable Functions Go Here */
    }
}
