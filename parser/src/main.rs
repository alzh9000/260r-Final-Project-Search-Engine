mod reader;
mod transaction;

fn main() {
    println!("Hello, world!");
    let mut p = reader::Matcher::new();
    println!("{}", p.f())
}
