use futures_util::Future;
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Debug, Deserialize)]
pub struct EthSubscription {
    pub jsonrpc: String,
    pub method: Option<String>,
    pub params: Option<Params>,
    pub subscription: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub result: ResultData,
}

#[derive(Debug, Deserialize)]
pub struct ResultData {
    pub removed: bool,
    pub transaction: ethers::types::Transaction,
}
