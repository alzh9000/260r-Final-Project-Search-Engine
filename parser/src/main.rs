mod output_writer;
mod parser;
mod sqlite;
mod transaction;

fn main() {
    println!("Hello, world!");

    // Change this type to write output in a different format (e.g. SQLiteDriver, or CustomWriter)
    let drainer = sqlite::SQLiteDriver::new();
    let sqlite_driver = Box::new(drainer);

    let mut p = parser::Parser::new(sqlite_driver);

    p.parse();
}
