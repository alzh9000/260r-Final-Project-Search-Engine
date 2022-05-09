mod parser;
mod transaction;

fn main() {
    println!("Hello, world!");
    let mut p = parser::Parser::new();
    p.parse();
}
