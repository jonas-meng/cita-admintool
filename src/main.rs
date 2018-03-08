extern crate secp256k1;
extern crate rand;

use std::env;
use std::fs::create_dir_all;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::process;
use secp256k1::Secp256k1;
use secp256k1::ContextFlag;
use secp256k1::key;

fn mk_keys_addr() {
    /*
    let args: Vec<String> = env::args().collect();
    let path = if args.len() == 2 {
        format!("{}",args[1])
    } else {
        format!("{}/node{}",args[1],args[2])
    };

    if let Err(e) = create_dir_all(&path) {
        println!("{}",e);
        process::exit(1);
    }

    let dump_path = format!("{}/privkey",path);
    let auth_path = format!("{}/authorities",args[1]);
    */
    
    let full = Secp256k1::with_caps(ContextFlag::Full);
    let (sk, pk) = full.generate_keypair(&mut rand::thread_rng()).unwrap();
    println!("{}",to_hex_string(&sk));

    // Notice: dump file shouldn't exist
    /*
    if let Err(e) = write_to_file(&dump_path,"This is dump path") {
        println!("{}",e);
        process::exit(1);
    }

    if let Err(e) = write_to_file(&auth_path,"This is dump path") {
        println!("{}",e);
        process::exit(1);
    }
    */
}

pub fn to_hex_string(sk: &key::SecretKey) -> String {
    let &key::SecretKey(bytes) = sk;
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join("")
}

fn write_to_file(file_path: &str, message: &str) -> Result<(), io::Error> {
    // try to create a file if not exist
    // append message to the end of the file
    let mut f = try!(OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path));
    try!(f.write_all(message.as_bytes()));
    Ok(())

}

fn main() {
    mk_keys_addr();
}
