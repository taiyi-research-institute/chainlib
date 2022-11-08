use crate::address::FilecoinAddress;
use crate::format::FilecoinFormat;
use crate::public_key::FilecoinPublicKey;
use chainlib_core::{Address, PrivateKey, PrivateKeyError, PublicKey, libsecp256k1,hex, Error, AddressError};

use core::{fmt, fmt::Display, str::FromStr};
use rand::Rng;

/// Represents a filecoin private key
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilecoinPrivateKey(libsecp256k1::SecretKey);

impl PrivateKey for FilecoinPrivateKey {
    type Address = FilecoinAddress;
    type Format = FilecoinFormat;
    type PublicKey = FilecoinPublicKey;

    /// Returns a randomly-generated filecoin private key.
    fn new<R: Rng>(rng: &mut R) -> Result<Self, PrivateKeyError> {
        let random: [u8; 32] = rng.gen();
        Ok(Self(libsecp256k1::SecretKey::parse_slice(&random)?))
    }

    /// Returns the public key of the corresponding filecoin private key.
    fn to_public_key(&self) -> Self::PublicKey {
        FilecoinPublicKey::from_private_key(self)
    }

    /// Returns the address of the corresponding filecoin private key.
    fn to_address(&self, _format: &Self::Format) -> Result<Self::Address, AddressError> {
        FilecoinAddress::from_private_key(self, _format)
    }
}

impl FilecoinPrivateKey {
    /// Returns a private key given a secp256k1 secret key.
    pub fn from_secp256k1_secret_key(secret_key: &libsecp256k1::SecretKey) -> Self {
        Self(secret_key.clone())
    }

    /// Returns the secp256k1 secret key of the private key.
    pub fn to_secp256k1_secret_key(&self) -> libsecp256k1::SecretKey {
        self.0.clone()
    }
}

impl FromStr for FilecoinPrivateKey {
    type Err = PrivateKeyError;

    fn from_str(private_key: &str) -> Result<Self, PrivateKeyError> {
        if private_key.len() != 64 {
            return Err(PrivateKeyError::InvalidCharacterLength(private_key.len()));
        }

        let secret_key = hex::decode(private_key)?;
        Ok(Self(libsecp256k1::SecretKey::parse_slice(&secret_key)?))
    }
}

impl Display for FilecoinPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut private_key = [0u8; 32];
        private_key.copy_from_slice(&self.0.serialize());
        write!(f, "{}", hex::encode(private_key).to_string())
    }
}
