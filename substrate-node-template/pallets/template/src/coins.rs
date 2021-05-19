use crate::consts;
use crate::{Config, Error};
use sp_std::{
    convert::{TryFrom, TryInto},
    marker::PhantomData,
};

// Information associated with an account
#[derive(Debug)]
pub struct AccountInfo<T> {
    pub name: Vec<u8>,
    pub coin: SupportedCoin<T>,
}

impl<T: Config> TryFrom<(Vec<u8>, u8)> for AccountInfo<T> {
    type Error = Error<T>;

    fn try_from(val: (Vec<u8>, u8)) -> Result<Self, Error<T>> {
        if val.0.is_empty() {
            return Err(Error::<T>::NameEmpty);
        }

        Ok(Self {
            name: val.0,
            coin: val.1.try_into()?,
        })
    }
}

// Supported coins
#[derive(Debug)]
pub enum SupportedCoin<T> {
    //Ethereum,
    Move,
    _Unreachable(PhantomData<T>),
}

impl<T: Config> TryFrom<u8> for SupportedCoin<T> {
    type Error = Error<T>;

    fn try_from(val: u8) -> Result<Self, Error<T>> {
        match val {
            //consts::ETH_CURRENCY_CODE => Ok(SupportedCoin::<T>::Ethereum),
            consts::MOVE_CURRENCY_CODE => Ok(SupportedCoin::<T>::Move),
            _ => Err(Error::CoinUnsupported),
        }
    }
}

impl<T: Config> Into<&str> for SupportedCoin<T> {
    fn into(self) -> &'static str {
        match self {
            SupportedCoin::Move => xp_compiler::consts::langs::MOVE,
            SupportedCoin::_Unreachable(_) => unreachable!()
        }
    }
}
