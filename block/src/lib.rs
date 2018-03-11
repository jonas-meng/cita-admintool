#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate time;

fn get_time_in_sec() -> i64 {
    time::get_time().sec
}
    
#[derive(Serialize, Deserialize)]
struct Admin {
    pubkey: String,
    crypto: String,
    identifier: String,
}
    
#[derive(Serialize, Deserialize)]
pub struct Block {
    prevhash: String,
    timestamp: i64,
    admin: Admin,
}
    
impl Block {

    pub fn genesis_block(pubkey: String, crypto: String, identifier: String) -> Block {
        let admin = Admin { pubkey, crypto, identifier  };
        let prevhash = String::from("0x0000000000000000000000000000000000000000000000000000000000000000");
        let timestamp = get_time_in_sec();
        Block { prevhash, timestamp, admin  }
    }
        
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block() {
        let bk = Block::genesis_block(
            String::from(""),
            String::from(""),
            String::from(""));
        let part1 = "{\"prevhash\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"timestamp\":";
        let part2 = ",\"admin\":{\"pubkey\":\"\",\"crypto\":\"\",\"identifier\":\"\"}}";
        let res = format!("{}{}{}", part1, bk.timestamp, part2);
        assert_eq!(res, bk.to_json());
    }
}
