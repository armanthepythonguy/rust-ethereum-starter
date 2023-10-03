use std::time::Duration;

use ethers::{
    prelude::{Address, LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256},
    utils::Ganache,
};

use eyre::{ContextCompat, Result};
use hex::ToHex;

#[tokio::main]
async fn main() -> Result<()>{
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("We got the endpoint :- {}", ganache.endpoint());

    let wallet : LocalWallet = ganache.keys()[0].clone().into();
    let first_wallet = wallet.address();
    println!("Wallet Address :- {}", first_wallet.encode_hex::<String>());

    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));
    let first_balance = provider.get_balance(first_wallet, None).await?;
    println!("First account balance is :- {}", first_balance);

    let other_address = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646".parse::<Address>()?;
    let other_balance = provider.get_balance(other_address, None).await?;
    println!("Other account balance is :- {}", other_balance);

    let tx : TransactionRequest = TransactionRequest::pay(other_address, U256::from(1000u64)).from(first_wallet);
    let receipt = provider
        .send_transaction(tx, None)
        .await?
        .log_msg("Pending transfer")
        .confirmations(1)
        .await?
        .context("MIssinf reciept")?;

    println!("Tx has ve successfully mined in {}",
        receipt.block_number.context("Can't get the block number")?
    );

    println!(
        "Balance of the other wallet is :- {}",
        provider.get_balance(other_address, None).await?
    );

    Ok(())
}