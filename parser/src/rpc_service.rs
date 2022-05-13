use crate::transaction::{Block, BlockHash, InputOutputPair, Transaction, TxHash};

use futures::{
    future::{self, Ready},
    prelude::*,
};
use tarpc::{
    client, context,
    server::{self, incoming::Incoming, Channel},
};

#[tarpc::service]
pub trait Search {
    async fn transactions_by_sources(targets: Vec<TxHash>) -> Vec<InputOutputPair>;
    async fn transactions_by_destinations(targets: Vec<TxHash>) -> Vec<InputOutputPair>;
    async fn get_transactions(targets: Vec<TxHash>) -> Vec<Transaction>;
    async fn get_blocks(targets: Vec<BlockHash>) -> Vec<Block>;
}
