extern crate secp256k1;
extern crate rand;
extern crate crypto;
extern crate bitcoin;

use std::ops::Index;
use secp256k1::Secp256k1;
use bitcoin::util::hash;
use crypto::sha3::Sha3;
use crypto::digest::Digest;

pub fn to_hex(bytes: &[u8]) -> String {
    println!("{}", bytes.len());
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    strs.join("")
}

pub fn generate_key_addr() -> (String, String) {
    /*
     * privkey: 32-byte
     * pubkey: 64-byte
     * address: 20-byte, no extended
     *          24-byte, extedned
     *  keccak256(pubkey)[12..], if no extended
     *  keccak256(pubkey)[12..] + checksum, extended
     * */
    let the_secp256k1 = Secp256k1::new();
    let (privkey, pubkey) = the_secp256k1.generate_keypair(&mut rand::thread_rng()).unwrap();
    // private key 32-byte
    let sk = &privkey.index(..);
    // publick key 64-byte
    let pk = &pubkey.serialize_vec(&the_secp256k1, false)[1..65];
    let sk_hex = to_hex(sk);
    println!("PRIVATE KEY: {}", sk_hex);
    println!("PUBLIC KEY: {}", to_hex(pk));

    let mut keccak = Sha3::keccak256();
    let mut pubhash = [0; 32];
    keccak.input(pk);
    keccak.result(&mut pubhash);
    // public hash 20-byte
    let ph = &pubhash[12..];
    println!("PUBLIC HASH: {}", to_hex(ph));

    let extended = true;
    let addr_hex: String;
    if extended {
        let mut checksum = [0; 32];
        keccak.reset();
        keccak.input(ph);
        keccak.result(&mut checksum);
        let cksm = &checksum[0..4];
        println!("CHECKSUM: {}", to_hex(cksm));
        let mut address: Vec<u8> = Vec::with_capacity(24);
        address.extend_from_slice(ph);
        address.extend_from_slice(cksm);
        addr_hex = format!("0x{}", to_hex(&address));
    } else {
        addr_hex = format!("0x{}",to_hex(ph));
    }
    println!("ADDRESS: {}", addr_hex);
    (sk_hex, addr_hex)
}

fn main() {
    generate_key_addr();
}
