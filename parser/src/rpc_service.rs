use crate::transaction::{Block, BlockHash, InputOutputPair, Transaction, TxHash};

pub const PORT: u16 = 6969;

#[tarpc::service]
pub trait Search {
    async fn transactions_by_sources(targets: Vec<TxHash>) -> Vec<InputOutputPair>;
    async fn transactions_by_destinations(targets: Vec<TxHash>) -> Vec<InputOutputPair>;
    async fn get_transactions(targets: Vec<TxHash>) -> Vec<Transaction>;
    async fn get_blocks(targets: Vec<BlockHash>) -> Vec<Block>;
}
