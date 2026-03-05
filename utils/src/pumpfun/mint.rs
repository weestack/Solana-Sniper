use solana_client::rpc_response::transaction::Signature;
use solana_client::rpc_response::transaction::versioned::VersionedTransaction;
use solana_program::pubkey;
use solana_program::pubkey::Pubkey;
use crate::dex::dex::{MintTransaction, MintedTokenTransaction};
use crate::dex::errors::MintTransactionError;

#[derive(Debug, Clone)]
pub struct PUMPFUNMintedTransaction {
    pub mint: Pubkey,
}
impl MintedTokenTransaction for PUMPFUNMintedTransaction {
    async fn get_transaction(_tx: Signature, _rpc_endpoint: String) -> Result<MintTransaction, MintTransactionError> {
        todo!()
    }

    fn parse(_transaction: &VersionedTransaction) -> Result<MintTransaction, MintTransactionError> {
        todo!()
    }

    fn get_dex(&self) -> &str { "PUMPFUN" }
    fn get_mint(&self) -> Pubkey { pubkey!("So1111111111111111111111111111111111PUMPFUN") }

    fn mock() -> Self {
        todo!()
    }
}