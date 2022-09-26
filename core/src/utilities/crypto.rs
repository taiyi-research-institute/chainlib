use sha2::{Digest, Sha256};
use sha3::Keccak256;
use ripemd::Ripemd160;

#[inline]
pub fn sha256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}

#[inline]
pub fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    hasher.finalize().into()
}

pub fn checksum(data: &[u8]) -> Vec<u8> {
    Sha256::digest(&Sha256::digest(&data)).to_vec()
}

pub fn hash160(bytes: &[u8]) -> Vec<u8> {
    Ripemd160::digest(&Sha256::digest(&bytes)).to_vec()
}