use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use std::fmt;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::{CommitmentConfig, RpcTransactionConfig, UiTransactionEncoding};
use solana_client::rpc_response::transaction::versioned::VersionedTransaction;
use solana_sdk::signature::Signature;
use crate::dex::dex::{MintTransaction, MintedTokenTransaction};
use crate::dex::errors::MintTransactionError;

#[derive(Debug,Clone)]
pub struct RaydiumMintedTransaction {
    token_program: Pubkey,
    spl_associated_token_account: Pubkey,
    system_program: Pubkey,
    rent_program: Pubkey,
    amm: Pubkey,
    amm_authority: Pubkey,
    amm_open_orders: Pubkey,
    lp_mint: Pubkey,
    coin_mint: Pubkey,
    pc_mint: Pubkey,
    pool_coin_token_account: Pubkey,
    pool_pc_token_account: Pubkey,
    pool_withdraw_queue: Pubkey,
    amm_target_orders: Pubkey,
    pool_temp_lp: Pubkey,
    serum_program: Pubkey,
    serum_market: Pubkey,
    user_wallet: Pubkey,
    user_token_coin: Pubkey,
    user_token_pc: Pubkey,
    user_lp_token_account: Pubkey,
}

impl MintedTokenTransaction for RaydiumMintedTransaction {

    async fn get_transaction(tx: Signature, rpc_endpoint: String) -> Result<MintTransaction, MintTransactionError> {
        let client =
            RpcClient::new_with_commitment(rpc_endpoint, CommitmentConfig::processed());

        let config = RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::Binary),
            commitment: Some(CommitmentConfig::confirmed()),
            max_supported_transaction_version: Some(2),
        };
        let transaction = client
            .get_transaction_with_config(&tx, config)
            .await
            .map_err(|_| MintTransactionError::NoTransactionFound)?;

        if let Some(versioned_transaction) = transaction.transaction.transaction.decode() {
            return RaydiumMintedTransaction::parse(&versioned_transaction)
        }

        Err(MintTransactionError::CouldNotParseTransaction)
    }
    fn parse(transaction: &VersionedTransaction) -> Result<MintTransaction, MintTransactionError> {
        let keys = transaction.message.static_account_keys();
        if keys.len() != 22 {
            return Err(MintTransactionError::NotEnoughKeys);
        }

        // Use to debug the order of the keys inputted, if something seems off
        /* for (index, solana_address) in keys.iter().enumerate() {
            debug!("{}: {}", index, solana_address);
        } */

        /* eval token addresses */
        let base_coin;
        let token_coin;

        if keys[13].to_string() == "So11111111111111111111111111111111111111112" {
            base_coin = pubkey!("So11111111111111111111111111111111111111112");
            token_coin = keys[18];
        } else if keys[18].to_string() == "So111111111111111111111111111111111111111111" {
            base_coin = pubkey!("So11111111111111111111111111111111111111112");
            token_coin = keys[13];
        } else {
            return Err(MintTransactionError::CantFindTokenAddress);
        }
        let tx = RaydiumMintedTransaction {
            token_program: keys[12],
            spl_associated_token_account: keys[16],
            system_program: keys[11],
            rent_program: keys[14],
            amm: keys[2],
            amm_authority: keys[17],
            amm_open_orders: keys[3],
            lp_mint: keys[4],
            coin_mint: base_coin,
            pc_mint: token_coin,
            pool_coin_token_account: keys[5],
            pool_pc_token_account: keys[6],
            pool_withdraw_queue: keys[7],
            amm_target_orders: keys[19],
            pool_temp_lp: keys[8],
            serum_program: keys[20],
            serum_market: keys[21],
            user_wallet: keys[0],
            user_token_coin: keys[1],
            user_token_pc: keys[9],
            user_lp_token_account: keys[10],
        };
        Ok(
            MintTransaction::RAYDIUM(tx.clone())
        )
    }

    fn get_dex(&self) -> &str {
        "Raydium"
    }

    fn get_mint(&self) -> Pubkey {
        self.pc_mint
    }

    fn mock() -> Self {
        let dummy_program = pubkey!("11111111111111111111111111111111");

        Self {
            token_program: dummy_program,
            spl_associated_token_account: Pubkey::new_unique(),
            system_program: dummy_program,
            rent_program: dummy_program,
            amm: Pubkey::new_unique(),
            amm_authority: Pubkey::new_unique(),
            amm_open_orders: Pubkey::new_unique(),
            lp_mint: Pubkey::new_unique(),
            coin_mint: pubkey!("So11111111111111111111111111111111111111112"), // Mocked as WSOL
            pc_mint: Pubkey::new_unique(), // The "New" token
            pool_coin_token_account: Pubkey::new_unique(),
            pool_pc_token_account: Pubkey::new_unique(),
            pool_withdraw_queue: Pubkey::new_unique(),
            amm_target_orders: Pubkey::new_unique(),
            pool_temp_lp: Pubkey::new_unique(),
            serum_program: dummy_program,
            serum_market: Pubkey::new_unique(),
            user_wallet: Pubkey::new_unique(),
            user_token_coin: Pubkey::new_unique(),
            user_token_pc: Pubkey::new_unique(),
            user_lp_token_account: Pubkey::new_unique(),
        }
    }
}

impl fmt::Display for RaydiumMintedTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "token_program:                {}
spl_associated_token_account: {}
system_program:               {}
rent_program:                 {}
amm:                          {}
amm_authority:                {}
amm_open_orders:              {}
lp_mint:                      {}
coin_mint:                    {}
pc_mint:                      {}
pool_coin_token_account:      {}
pool_pc_token_account:        {}
pool_withdraw_queue:          {}
amm_target_orders:            {}
pool_temp_lp:                 {}
serum_program:                {}
serum_market:                 {}
user_wallet:                  {}
user_token_coin:              {}
user_token_pc:                {}
user_lp_token_account:        {}",
            self.token_program,
            self.spl_associated_token_account,
            self.system_program,
            self.rent_program,
            self.amm,
            self.amm_authority,
            self.amm_open_orders,
            self.lp_mint,
            self.coin_mint,
            self.pc_mint,
            self.pool_coin_token_account,
            self.pool_pc_token_account,
            self.pool_withdraw_queue,
            self.amm_target_orders,
            self.pool_temp_lp,
            self.serum_program,
            self.serum_market,
            self.user_wallet,
            self.user_token_coin,
            self.user_token_pc,
            self.user_lp_token_account
        )
    }
}