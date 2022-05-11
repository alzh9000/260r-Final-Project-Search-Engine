use crate::sqlite::{insert_block, insert_iopair, insert_tx};

mod parser;
mod sqlite;
mod transaction;

fn main() {
    println!("Hello, world!");
    let mut p = parser::Parser::new(insert_tx, insert_block, insert_iopair);
    p.parse();
    sqlite::initialize();
}
