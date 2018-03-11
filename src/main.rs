extern crate block;
extern crate keypair;

use block::Block;
use keypair::Keypair;
use std::fs::OpenOptions;
use std::io::Write;

fn write_to_file(path: &str, data: &str, append: bool) {
    let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .open(path)
            .unwrap();
    f.write_all(data.as_bytes()).unwrap();
}

fn main() {
    // generate private key, public key, and address
    let kp = Keypair::new();
    kp.display();
    write_to_file("/home/xuming/projects/garbage/private.key", &kp.privkey, false);
    // generate genesis block
    let bk = Block::genesis_block(
        String::from(""),
        String::from(""),
        String::from(""));
    println!("{}", bk.to_json());
    write_to_file("/home/xuming/projects/garbage/genesis.json", &bk.to_json(), true);
}
