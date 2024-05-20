use std::str;
use hex::{ decode, encode };
use std::ops::BitXor;

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    decode(hex).expect("Invalid hex string")
}

pub fn xor_bytes(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&b1, &b2)| b1.bitxor(b2))
        .collect()
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    encode(bytes)
}
