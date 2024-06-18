use std::{fmt::Error, process::exit};

use chrono::Utc;
use dotenv;
use ethers::providers::{Http, Middleware, Provider};

async fn get_gas_price_at_block(block_number: ethers::types::BlockNumber) -> Result<u64, Error> {
    let provider: Provider<Http> = ethers::providers::Provider::<Http>::try_from(String::from(dotenv::var("ARB_RPC_URL").unwrap())).unwrap();
    let transactions = provider.get_block_receipts(block_number).await.unwrap();
    let mut total_gas_price: u64 = 0;
    for tx in &transactions {
        let gas_price = tx.effective_gas_price.unwrap().as_u64();
        total_gas_price += gas_price;
    }
    let avg_gas_price = total_gas_price / transactions.len() as u64;
    println!("Block number {:?} Gas Price {:?}", block_number, avg_gas_price);
    Ok(avg_gas_price)
}

async fn fetch_gas_prices(
    start_time: u64,
    interval_secs: u64,
    intervals: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let client: ethers::etherscan::Client = ethers::etherscan::Client::builder()
        .with_api_key(dotenv::var("ARB_API_KEY").unwrap())
        .chain(ethers::types::Chain::Arbitrum).unwrap().build().unwrap();

    for i in 0..intervals {
        let timestamp: u64 = start_time + (i as u64 * interval_secs);
        let block_number = match client.get_block_by_timestamp(timestamp, "after").await {
            Ok(bn) => bn,
            Err(e) => {
                eprintln!("{:?}", e);
                exit(0);
            }
        };
        let block_number = block_number.block_number;
        
        let gas_price: u64 = get_gas_price_at_block(block_number).await?;
        println!(
            "Timestamp: {}, Block: {}, Gas Price: {} wei",
            timestamp, block_number, gas_price
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let now: u64 = Utc::now().timestamp().try_into().unwrap();
    let seven_days_ago: u64 = now - (7 * 24 * 60 * 60);
    let interval_secs: u64 = 40;
    let intervals: u64 = (7 * 24 * 60 * 60) / interval_secs;

    fetch_gas_prices(seven_days_ago, interval_secs, intervals as usize).await?;

    Ok(())
}
