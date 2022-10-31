use crate::format::OmniFormat;
use chainlib_core::no_std::*;
use chainlib_core::{
    AddressError, Network, PrivateKeyError,
};

pub mod mainnet;
pub use self::mainnet::*;

pub mod testnet;
pub use self::testnet::*;

/// The interface for a Omni network.
pub trait OmniNetwork: Network {

    /// Returns the address prefix of the given network.
    fn to_address_prefix(format: &OmniFormat) -> Vec<u8>;

    /// Returns the network of the given address prefix.
    fn from_address_prefix(prefix: &[u8]) -> Result<Self, AddressError>;

    /// Returns the wif prefix of the given network.
    fn to_private_key_prefix() -> u8;

    /// Returns the network of the given wif prefix.
    fn from_private_key_prefix(prefix: u8) -> Result<Self, PrivateKeyError>;

}
