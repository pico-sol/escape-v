use anyhow::Result;
use std::sync::Arc;
use std::str::FromStr;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    commitment_config::CommitmentConfig,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use spl_associated_token_account::get_associated_token_address;
use std::cmp::min;
use solana_sdk::program_pack::Pack;

struct Ctx {
    rpc: RpcClient,
    payer: Keypair,
}
// ご自身のRPCエンドポイントを入力
const RPC: &str = "https://mainnet.helius-rpc.com/?api-key=*******************";
// 預入れまたは引出しtxの#2のアカウントをコピーする
const ACC2 : &str = "*******************";
// id.jsonの中に[***,***,...,***]形式の秘密鍵を貼り付ける
const KEYPAIR_FILE: &str = "./id.json";
// 預入れたUSDCの数量 x 1,000,000 エラーログの"Program log: Max withdraw: ******に表示される値"
// わからないときはu64::MAXにしておくと、vaultkaのUSDC vaultの全額を引き出そうとするtxになります。ただし、預入残高よりvaultkaのvault内のUSDCの方が大きいとエラーになります
const DEPOSITED_AMOUNT: u64 = u64::MAX;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = Arc::new(init().await?);
    loop {
        // VaultkaのUSDCプールの残高を表示
        let usdc_balance_data = ctx.rpc.get_account_data(&Pubkey::from_str("E19zKjNZhWhvHfHao89Pk9xQ2zSr6DErGSaFnddyuY3A")?).await?;
        let balance = spl_token::state::Account::unpack_from_slice(&usdc_balance_data).unwrap().amount;
        println!("vaultka balance: {:?} USDC", balance as f64 / 1_000_000_f64);
        let balance = if balance < 176000 {
            0
        } else {
            balance - 176000
        };
        let withdraw_amount = min(balance, DEPOSITED_AMOUNT);
        send_ix(ctx.clone(), withdraw_amount).await?;
        //10秒待機
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
    // Ok(())
}

async fn init() -> Result<Ctx> {
    std::env::set_var("RUST_LOG", "info");
    // [***,***,...,***]（数値）形式の秘密鍵をid.jsonに貼り付ける
    let payer = solana_sdk::signature::read_keypair_file(&*shellexpand::tilde(&KEYPAIR_FILE)).expect("parse keypair file");
    // base58型(文字列)の秘密鍵を使う場合は53行目をコメントアウトし55-56行目を有効にしてください。id.jsonファイルの編集は不要です
    // let secret_key = "*************";
    // let payer = Keypair::from_base58_string(secret_key);
    let rpc = RpcClient::new_with_commitment(
        RPC.to_string(),
        CommitmentConfig::finalized(),
    );
    Ok(
        Ctx {
        rpc,
        payer,
        }
    )
}

async fn send_ix(ctx: Arc<Ctx>, balance: u64) -> Result<()> {
    let mut ix = Vec::new();
    let price = 0;
    let budget = 200000;
    let budget_ix1 =
        solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_price(price);
    let budget_ix2 =
        solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(budget);
    let escape_ix = escape_ix(ctx.clone(), balance)?;
    ix.push(budget_ix1.clone());
    ix.push(budget_ix2.clone());
    ix.push(escape_ix);
    let blockhash = ctx.rpc.get_latest_blockhash().await?;
    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &ix,
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer],
        blockhash,
    );
    let skip_preflight = false;
    let commitment = CommitmentConfig::confirmed();
    let config = solana_client::rpc_config::RpcSendTransactionConfig {
        skip_preflight,
        preflight_commitment: None,
        encoding: None,
        max_retries: None,
        min_context_slot: None,
    };
    match ctx.rpc.send_and_confirm_transaction_with_spinner_and_config(&tx, commitment, config).await {
        Ok(sig) => {
            println!("send tx; {:?}", sig);
            // panic!("******");
        }
        Err(_err) => {
            println!("error: {:?}", _err);
            // panic!("******");
        }
    }
    Ok(())
}

fn escape_ix(ctx: Arc<Ctx>, balance: u64) -> Result<Instruction> {
    let usdc = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")?;
    let vault = Pubkey::from_str("DefkwTSvkHeZASCuaVJ8AxUWS6zvBCwrLFpW2FniLSWo")?;
    let fee_acc = Pubkey::from_str("CfsZxi7XH9N1LtWjX7R61HyaombyPRrT5ieDpxQZif9U")?;
    let program_id = Pubkey::from_str("nKMLJtN1rr64K9DjmfzXvzaq4JEy5a4AJHHP9gY1dW6")?;
    let acc1 = ctx.payer.pubkey();
    let acc2 = Pubkey::from_str(ACC2)?;
    let acc3 = Pubkey::from_str("F2kJ9Mx6d7KkJogbLeJC7ir4td2heyDPa7qvY3kzyqpa")?;
    let acc4 = solana_sdk::system_program::id();
    let acc5 = get_associated_token_address(&fee_acc, &usdc);
    let acc6 = get_associated_token_address(&vault, &usdc);
    let acc7 = get_associated_token_address(&ctx.payer.pubkey(), &usdc);
    let acc8 = spl_token::id();
    let acc9 = vault;
    let acc10 = usdc;
    let mut data = Vec::new();

    println!("acc1: {:?}", acc1);
    println!("acc2: {:?}", acc2);
    println!("acc3: {:?}", acc3);
    println!("acc4: {:?}", acc4);
    println!("acc5: {:?}", acc5);
    println!("acc6: {:?}", acc6);
    println!("acc7: {:?}", acc7);
    println!("acc8: {:?}", acc8);
    println!("acc9: {:?}", acc9);
    println!("acc10: {:?}", acc10);

    data.extend_from_slice(&0x9c4612b7_u32.to_le_bytes());
    data.extend_from_slice(&0x22a16d94_u32.to_le_bytes());
    data.extend_from_slice(&balance.to_le_bytes());
    let ix = solana_sdk::instruction::Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(acc1, true),
            AccountMeta::new(acc2, false),
            AccountMeta::new(acc3, false),
            AccountMeta::new_readonly(acc4, false),
            AccountMeta::new(acc5, false),
            AccountMeta::new(acc6, false),
            AccountMeta::new(acc7, false),
            AccountMeta::new_readonly(acc8, false),
            AccountMeta::new(acc9, false),
            AccountMeta::new_readonly(acc10, false),
        ],
        data,
    };
    Ok(ix)
}
