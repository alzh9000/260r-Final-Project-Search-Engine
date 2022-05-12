mod custom_format;
mod output_writer;
mod parser;
mod sqlite;
mod transaction;

fn main() {
    println!("Hello, world!");

    // Change this type to write output in a different format (e.g. SQLiteDriver, or CustomWriter)
    let sqlite_connection = rusqlite::Connection::open("btc-test.db").unwrap();
    let mut drainer = sqlite::SQLiteDriver::new(&sqlite_connection);

    let mut p = parser::Parser::new(&mut drainer);

    p.parse();
}
