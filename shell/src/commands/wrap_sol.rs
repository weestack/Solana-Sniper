use std::sync::Arc;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_response::transaction::Transaction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_system_interface::instruction::transfer;
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;

#[derive(Debug, clap::Args, Clone)]
pub struct WrapArgs {
    #[arg(long)]
    pub amount: f32,
}

fn rpc(rpc_endpoint: String) -> Arc<RpcClient> {
    Arc::new(RpcClient::new(rpc_endpoint))
}

pub async fn wrap_sol_fn(wrap_amount: u64, keypair: &Arc<Keypair>, rpc_endpoint: String) {
    let wsol_wrap_amount: u64 = wrap_amount;
    let client = rpc(rpc_endpoint);

    let wsol_mint = spl_token::native_mint::id();
    let user_wsol_account = get_associated_token_address(&keypair.pubkey(), &wsol_mint);

    // Check if WSOL account exists
    let wsol_acc_exists = client.get_account(&user_wsol_account).await;

    if wsol_acc_exists.is_err() {
        let create_ata_ix = create_associated_token_account(
            &keypair.pubkey(),
            &keypair.pubkey(),
            &wsol_mint,
            &spl_token::ID,
        );

        let blockhash = client.get_latest_blockhash().await.unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[create_ata_ix],
            Some(&keypair.pubkey()),
            &[keypair.as_ref()],
            blockhash,
        );

        let signature = client.send_transaction(&tx).await.unwrap();
        println!("Created WSOL account: {signature:?}");
    }

    let balance = match client.get_token_account_balance(&user_wsol_account).await {
        Ok(bal) => bal.amount.parse::<u64>().unwrap_or(0),
        Err(_) => 0,
    };

    if balance < wsol_wrap_amount {
        let transfer_amt = wsol_wrap_amount - balance;
        let blockhash = client.get_latest_blockhash().await.unwrap();

        let transfer_instruction = transfer(
            &keypair.pubkey(),
            &user_wsol_account,
            transfer_amt,
        );

        let sync_instruction =
            spl_token::instruction::sync_native(&spl_token::ID, &user_wsol_account).unwrap();

        let tx = Transaction::new_signed_with_payer(
            &[transfer_instruction, sync_instruction],
            Some(&keypair.pubkey()),
            &[keypair.as_ref()],
            blockhash,
        );

        let signature = client.send_transaction(&tx).await.unwrap();
        println!("Wrapped SOL signature: {signature:?}");
    } else {
        println!("WSOL account already has sufficient balance: {balance} >= {wsol_wrap_amount}");
    }
}
