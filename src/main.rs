extern crate block;
extern crate keypair;

use block::Block;
use keypair::Keypair;


fn main() {
    // generate private key, public key, and address
    let kp = Keypair::new();
    kp.display();
    // generate genesis block
    /*
    let bk = Block::genesis_block(
        String::from(""),
        String::from(""),
        String::from(""));
    println!("{}", bk.to_json());
    */
}
