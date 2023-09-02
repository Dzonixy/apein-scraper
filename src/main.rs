use env_logger::{Builder, WriteStyle};
use ethers::prelude::*;
use ethers::types::Filter;
use log::LevelFilter;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

const RPC_URL: &'static str = "https://eth.llamarpc.com";
const WSS_URL: &'static str = "wss://eth-mainnet.g.alchemy.com/v2/jdL1Z4WEHYrliONc3q_TNJ5q8l3FV1cj";

// abigen!(
//     IUniswapV2Pair,
//     r#"[function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)]"#
// );

pub struct AbiStorage {
    pub abis: HashMap<Address, abi::Abi>,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> eyre::Result<()> {
    let filter_level = std::env::var("RUST_LOG").unwrap_or("info".to_string());
    Builder::new()
        .filter(
            None,
            filter_level
                .parse::<LevelFilter>()
                .unwrap_or(LevelFilter::Info),
        )
        .write_style(WriteStyle::Always)
        .init();

    // let function_selector = &ethers::utils::id("getReserves()")[0..4];

    let provider = Arc::new(Provider::<Ws>::connect_with_reconnects(WSS_URL, 10).await?);
    let http_provider = Provider::try_from(RPC_URL)?;

    // let mut abis: HashMap<Address, abi::Abi> = HashMap::new();

    let mut stream = provider.subscribe_logs(&Filter::new()).await?;
    // let swap_event_id: Vec<u8> =
    //     ethers::utils::id("Swap(address,uint256,uint256,uint256,uint256,address)").to_vec();
    while let Some(message) = stream.next().await {
        let _a = message;
    }
    // let rpc_provider = Provider::<Http>::connect(RPC_URL).await;
    // let data = log.data;
    // let tx = http_provider
    // .get_transaction(message.transaction_hash.unwrap())
    // .await;
    // log::info!("{:#?}", &i);
    // log::info!("{:#?}", i);
    // if let Some(tx) = transaction_provider
    //     .get_transaction(log.transaction_hash.unwrap())
    //     .await?
    // {
    //     log::info!("{:#?}", i);
    //     i += 1;
    // }

    //     log::debug!("Transaction found");
    //     if let Some(contract_address) = tx.to {
    //         let option_abi = if let Some(abi) = abis.get(&contract_address) {
    //             log::debug!("ABI for contract {:?} found in cache", contract_address);
    //             Some(abi.clone())
    //         } else {
    //             let abi_url = format!(
    //                 "https://api.etherscan.io/api\
    //                 ?module=contract\
    //                 &action=getabi\
    //                 &address={:?}\
    //                 &apikey=6NJHDJ7CJPSJ4ITC81HB4UJB49J29M2HSM",
    //                 contract_address
    //             );
    //             let response = reqwest::get(&abi_url)
    //                 .await?
    //                 .json::<HashMap<String, String>>()
    //                 .await?;
    //
    //             if let Some(result) = response.get("result") {
    //                 if let Ok(parsed_abi) = serde_json::from_str::<abi::Abi>(result) {
    //                     log::debug!("ABI for contract {:?} added to cache", contract_address);
    //                     abis.insert(contract_address, parsed_abi.clone());
    //                     Some(parsed_abi)
    //                 } else {
    //                     log::debug!("Failed to parse found ABI");
    //                     None
    //                 }
    //             } else {
    //                 log::debug!("No ABI found");
    //                 None
    //             }
    //         };
    //         if let Some(unpacked_abi) = &option_abi {
    //             log::debug!("Events in abi: \n{:#?}", unpacked_abi.events);
    //         }
    //     }
    // }
    // }

    Ok(())
}
