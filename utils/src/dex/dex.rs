use std::fmt;
use solana_client::rpc_response::transaction::Signature;
use solana_client::rpc_response::transaction::versioned::VersionedTransaction;
use solana_program::pubkey::Pubkey;
use crate::dex::errors::MintTransactionError;
use crate::orca::mint::OrcaMintedTransaction;
use crate::pumpfun::mint::PUMPFUNMintedTransaction;
use crate::raydium::mint::RaydiumMintedTransaction;
use crate::solarflare::mint::SolarFlareMintedTransaction;

pub trait MintedTokenTransaction {
    fn get_transaction(tx: Signature, rpc_endpoint: String) -> impl std::future::Future<Output = Result<MintTransaction, MintTransactionError>> + Send;
    fn parse(transaction: &VersionedTransaction) -> Result<MintTransaction, MintTransactionError>;
    fn get_dex(&self) -> &str;
    fn get_mint(&self) -> Pubkey;
    fn mock() -> Self;
}

#[derive(Debug, Clone)]
pub enum MintTransaction {
    ORCA(OrcaMintedTransaction),
    RAYDIUM(RaydiumMintedTransaction),
    PUMPFUN(PUMPFUNMintedTransaction),
    SOLARFLARE(SolarFlareMintedTransaction),
}
impl MintedTokenTransaction for MintTransaction {
    async fn get_transaction(_tx: Signature, _rpc_endpoint: String) -> Result<MintTransaction, MintTransactionError> {
        todo!("Not yet implemented")
    }

    fn parse(_transaction: &VersionedTransaction) -> Result<MintTransaction, MintTransactionError> {
        todo!("Not yet implemented")
    }

    fn get_dex(&self) -> &str {
        match self {
            MintTransaction::ORCA(tx) => tx.get_dex(),
            MintTransaction::RAYDIUM(tx) => tx.get_dex(),
            MintTransaction::PUMPFUN(tx) => tx.get_dex(),
            MintTransaction::SOLARFLARE(tx) => tx.get_dex()
        }
    }
    fn get_mint(&self) -> Pubkey {
        match self {
            MintTransaction::ORCA(tx) => tx.get_mint(),
            MintTransaction::RAYDIUM(tx) => tx.get_mint(),
            MintTransaction::PUMPFUN(tx) => tx.get_mint(),
            MintTransaction::SOLARFLARE(tx) => tx.get_mint(),
        }
    }

    fn mock() -> Self {
        todo!()
    }
}

impl fmt::Display for MintTransaction {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
        MintTransaction::ORCA(tx) => write!(f, "DEX: ORCA\n{:?}", tx),
        MintTransaction::RAYDIUM(tx) => write!(f, "{}", tx),
        MintTransaction::PUMPFUN(tx) => write!(f, "DEX: PUMPFUN\n{:?}", tx),
        MintTransaction::SOLARFLARE(tx) => write!(f, "DEX: SOLARFLARE\n{:?}", tx),
    }
}
}