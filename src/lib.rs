#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use revm::db::EthersDB;

    #[tokio::test]
    async fn test_hang() {
        use ethers_providers::{Http, Middleware, Provider};

        let url = "http://localhost:8124/eth";
        let client = Provider::<Http>::try_from(url).unwrap();

        let block_number = client.get_block_number().await.unwrap();
        println!("block_number: {:?}", block_number);

        let _ = EthersDB::new(Arc::new(client), None).unwrap();

        panic!("let's panic");
    }
}
