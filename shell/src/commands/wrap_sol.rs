use std::sync::Arc;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair};
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use spl_token_client::client::{ProgramClient, ProgramRpcClient, ProgramRpcClientSendTransaction};
use spl_token_client::token::Token;

#[derive(Debug, clap::Args, Clone)]
pub struct WrapArgs {
    #[arg(
            long,
    )]
    pub amount: f32,
}

fn rpc(rpc_endpoint: String) -> Arc<RpcClient> {
    Arc::new(RpcClient::new(rpc_endpoint.to_string()))
}
fn program_rpc(rpc: Arc<RpcClient>) -> Arc<dyn ProgramClient<ProgramRpcClientSendTransaction>> {
    let program_client: Arc<dyn ProgramClient<ProgramRpcClientSendTransaction>> = Arc::new(
        ProgramRpcClient::new(rpc.clone(), ProgramRpcClientSendTransaction),
    );
    program_client
}

fn keypair_clone(kp: &Keypair) -> Keypair {
    Keypair::from_bytes(&kp.to_bytes()).expect("failed to copy keypair")
}

pub async fn wrap_sol_fn(wrap_amount: u64, keypair: &Arc<Keypair>, rpc_endpoint: String) {
    let wsol_wrap_amount: u64 = wrap_amount;
    let client = rpc(rpc_endpoint);
    let program_client = program_rpc(Arc::clone(&client));

    /* prepare the wSOL account */
    let in_token_client = Token::new(
        Arc::clone(&program_client),
        &spl_token::ID,
        &spl_token::native_mint::id(),
        None,
        Arc::new(keypair_clone(&keypair)),
    );

    let user_in_token_account = in_token_client.get_associated_token_address(&keypair.pubkey());
    let wsol_acc_exists = in_token_client
        .get_account_info(&user_in_token_account)
        .await;

    /* ensure wsol token program has not been closed! */
    if wsol_acc_exists.is_err() {
        in_token_client.create_associated_token_account(
            &keypair.pubkey()
        ).await.unwrap();
    }

    let user_in_acct = in_token_client
        .get_account_info(&user_in_token_account)
        .await.unwrap();


    let balance = user_in_acct.base.amount;
    if in_token_client.is_native() && balance < wsol_wrap_amount {
        let transfer_amt = wsol_wrap_amount - balance;
        let blockhash = client.get_latest_blockhash().await.unwrap();
        let transfer_instruction = solana_sdk::system_instruction::transfer(
            &keypair.pubkey(),
            &user_in_token_account,
            transfer_amt,
        );
        let sync_instruction =
            spl_token::instruction::sync_native(&spl_token::ID, &user_in_token_account).unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[transfer_instruction, sync_instruction],
            Some(&keypair.pubkey()),
            &[&keypair],
            blockhash,
        );

        let signature = client.send_transaction(&tx).await.unwrap();
        println!("signature {signature:?}");
    }
}
