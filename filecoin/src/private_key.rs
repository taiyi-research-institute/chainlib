use crate::address::FilecoinAddress;
use crate::format::FilecoinFormat;
use crate::public_key::FilecoinPublicKey;
use chainlib_core::{
    PrivateKey,
    PrivateKeyError,
    PublicKey,
    Address,
    AddressError,
    libsecp256k1,
    bls_signatures::{self, Serialize},
    hex,
};

use core::{fmt, fmt::Display, str::FromStr, panic};
use rand::Rng;

/// Represents a filecoin private key
#[derive(Debug, Clone, PartialEq)]
pub enum FilecoinPrivateKey {
    Secp256k1(libsecp256k1::SecretKey),
    Bls(bls_signatures::PrivateKey),
}

impl PrivateKey for FilecoinPrivateKey {
    type Address = FilecoinAddress;
    type Format = FilecoinFormat;
    type PublicKey = FilecoinPublicKey;

    /// Returns a new filecoin private key wrapping a randomly generated
    /// secp256k1 secret key, to return a new filecoin private key that
    /// wraps a bls private key, call the non-trait function new_bls()
    fn new<R: Rng>(rng: &mut R) -> Result<Self, PrivateKeyError> {
        let random: [u8; 32] = rng.gen();
        Ok(Self::Secp256k1(libsecp256k1::SecretKey::parse_slice(&random)?))
    }

    /// Returns the public key of the corresponding filecoin private key.
    fn to_public_key(&self) -> Self::PublicKey {
        Self::PublicKey::from_private_key(self)
    }

    /// Returns the address of the corresponding filecoin private key.
    fn to_address(&self, _format: &Self::Format) -> Result<Self::Address, AddressError> {
        Self::Address::from_private_key(self, _format)
    }
}

impl FilecoinPrivateKey {

    /// Returns a randomly generated filecoin private key for secp256k1 curve
    pub fn new_secp256k1() -> Result<Self, PrivateKeyError> {
        let mut rng = rand::thread_rng();
        Self::new(&mut rng)
    }

    /// Returns a randomly generated filecoin private key for bls curve
    pub fn new_bls() -> Result<Self, PrivateKeyError> {
        let mut rng = rand::thread_rng();
        let random: [u8; 32] = rng.gen();
        Ok(Self::Bls(bls_signatures::PrivateKey::new(&random)))
    }

    /// Returns a filecoin private key given an secp256k1 secret key.
    pub fn from_secp256k1_secret_key(secret_key: &libsecp256k1::SecretKey) -> Self {
        Self::Secp256k1(secret_key.clone())
    }

    /// Returns the secp256k1 secret key of this filecoin private key.
    pub fn to_secp256k1_secret_key(&self) -> libsecp256k1::SecretKey {
        match self {
            Self::Secp256k1(key) => key.clone(),
            _ => panic!("not an secp256k1 secret key")
        }
    }

    /// Returns a filecoin private key given a bls private key
    pub fn from_bls_private_key(private_key: &bls_signatures::PrivateKey) -> Self {
        Self::Bls(private_key.clone())
    }

    /// Returns the bls private key of this filecoin private key
    pub fn to_bls_private_key(&self) -> bls_signatures::PrivateKey {
        match self {
            Self::Bls(key) => key.clone(),
            _ => panic!("not a bls private key"),
        }
    }
}

impl FromStr for FilecoinPrivateKey {
    type Err = PrivateKeyError;

    fn from_str(private_key: &str) -> Result<Self, Self::Err> {
        let mut s = private_key.to_string();
        let mut is_bls = false;
        if s.starts_with("secp256k1_priv_") {
            s = s[15..].to_string();
        } else if s.starts_with("bls_priv_") {
            s = s[9..].to_string();
            is_bls = true;
        } else {
            return Err(PrivateKeyError::InvalidPrefix(vec![]));
        }
        if s.len() != 64 {
            return Err(PrivateKeyError::InvalidCharacterLength(private_key.len()));
        }

        let stream = hex::decode(&s)?;
        match is_bls {
            true => {
                let privkey = bls_signatures::PrivateKey::from_bytes(&stream).unwrap();
                Ok(Self::Bls(privkey))
            },
            false => {
                let privkey = libsecp256k1::SecretKey::parse_slice(&stream).unwrap();
                Ok(Self::Secp256k1(privkey))
            }
        }
    }
}

impl Display for FilecoinPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Secp256k1(key) => {
                let mut s = "secp256k1_priv_".to_string();
                s.push_str(&hex::encode(&key.serialize()));
                write!(f, "{}", s)
            },
            Self::Bls(key) => {
                let mut s = "bls_priv_".to_string();
                s.push_str(&hex::encode(&key.as_bytes()));
                write!(f, "{}", s)
            },
        }
    }
}

#[test]
fn test() {

    let secp_priv_key = FilecoinPrivateKey::new_secp256k1().unwrap();
    let bls_priv_key = FilecoinPrivateKey::new_bls().unwrap();
    
    let secp_pub_key = secp_priv_key.to_public_key();
    let bls_pub_key = bls_priv_key.to_public_key();

    println!("secp priv key = {}\nbls priv key = {}\n", secp_priv_key, bls_priv_key);
    println!("secp pub key = {}\nbls pub key = {}", secp_pub_key, bls_pub_key);

    let s = "secp256k1_pub_045e2a96afdebf578c06b2936338cbff7995b8ff33cbbed4f4fb8fac22b9bf55e34db66f5711f4ca1c903e88e9708b95417b4c77ddba31824674ca2f5b05673531";
    let key = FilecoinPublicKey::from_str(s).unwrap();

    println!("pub key = {}", key);
}