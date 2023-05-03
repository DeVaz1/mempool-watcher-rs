use ethers::prelude::*;
use std::{
    env::{self, VarError},
    process,
    sync::Arc,
    time::Duration,
};

fn get_node_endpoint() -> Result<String, VarError> {
    return env::var("MAINNET_WSS_ENDPOINT");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seaport_address: H160 = "0x00000000000001ad428e4906aE43D8F9852d0dD6".parse()?;
    let ws_endpoint = match get_node_endpoint() {
        Ok(r) => r,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1)
        }
    };

    let provider = Provider::<Ws>::connect(&ws_endpoint)
        .await?
        .interval(Duration::from_millis(1500));
    let provider = Arc::new(provider);

    let tx_provider = provider.clone();
    let mut stream = tx_provider
        .subscribe_pending_txs()
        .await
        .unwrap()
        .transactions_unordered(200);

    while let Some(tx) = stream.next().await {
        let tx = match tx {
            Ok(tx) => tx,
            Err(e) => {
                continue;
            }
        };

        if tx.to != Some(seaport_address) {
            println!("Not SeaPort");
            continue;
        }
        println!("Found SeaPort tx{:?} ", &tx.hash);
    }
    Ok(())
}
