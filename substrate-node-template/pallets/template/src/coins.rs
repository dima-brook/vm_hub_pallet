use crate::consts;
use crate::{Config, Error};
use sp_std::{
    marker::PhantomData,
    convert::{TryFrom, TryInto}
};

pub struct AccountInfo<T> {
    name: Vec<u8>,
    coin: SupportedCoin<T>
}

impl<T: Config> TryFrom<(Vec<u8>, u8)> for AccountInfo<T> {
    type Error = Error<T>;

    fn try_from(val: (Vec<u8>, u8)) -> Result<Self, Error<T>> {
        return Ok(Self {
            name: val.0,
            coin: val.1.try_into()?
        })
    }
}

// Supported coins
pub enum SupportedCoin<T> {
    Ethereum,
    _Unreachable(PhantomData<T>)
}

impl<T: Config> TryFrom<u8> for SupportedCoin<T> {
    type Error = Error<T>;

    fn try_from(val: u8) -> Result<Self, Error<T>> {
        return match val {
            consts::ETH_CURRENCY_CODE => Ok(SupportedCoin::<T>::Ethereum),
            _ => Err(Error::CoinUnsupported)
        }
    }
}
