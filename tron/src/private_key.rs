use chainlib_core::Address;
use chainlib_core::AddressError;
use chainlib_core::PublicKey;
use chainlib_core::libsecp256k1;
use chainlib_core::PrivateKey;
use chainlib_core::PrivateKeyError;
use core::fmt::{self,Display};

use core::str::FromStr;
use crate::{TronAddress,TronFormat,TronPublicKey};
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TronPrivateKey(libsecp256k1::SecretKey);

impl PrivateKey for TronPrivateKey {
    type Address = TronAddress;

    type Format = TronFormat;

    type PublicKey = TronPublicKey;

    fn new<R: Rng>(rng: &mut R) -> Result<Self, PrivateKeyError> {
        let random: [u8; 32] = rng.gen();
        Ok(Self(libsecp256k1::SecretKey::parse_slice(&random)?))
    }

    fn to_public_key(&self) -> Self::PublicKey {
        TronPublicKey::from_private_key(self)
    }

    fn to_address(&self, format: &Self::Format) -> Result<Self::Address, AddressError> {
        TronAddress::from_private_key(self, format)
    }
}

impl TronPrivateKey {
    /// Returns a private key given a secp256k1 secret key.
    pub fn from_secp256k1_secret_key(secret_key: &libsecp256k1::SecretKey) -> Self {
        Self(secret_key.clone())
    }

    /// Returns the secp256k1 secret key of the private key.
    pub fn to_secp256k1_secret_key(&self) -> libsecp256k1::SecretKey {
        self.0.clone()
    }
}


impl FromStr for TronPrivateKey {
    type Err = PrivateKeyError;

    fn from_str(private_key: &str) -> Result<Self, PrivateKeyError> {
        if private_key.len() != 64 {
            return Err(PrivateKeyError::InvalidCharacterLength(private_key.len()));
        }

        let secret_key = hex::decode(private_key)?;
        Ok(Self(libsecp256k1::SecretKey::parse_slice(&secret_key)?))
    }
}

impl Display for TronPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut private_key = [0u8; 32];
        private_key.copy_from_slice(&self.0.serialize());
        write!(f, "{}", hex::encode(private_key).to_string())
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::{TronAddress,TronFormat,TronPublicKey};

    #[test]
    pub fn test_from_hex(){
        let privkey = TronPrivateKey::from_str("0838b9c472def15e82fed31208944a683b37dfb09f5a04febc45416bd8a00161").unwrap();
        println!("{}",privkey);
        let pubkey = TronPublicKey::from_private_key(&privkey);
        let addr = TronAddress::from_public_key(&pubkey,&TronFormat::Standard).unwrap();
        assert_eq!(addr.to_base58(),"TG7jQ7eGsns6nmQNfcKNgZKyKBFkx7CvXr".to_string())
    }
}