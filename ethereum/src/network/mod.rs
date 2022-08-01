use chainlib_core::{Network};

pub mod goerli;
pub use self::goerli::*;

pub mod kovan;
pub use self::kovan::*;

pub mod mainnet;
pub use self::mainnet::*;

pub mod rinkeby;
pub use self::rinkeby::*;

pub mod ropsten;
pub use self::ropsten::*;

pub mod heco;
pub use self::heco::*;

pub mod bsc;
pub use self::bsc::*;

/// The interface for an Ethereum network.
pub trait EthereumNetwork: Network {
    const CHAIN_ID: u32;
    const NETWORK_ID: u32;
}
