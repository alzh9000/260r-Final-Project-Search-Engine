use parser::custom_format::{read_custom_formats, sort_data, CustomWriter};

fn main() {
    println!("Hello, world!");

    // Change this type to write output in a different format (e.g. SQLiteDriver, or CustomWriter)
    let mut custom_drainer = CustomWriter::new();

    // let sqlite_connection = rusqlite::Connection::open("btc-test.db").unwrap();
    // let mut sqlite_drainer = sqlite::SQLiteDriver::new(&sqlite_connection);

    let mut p = parser::parser::Parser::new(&mut custom_drainer);

    p.parse(1);

    sort_data();
}
