use crate::address::FilecoinAddress;
use crate::format::FilecoinFormat;
use crate::private_key::FilecoinPrivateKey;
use chainlib_core::{
    Address,
    PublicKey,
    PublicKeyError,
    libsecp256k1,
    hex,
    Error,
    AddressError
};

use core::{fmt, fmt::Display, str::FromStr};

/// Represents a filecoin public key
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilecoinPublicKey(libsecp256k1::PublicKey);

impl PublicKey for FilecoinPublicKey {
    type Address = FilecoinAddress;
    type Format = FilecoinFormat;
    type PrivateKey = FilecoinPrivateKey;

    /// Returns the public key corresponding to the given private key.
    fn from_private_key(private_key: &Self::PrivateKey) -> Self {
        Self(libsecp256k1::PublicKey::from_secret_key(
            &private_key.to_secp256k1_secret_key(),
        ))
    }

    /// Returns the address of this public key.
    fn to_address(&self, _format: &Self::Format) -> Result<Self::Address, AddressError> {
        FilecoinAddress::from_public_key(self, _format)
    }
}

impl FilecoinPublicKey {
    /// Returns a public key given a secp256k1 public key.
    pub fn from_secp256k1_public_key(public_key: libsecp256k1::PublicKey) -> Self {
        Self(public_key)
    }

    pub fn from_slice(sl: &[u8]) -> Self {
        Self(libsecp256k1::PublicKey::parse_slice(sl, None).unwrap())
    }

    /// Returns the secp256k1 public key of the public key
    pub fn to_secp256k1_public_key(&self) -> libsecp256k1::PublicKey {
        self.0.clone()
    }
}

impl FromStr for FilecoinPublicKey {
    type Err = PublicKeyError;

    fn from_str(public_key: &str) -> Result<Self, Self::Err> {
        Ok(Self(libsecp256k1::PublicKey::parse_slice(
            hex::decode(public_key)?.as_slice(),
            None,
        )?))
    }
}

impl Display for FilecoinPublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for s in &self.0.serialize()[1..] {
            write!(f, "{:02x}", s)?;
        }
        Ok(())
    }
}
