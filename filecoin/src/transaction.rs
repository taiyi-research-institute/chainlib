use crate::address::FilecoinAddress;
use crate::address::Address;
use crate::address::ADDRESS_ENCODER as BASE32_ENCODER;
use crate::format::FilecoinFormat;
use crate::private_key::FilecoinPrivateKey;
use crate::public_key::FilecoinPublicKey;
use chainlib_core::{PublicKey, Transaction, TransactionId, libsecp256k1, Error, TransactionError, crypto::blake2b_256};

use anyhow::anyhow;
use fvm_ipld_encoding::de::{Deserialize, Deserializer};
use fvm_ipld_encoding::ser::{Serialize, Serializer};
use fvm_ipld_encoding::{de, ser, serde_bytes, Cbor, RawBytes, to_vec};

use fvm_shared::bigint::bigint_ser::{BigIntDe, BigIntSer};
use fvm_shared::econ::TokenAmount;
use fvm_shared::MethodNum;
use num_derive::FromPrimitive;
use forest_encoding::tuple::*;
use fvm_ipld_encoding::repr::*;

use core::panic;
use std::fmt;
use std::borrow::Cow;

/// Represents the parameters for a filecoin transaction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FilecoinTransactionParameters {
    pub version: i64,
    pub from: Address,
    pub to: Address,
    pub sequence: u64,
    pub value: TokenAmount,
    pub method_num: MethodNum,
    pub params: RawBytes,
    pub gas_limit: i64,
    pub gas_fee_cap: TokenAmount,
    pub gas_premium: TokenAmount,
}

impl Cbor for FilecoinTransactionParameters {}

impl FilecoinTransactionParameters {
    /// Helper function to convert the message into signing bytes.
    /// This function returns the message `Cid` bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        // Safe to unwrap here, unsigned message cannot fail to serialize.
        self.cid().unwrap().to_bytes()
    }

    /// Does some basic checks on the Message to see if the fields are valid.
    pub fn check(self: &FilecoinTransactionParameters) -> anyhow::Result<()> {
        if self.gas_limit == 0 {
            return Err(anyhow!("Message has no gas limit set"));
        }
        if self.gas_limit < 0 {
            return Err(anyhow!("Message has negative gas limit"));
        }
        Ok(())
    }
}

impl Serialize for FilecoinTransactionParameters {
    fn serialize<S>(&self, s: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (
            &self.version,
            &self.to,
            &self.from,
            &self.sequence,
            BigIntSer(&self.value),
            &self.gas_limit,
            BigIntSer(&self.gas_fee_cap),
            BigIntSer(&self.gas_premium),
            &self.method_num,
            &self.params,
        )
            .serialize(s)
    }
}

impl<'de> Deserialize<'de> for FilecoinTransactionParameters {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (
            version,
            to,
            from,
            sequence,
            BigIntDe(value),
            gas_limit,
            BigIntDe(gas_fee_cap),
            BigIntDe(gas_premium),
            method_num,
            params,
        ) = Deserialize::deserialize(deserializer)?;
        Ok(Self {
            version,
            from,
            to,
            sequence,
            value,
            method_num,
            params,
            gas_limit,
            gas_fee_cap,
            gas_premium,
        })
    }
}

/// Signature variants for Filecoin signatures.
#[derive(
    Clone, Debug, PartialEq, FromPrimitive, Copy, Eq, Serialize_repr, Deserialize_repr, Hash, Default
)]
#[repr(u8)]
pub enum FilecoinSignatureType {
    #[default]
    Secp256k1 = 1,
    BLS = 2,
}

/// A cryptographic signature, represented in bytes, of any key protocol.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct FilecoinSignature {
    pub sig_type: FilecoinSignatureType,
    pub bytes: Vec<u8>,
}

impl Cbor for FilecoinSignature {}

impl ser::Serialize for FilecoinSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut bytes = Vec::with_capacity(self.bytes.len() + 1);
        // Insert signature type byte
        bytes.push(self.sig_type as u8);
        bytes.extend_from_slice(&self.bytes);

        serde_bytes::Serialize::serialize(&bytes, serializer)
    }
}

impl<'de> de::Deserialize<'de> for FilecoinSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let bytes: Cow<'de, [u8]> = serde_bytes::Deserialize::deserialize(deserializer)?;
        if bytes.is_empty() {
            return Err(de::Error::custom("Cannot deserialize empty bytes"));
        }

        // Remove signature type byte
        let mut sig_type = FilecoinSignatureType::Secp256k1;
        let b = bytes[0];
        if b == 1 {
        } else if b == 2 {
            sig_type = FilecoinSignatureType::BLS;
        } else {
            panic!("Invalid signature type byte (must be 1 or 2)")
        }
        
        Ok(FilecoinSignature {
            bytes: bytes[1..].to_vec(),
            sig_type,
        })
    }
}

/// Represents a wrapped filecoin transaction with signature bytes.
#[derive(PartialEq, Clone, Debug, Serialize_tuple, Deserialize_tuple, Hash, Eq, Default)]
pub struct FilecoinTransaction {
    pub params: FilecoinTransactionParameters,
    pub signature: FilecoinSignature,
}

/// Represents a filecoin transaction id
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FilecoinTransactionId {
    hash: Vec<u8>
}

impl TransactionId for FilecoinTransactionId {}

impl fmt::Display for FilecoinTransactionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &BASE32_ENCODER.encode(&self.hash[..]))
    }
}

impl Transaction for FilecoinTransaction {
    type Address = FilecoinAddress;
    type Format = FilecoinFormat;
    type PrivateKey = FilecoinPrivateKey;
    type PublicKey = FilecoinPublicKey;
    type TransactionId = FilecoinTransactionId;
    type TransactionParameters = FilecoinTransactionParameters;

    fn new(parameters: &Self::TransactionParameters) -> Result<Self, TransactionError> {
        Ok(Self {
            params: parameters.clone(),
            signature: FilecoinSignature::default(),
        })
    }
    
    fn from_bytes(transaction: &Vec<u8>) -> Result<Self, TransactionError> {
        // deserialization waited to be specified
        Ok(FilecoinTransaction::default())
    }
    
    fn sign(&mut self, signature: Vec<u8>, recid: u8) -> Result<Vec<u8>, TransactionError> {
        let mut signature = signature;
        signature.push(recid);
        let sig = FilecoinSignature{
            sig_type: FilecoinSignatureType::Secp256k1,
            bytes: signature,
        };
        self.signature = sig;
        self.to_bytes()
    }

    fn sign_with_private_key(&mut self, private_key: &Self::PrivateKey) -> Result<Vec<u8>, TransactionError> {
        let hash = blake2b_256(&self.params.to_bytes()[..]);
        let msg = libsecp256k1::Message::parse_slice(&hash[..]).unwrap();
        let key = private_key.to_secp256k1_secret_key();
        let (sig, recid) = libsecp256k1::sign(&msg, &key);
        self.sign(sig.serialize().to_vec(), recid.serialize())
    }

    fn to_bytes(&self) -> Result<Vec<u8>, TransactionError> {
        Ok(to_vec(self).unwrap())
    }

    fn to_transaction_id(&self) -> Result<Self::TransactionId, TransactionError> {
        let stream = self.to_bytes().unwrap();
        Ok(FilecoinTransactionId{
            hash: blake2b_256(&stream[..]).to_vec(),
        })
    }
}
