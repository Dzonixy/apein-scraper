use ethereum::build_log;
use ethereum::configurations::get_configuration;
use ethereum::subscription::EthSubscription;
use ethers::types::Transaction;
use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use serde_json::Value;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

const WSS_URL: &'static str = "wss://eth-mainnet.g.alchemy.com/v2/jdL1Z4WEHYrliONc3q_TNJ5q8l3FV1cj";

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> eyre::Result<()> {
    build_log();
    let url = Url::parse(WSS_URL)?;

    let (stream, _) = connect_async(url).await?;
    let (mut write, mut read) = stream.split();

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let pool = actix_web::web::Data::new(connection_pool);

    let request = r#"{"jsonrpc":"2.0","id": 2, "method": "eth_subscribe", "params": ["alchemy_minedTransactions"]}"#;
    write.send(Message::Text(request.to_string())).await?;

    let r = read.next();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let value = Value::from(text.clone());

                if let Value::String(string_value) = &value {
                    let subscription_result =
                        serde_json::from_str::<EthSubscription>(string_value)?;

                    match subscription_result.params {
                        Some(params) => {
                            info!("Received transaction: {:#?}", params.result.transaction);
                            // insert_trader(&pool, &params.result.transaction).await?;
                            // insert_transaction(&pool, &params.result.transaction).await?;
                        }
                        None => continue,
                    }
                }
            }

            Ok(Message::Binary(bin)) => info!("Received binary: {:?}", bin),
            Err(e) => error!("Error receiving message: {:?}", e),
            _ => {}
        }
    }
    Ok(())
}

// pub async fn insert_trader(pool: &PgPool, transaction: &Transaction) -> Result<(), sqlx::Error> {
//     let tx = transaction.clone();
//     let trader = tx.from.to_fixed_bytes();
//     let trader_name = "";
//
//     sqlx::query!(
//         r#"
//             INSERT INTO traders (public_key, trader_name)
//             VALUES ($1, $2)
//             ON CONFLICT (public_key) DO NOTHING;
//         "#,
//         trader.as_ref(),
//         trader_name
//     )
//     .execute(pool)
//     .await
//     .map_err(|e| {
//         tracing::error!("Failed to execute query: {:?}", e);
//         e
//     })?;
//
//     Ok(())
// }
//
// pub async fn insert_transaction(
//     pool: &PgPool,
//     transaction: &Transaction,
// ) -> Result<(), sqlx::Error> {
//     let tx = transaction.clone();
//
//     let hash = tx.hash.to_fixed_bytes();
//     let from = tx.from.to_fixed_bytes();
//     let block_hash = tx.block_hash.unwrap().to_fixed_bytes();
//
//     sqlx::query!(
//         r#"
//             INSERT INTO transactions (
//                 transaction_hash,
//                 wallet_public_key,
//                 transaction_data,
//                 nonce,
//                 block_hash,
//                 block_number
//             )
//             VALUES ($1, $2, $3, $4, $5, $6)
//         "#,
//         hash.as_ref(),
//         from.as_ref(),
//         &tx.input.as_ref(),
//         tx.nonce.as_u64() as i64,
//         block_hash.as_ref(),
//         tx.block_number.unwrap().as_u64() as i64
//     )
//     .execute(pool)
//     .await
//     .map_err(|e| {
//         tracing::error!("Failed to execute query: {:?}", e);
//         e
//     })?;
//     Ok(())
// }
