use std::sync::Arc;
use tokio;

use ethers_providers::{Http, Middleware, Provider};
use revm::db::EthersDB;

#[tokio::main]
async fn main() {
    let url = "http://localhost:8124/eth";
    let client = Provider::<Http>::try_from(url).unwrap();

    let block_number = client.get_block_number().await.unwrap();
    println!("block_number: {:?}", block_number);

    let _ = EthersDB::new(Arc::new(client), None).unwrap();

    panic!("we should panic");
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

        panic!("let's panic");
    }
}
