use std::sync::Arc;

use ethers_providers::{Http, Provider};
use revm::{
    db::EthersDB,
    primitives::{Address, U256},
    Database,
};

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: cargo run <hang|nohang>");
        return;
    }

    // setup
    let url = "http://localhost:8124/eth";
    let client = Provider::<Http>::try_from(url).unwrap();

    if args[1] == "nohang" {
        let _ = EthersDB::new(Arc::new(client), None).unwrap();
        panic!("this panic is expected");
    } else {
        let mut ethers_db = EthersDB::new(Arc::new(client), None).unwrap();

        let addr = Address::ZERO;
        let slot = U256::ZERO;
        for _ in 0..1000 {
            ethers_db.storage(addr, slot).unwrap();
        }
        panic!("we want to panic but it hangs");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_hang() {
        let url = "http://localhost:8124/eth";
        let client = Provider::<Http>::try_from(url).unwrap();

        let block_number = client.get_block_number().await.unwrap();
        println!("block_number: {:?}", block_number);

        let _ = EthersDB::new(Arc::new(client), None).unwrap();

        panic!("we want to panic but it hangs");
    }
}
