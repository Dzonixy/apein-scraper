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

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let value = Value::from(text.clone());

                if let Value::String(string_value) = &value {
                    let subscription_result =
                        serde_json::from_str::<EthSubscription>(string_value)?;

                    match subscription_result.params {
                        Some(params) => {
                            info!("Received transaction: {:?}", params.result.transaction);
                            insert_transaction(&pool, &params.result.transaction).await?;
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

pub async fn insert_transaction(
    pool: &PgPool,
    transaction: &Transaction,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO transactions (
        block_hash, 
        block_number
    )
    VALUES (
        $1, $2
    )
    "#,
        transaction.block_hash.unwrap().to_fixed_bytes().to_vec(),
        transaction.block_number.unwrap().as_u32() as i32,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
