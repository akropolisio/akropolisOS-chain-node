/// Pallet for realworld price oracle requests.
///
/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs
///
/// This module based on project written by Jimmy Chu
/// https://github.com/jimmychu0807/substrate-offchain-pricefetch
///
/// and alpha release example-offchain-worker frame
/// https://github.com/paritytech/substrate/blob/master/frame/example-offchain-worker/src/lib.rs
///
use core::convert::From;
#[cfg(not(feature = "std"))]
#[allow(unused)]
use num_traits::float::FloatCore;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, fail, StorageMap};
// use sp_io::{self, misc::print_utf8 as print_bytes};
use runtime_primitives::traits::{As, Zero};
// We have to import a few things
use rstd::prelude::*;
use system::{self, ensure_signed};

pub const TOKENS_TO_KEEP: usize = 10;

pub const FETCHED_CRYPTOS: [(&[u8], &[u8], &[u8]); 4] = [
    (
        b"DAI",
        b"cryptocompare",
        b"https://min-api.cryptocompare.com/data/price?fsym=DAI&tsyms=USD",
    ),
    (
        b"USDT",
        b"cryptocompare",
        b"https://min-api.cryptocompare.com/data/price?fsym=USDT&tsyms=USD",
    ),
    (
        b"USDC",
        b"cryptocompare",
        b"https://min-api.cryptocompare.com/data/price?fsym=USDC&tsyms=USD",
    ),
    (
        b"cDAI",
        b"coingecko",
        b"https://api.coingecko.com/api/v3/simple/price?ids=cDAI&vs_currencies=USD",
    ),
];

/// The module's configuration trait.
pub trait Trait: timestamp::Trait + balances::Trait + system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where
        Moment = <T as timestamp::Trait>::Moment,
        Balance = <T as balances::Trait>::Balance,
    {
        RecordedPrice(Vec<u8>, Moment, Balance),
        AggregatedPrice(Vec<u8>, Moment, Balance),
    }
);

decl_storage! {
  trait Store for Module<T: Trait> as Oracle {
    /// List of last prices with length of TOKENS_TO_KEEP
    pub TokenPriceHistory get(token_price_history): map Vec<u8> => Vec<T::Balance>;

    /// Tuple of timestamp and average price for token
    pub AggregatedPrices get(aggregated_prices): map Vec<u8> => (T::Moment, T::Balance);
  }
}

// The module's dispatchable functions.
decl_module! {
  /// The module declaration.
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    // Initializing events
    // this is needed only if you are using events in your module
    fn deposit_event<T>() = default;

    pub fn record_price(origin, sym: Vec<u8>, price: T::Balance) -> Result {
        ensure_signed(origin)?;
        Self::_record_price(sym, price)
    }

    pub fn record_aggregated_prices(origin) -> Result {
        ensure_signed(origin)?;
        Self::_record_aggregated_prices()
    }

    fn on_finalize(n : T::BlockNumber){
        let block = <system::Module<T>>::block_number();
        if block % T::BlockNumber::sa(10) == T::BlockNumber::sa(0) {
            let _ = Self::_record_aggregated_prices();
        }
    }
  }
}

impl<T: Trait> Module<T> {
    fn aggregate_prices<'a>(symbol: &'a [u8]) -> T::Balance {
        let token_pricepoints_vec = <TokenPriceHistory<T>>::get(symbol.to_vec());
        let price_sum: T::Balance = token_pricepoints_vec
            .iter()
            .fold(T::Balance::zero(), |mem, price| mem + *price);

        match token_pricepoints_vec.len() {
            0 => T::Balance::sa(0),
            _ => price_sum / T::Balance::sa(token_pricepoints_vec.len() as u64),
        }
    }

    fn _record_price(symbol: Vec<u8>, price: T::Balance) -> Result {
        let now = <timestamp::Module<T>>::get();

        //     //DEBUG
        //     debug::info!("record_price: {:?}, {:?}, {:?}",
        //     core::str::from_utf8(&symbol).map_err(|_| "`symbol` conversion error")?,
        //     core::str::from_utf8(&remote_src).map_err(|_| "`remote_src` conversion error")?,
        //     price
        // );
        <TokenPriceHistory<T>>::mutate(&symbol, |prices| prices.push(price));

        Self::deposit_event(RawEvent::RecordedPrice(symbol, now, price));
        Ok(())
    }
    fn _record_aggregated_prices() -> Result {
        //     //DEBUG
        //     debug::info!("record_aggregated_price_points: {}: {:?}",
        //     core::str::from_utf8(&symbol).map_err(|_| "`symbol` string conversion error")?,
        //     price
        // );
        let result = FETCHED_CRYPTOS
            .iter()
            .map(|t| {
                let symbol = t.0;
                let mut old_vec = <TokenPriceHistory<T>>::get(symbol.to_vec());
                if old_vec.len() == 0 {
                    fail!("Error aggregating price");
                }
                let price = Self::aggregate_prices(symbol);
                let now = <timestamp::Module<T>>::get();
                let price_pt = (now.clone(), price.clone());
                <AggregatedPrices<T>>::insert(symbol.to_vec(), price_pt.clone());

                let new_vec = if old_vec.len() < TOKENS_TO_KEEP {
                    old_vec
                } else {
                    let preserve_from_index =
                        &old_vec.len().checked_sub(TOKENS_TO_KEEP).unwrap_or(9usize);
                    old_vec
                        .drain(preserve_from_index..)
                        .collect::<Vec<T::Balance>>()
                };
                <TokenPriceHistory<T>>::insert(symbol.to_vec(), new_vec.clone());

                Self::deposit_event(RawEvent::AggregatedPrice(
                    symbol.clone().to_vec(),
                    now.clone(),
                    price.clone(),
                ));
                Ok(())
            })
            .fold(
                Err("Error aggregating price"),
                |_, el: Result | match el {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                },
            );


        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    /// tests for this module
    use super::*;
    use frame_support::{impl_outer_dispatch, impl_outer_origin, parameter_types, weights::Weight};
    use sp_core::H256;
    use sp_runtime::{
        testing::{Header, TestXt},
        traits::{BlakeTwo256, IdentityLookup},
        Perbill,
    };
    use std::cell::RefCell;

    pub type Balance = u128;
    pub type BlockNumber = u64;

    thread_local! {
        static EXISTENTIAL_DEPOSIT: RefCell<u128> = RefCell::new(500);
    }

    impl_outer_origin! {
      pub enum Origin for Test {}
    }

    impl_outer_dispatch! {
      pub enum Call for Test where origin: Origin {
        price_fetch::OracleModule,
      }
    }

    pub struct ExistentialDeposit;
    impl Get<u128> for ExistentialDeposit {
        fn get() -> u128 {
            EXISTENTIAL_DEPOSIT.with(|v| *v.borrow())
        }
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: Weight = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    }
    impl system::Trait for Test {
        type Origin = Origin;
        type Call = ();
        type Index = u64;
        type BlockNumber = BlockNumber;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type BlockHashCount = BlockHashCount;
        type MaximumBlockWeight = MaximumBlockWeight;
        type MaximumBlockLength = MaximumBlockLength;
        type AvailableBlockRatio = AvailableBlockRatio;
        type Version = ();
        type ModuleToIndex = ();
        type AccountData = balances::AccountData<u128>;
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type DbWeight = ();
    }

    impl balances::Trait for Test {
        type Balance = Balance;
        type DustRemoval = ();
        type Event = ();
        type ExistentialDeposit = ExistentialDeposit;
        type AccountStore = system::Module<Test>;
    }

    impl timestamp::Trait for Test {
        type Moment = u64;
        type OnTimestampSet = ();
        type MinimumPeriod = ();
    }

    pub type Extrinsic = TestXt<Call, ()>;
    type SubmitPFTransaction =
        system::offchain::TransactionSubmitter<crypto::Public, Call, Extrinsic>;

    pub type OracleModule = Module<Test>;

    parameter_types! {
        pub const BlockFetchPeriod: BlockNumber = 2;
    }

    impl Trait for Test {
        type Event = ();
        type Call = Call;
        type SubmiTransaction = SubmitPFTransaction;
        type BlockFetchPeriod = BlockFetchPeriod;
    }

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    pub fn new_test_ext() -> sp_io::TestExternalities {
        system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .into()
    }
}
