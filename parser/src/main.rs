use std::rc::Rc;

mod parser;
mod sqlite;
mod transaction;

fn main() {
    println!("Hello, world!");
    let sqlite_driver = sqlite::initialize();
    let sqlite_driver = Rc::new(sqlite_driver);

    let sqlite_driver1 = sqlite_driver.clone();
    let itx = move |x| {
        sqlite::insert_tx(&sqlite_driver1, x);
    };

    let sqlite_driver2 = sqlite_driver.clone();
    let btx = move |x| {
        sqlite::insert_block(&sqlite_driver2, x);
    };

    let sqlite_driver3 = sqlite_driver.clone();
    let ptx = move |x| {
        sqlite::insert_iopair(&sqlite_driver3, x);
    };

    let mut p = parser::Parser::new(itx, btx, ptx);

    p.parse();
}
