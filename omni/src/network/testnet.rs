use crate::format::OmniFormat;
use crate::network::OmniNetwork;
use chainlib_core::no_std::*;
use chainlib_core::{
    AddressError, Network, NetworkError, PrivateKeyError
};

use core::{fmt, str::FromStr};
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Testnet;

impl Network for Testnet {
    const NAME: &'static str = "testnet";
}

impl OmniNetwork for Testnet {

    /// Returns the address prefix of the given network.
    fn to_address_prefix(format: &OmniFormat) -> Vec<u8> {
        match format {
            OmniFormat::P2PKH => vec![0x6F],
            OmniFormat::P2WSH => vec![0x00],
            OmniFormat::P2SH_P2WPKH => vec![0xC4],
            OmniFormat::Bech32 => vec![0x74, 0x62],
        }
    }

    /// Returns the network of the given address prefix.
    fn from_address_prefix(prefix: &[u8]) -> Result<Self, AddressError> {
        match (prefix[0], prefix[1]) {
            (0x6F, _) | (0xC4, _) | (0x74, 0x62) => Ok(Self),
            _ => Err(AddressError::InvalidPrefix(String::from_utf8(prefix.to_owned())?)),
        }
    }

    /// Returns the wif prefix of the given network.
    fn to_private_key_prefix() -> u8 {
        0xEF
    }

    /// Returns the network of the given wif prefix.
    fn from_private_key_prefix(prefix: u8) -> Result<Self, PrivateKeyError> {
        match prefix {
            0xEF => Ok(Self),
            _ => Err(PrivateKeyError::InvalidPrefix(vec![prefix])),
        }
    }
}

impl FromStr for Testnet {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::NAME => Ok(Self),
            _ => Err(NetworkError::InvalidNetwork(s.into())),
        }
    }
}

impl fmt::Display for Testnet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
