pub mod env;
pub mod raydium;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use solana_sdk::signature::Signature;
    use crate::raydium::initialize2::RaydiumInitialize2Transaction;

    #[tokio::test]
    async fn test_raydium_initialize2() {
        let rpc_endpoint = "https://api.mainnet-beta.solana.com";
        /* https://solscan.io/tx/fDJR6Bgm7Fm7uF4uJDwXmC99i8WvqKQ3hz6z64iXvjxWqzNsTWYDtEY6VcHzivkHvRZkZhdLEuLybMWQgvy3EJq */
        let initialize2_1 = Signature::from_str("fDJR6Bgm7Fm7uF4uJDwXmC99i8WvqKQ3hz6z64iXvjxWqzNsTWYDtEY6VcHzivkHvRZkZhdLEuLybMWQgvy3EJq").unwrap();
        /* https://solscan.io/tx/4UpnVxZJoSuTz1qefjBLX6Y1pS2krKY6xym9UrHEyqKiNXbDcxTGLoGDd6YxWqWH85K6PFwnpywgKN1tx97cabCb */
        let initialize2_2 = Signature::from_str("4UpnVxZJoSuTz1qefjBLX6Y1pS2krKY6xym9UrHEyqKiNXbDcxTGLoGDd6YxWqWH85K6PFwnpywgKN1tx97cabCb").unwrap();

        let test1 = RaydiumInitialize2Transaction::get_transaction(initialize2_1, rpc_endpoint.to_string()).await;
        let test2 = RaydiumInitialize2Transaction::get_transaction(initialize2_2, rpc_endpoint.to_string()).await;

        assert!(test1.is_ok());
        assert!(test2.is_ok());

        let initialize2 = test1.unwrap();
        assert_eq!(initialize2.get_mint().to_string(), "BtNQebJw96viooG7mSWhK1dgeb6tLNKn2EVab8Gnpump");
        let initialize2 = test2.unwrap();
        assert_eq!(initialize2.get_mint().to_string(), "Gyd7ZymwPv7hosgzr46biJYMC6tq65LG3v5SLaApump");
    }

    #[tokio::test]
    async fn test_raydium_initialize2_wrong_transaction() {
        let rpc_endpoint = "https://api.mainnet-beta.solana.com";
        let initialize2_random = Signature::new_unique();

        let test_random = RaydiumInitialize2Transaction::get_transaction(initialize2_random, rpc_endpoint.to_string()).await;
        assert!(test_random.is_err());
    }
}
