use parser::custom_format::{load_data_sorted, read_custom_format, CustomWriter};
use parser::transaction::{Block, InputOutputPair, Transaction};
use tonic::{transport::Server, Request, Response, Status};

use search::search_server::{Search, SearchServer};
use search::{Blocks, InputOutputPairs, TargetBlockHashes, TargetTxHashes, Transactions};

pub mod search {
    tonic::include_proto!("search");
}

#[derive(Debug, Default)]
pub struct Searcher {
    txs: Vec<Transaction>,
    blocks: Vec<Block>,
    iopairs_sorted_src: Vec<InputOutputPair>,
    iopairs_sorted_dest: Vec<InputOutputPair>,
}

impl Searcher {
    fn new() -> Searcher {
        let (txs, blocks, iopairs_sorted_src, iopairs_sorted_dest) = load_data_sorted();

        Searcher {
            txs,
            blocks,
            iopairs_sorted_src,
            iopairs_sorted_dest,
        }
    }
}

#[tonic::async_trait]
impl Search for Searcher {
    async fn transactions_by_sources(
        &self,
        request: tonic::Request<TargetTxHashes>,
    ) -> Result<tonic::Response<InputOutputPairs>, tonic::Status> {
        let mut result: Vec<InputOutputPair> = Vec::new();

        for t in request.get_ref().target.iter() {
            find_elements_in_sorted_vec(&self.iopairs_sorted_src, f, t, &mut result);
        }

        Err(tonic::Status::unimplemented(""))
    }

    async fn transactions_by_destinations(
        &self,
        request: tonic::Request<TargetTxHashes>,
    ) -> Result<tonic::Response<InputOutputPairs>, tonic::Status> {
        Err(tonic::Status::unimplemented(""))
    }

    async fn get_transactions(
        &self,
        request: tonic::Request<TargetTxHashes>,
    ) -> Result<tonic::Response<Transactions>, tonic::Status> {
        Err(tonic::Status::unimplemented(""))
    }

    async fn get_blocks(
        &self,
        request: tonic::Request<TargetBlockHashes>,
    ) -> Result<tonic::Response<Blocks>, tonic::Status> {
        Err(tonic::Status::unimplemented(""))
    }
}

// This function finds the elements `x` in `v` that match `F(x) == y` and appends them to
// `collector`. Note that `v` must be pre-sorted in such a way that all the elements that match
// `F(x) == y` must be consecutive, and all of the elements that match `F(x) < y` must be before
// the elements that match `F(x) == y`.
fn find_elements_in_sorted_vec<T, F, Y: Ord>(v: &Vec<T>, f: F, y: Y, collector: &mut Vec<T>) -> ()
where
    F: Fn(&T) -> Y,
{
    let start_index = v.partition_point(|x| f(x) >= y);
    let end_index = v.partition_point(|x| f(x) > y);

    for i in start_index..end_index {
        collector.push(v[i])
    }
}

fn main() {}
