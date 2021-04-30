use crate as pallet_custom;
use crate::*;

use frame_support::{assert_err, assert_ok, parameter_types, traits::GenesisBuild};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use sp_std::convert::*;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Custom: pallet_custom::{Module, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
}

impl Config for Test {
    type Event = Event;
}

fn test_obj() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    pallet_custom::GenesisConfig::<Test> {
        ..Default::default()
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}

#[test]
fn new_account_creation() {
    test_obj().execute_with(|| {
        assert_ok!(Custom::create_account(
            Origin::signed(2),
            consts::MOVE_CURRENCY_CODE,
            0x1
        ));
        assert_err!(
            Custom::create_account(Origin::signed(334), 0xff, 255),
            Error::<Test>::CoinUnsupported
        );
        let script = String::from_utf8(<AccountsStore<Test>>::get(2).unwrap()).unwrap();
        println!("{}", script);
    });
}

#[test]
fn transfer_money_test() {
    test_obj().execute_with(|| {
        assert_ok!(Custom::transfer_funds(
            Origin::signed(2),
            consts::MOVE_CURRENCY_CODE,
            0x1,
            0x32
        ));
        assert_err!(
            Custom::transfer_funds(Origin::signed(2), 255, 1, 43),
            Error::<Test>::CoinUnsupported
        );

        let script = String::from_utf8(<AccountsStore<Test>>::get(2).unwrap()).unwrap();
        println!("{}", script);
    })
}

#[test]
fn coin_sanity_test() {
    assert_ok!(coins::SupportedCoin::<Test>::try_from(
        consts::MOVE_CURRENCY_CODE
    ));
    assert_eq!(
        coins::SupportedCoin::<Test>::try_from(255).err().unwrap(),
        Error::CoinUnsupported
    );
}

#[test]
fn account_info_sanity_test() {
    assert_ok!(coins::AccountInfo::<Test>::try_from((
        b"Bob".to_vec(),
        consts::MOVE_CURRENCY_CODE
    )));

    assert_eq!(
        coins::AccountInfo::<Test>::try_from((b"Bad".to_vec(), 255))
            .err()
            .unwrap(),
        Error::<Test>::CoinUnsupported
    );
    assert_eq!(
        coins::AccountInfo::<Test>::try_from((vec![], 255))
            .err()
            .unwrap(),
        Error::<Test>::NameEmpty
    );
}
