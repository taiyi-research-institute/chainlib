#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused_extern_crates, dead_code)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate thiserror;

pub mod address;
pub use self::address::*;

pub mod format;
pub use self::format::*;

pub mod network;
pub use self::network::*;

pub mod private_key;
pub use private_key::*;

pub mod public_key;

pub mod witness_program;

pub mod transaction;

pub mod amount;

mod testnet_daemon{
    use super::*;
    use chainlib_core::PrivateKey;
    use rand::thread_rng;
    type N = super::network::Testnet;

    /**
     *  cTt14Wpo6gKBSjuf9PjrbYc1Jz9hN4wepeJLm1DANsE6v5szQn4h
        mhSWVCZ7GtrYeDavBbZCUKownLPSAnxMyD
     */
    const ACOUNTS: [(&str,&str);2] = [
        ("cTt14Wpo6gKBSjuf9PjrbYc1Jz9hN4wepeJLm1DANsE6v5szQn4h","mhSWVCZ7GtrYeDavBbZCUKownLPSAnxMyD"),
        ("cRfuiTAEpcEdgjXHLkYK3mFZWARjgMDRYGdka2G5hGpQdGKVATqN","miByfZ8aBt8xQwUW9DXJw4ockykf2P42MZ")
    ];
    
    #[test]
    fn create_account(){
        let priv_key = BitcoinPrivateKey::<N>::new(&mut thread_rng()).unwrap();
        println!("{}",priv_key.to_string());
        let address = priv_key.to_address(&BitcoinFormat::P2PKH).unwrap();
        println!("{}",address)
    }

    #[test]
    fn send_p2pkh(){

    }
}