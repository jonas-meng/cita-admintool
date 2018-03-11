extern crate secp256k1;
extern crate rand;
extern crate crypto;

use secp256k1::Secp256k1;
use secp256k1::key::PublicKey;
use std::ops::Index;
use crypto::sha3::Sha3;
use crypto::digest::Digest;

pub struct Keypair {
    privkey: String, // 32-byte
    pubkey: String, // 64-byte
    address: String, // 20-byte
}

impl Keypair {
    pub fn new() -> Keypair {
        let the_secp256k1 = Secp256k1::new();
        let (privkey, pubkey) = the_secp256k1.generate_keypair(&mut rand::thread_rng()).unwrap();
        let privkey = to_hex(&privkey.index(..));
        let address = generate_address(pubkey, &the_secp256k1);
        let pubkey = to_hex(&pubkey.serialize_vec(&the_secp256k1, false)[1..]);
        Keypair { privkey, pubkey, address }
    }

    pub fn display(&self) {
        println!("PRIVATE KEY ({} BYTES): {}", self.privkey.len()/2, self.privkey);
        println!("PUBLIC KEY ({} BYTES): {}", self.pubkey.len()/2, self.pubkey);
        println!("ADDRESS ({} BYTES): {}", self.address.len()/2, self.address);
    }
    
}

fn generate_address(pk: PublicKey, the_secp256k1: &Secp256k1) -> String {
    let mut keccak = Sha3::keccak256();
    let mut pubhash = [0; 32];
    keccak.input(&pk.serialize_vec(the_secp256k1, false)[1..]);
    keccak.result(&mut pubhash);
    to_hex(&pubhash[12..])
}

pub fn to_hex(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    strs.join("")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hex() {
        let res = "0000";
        let bytes: [u8; 2] = [0; 2];
        assert_eq!(res, to_hex(&bytes));
    }

    #[test]
    fn test_keypair_length() {
        let test_keypair = Keypair::new();
        const PRIVATE_KEY_SIZE: usize = 32;
        const PUBLIC_KEY_SIZE: usize = 64;
        const ADDRESS_SIZE: usize = 20;
        assert_eq!(PRIVATE_KEY_SIZE*2, test_keypair.privkey.len());
        assert_eq!(PUBLIC_KEY_SIZE*2, test_keypair.pubkey.len());
        assert_eq!(ADDRESS_SIZE*2, test_keypair.address.len());
    }
}
