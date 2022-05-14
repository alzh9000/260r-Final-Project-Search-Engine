use clap::{ArgEnum, Parser};
use parser::custom_format::{sort_data, CustomWriter};

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(arg_enum, short, long, default_value = "Operation::DumpSortedCustomDB")]
    operation: Operation,

    #[clap(short, long, default_value = "1")]
    dat_files_to_parse: u32,
}

#[derive(Clone, ArgEnum, Debug)]
enum Operation {
    DumpSqlite,
    DumpUnsortedCustomDB,
    DumpSortedCustomDB,
}

fn main() {
    println!("Hello, world!");
    let args = Args::parse();

    match args.operation {
        Operation::DumpSqlite => {
            let sqlite_connection = rusqlite::Connection::open("btc-test.db").unwrap();
            let mut sqlite_drainer = parser::sqlite::SQLiteDriver::new(&sqlite_connection);
            let mut p = parser::parser::Parser::new(&mut sqlite_drainer);
            p.parse(args.dat_files_to_parse);
        }
        Operation::DumpUnsortedCustomDB => {
            let mut custom_drainer = CustomWriter::new();
            let mut p = parser::parser::Parser::new(&mut custom_drainer);
            p.parse(args.dat_files_to_parse);
        }
        Operation::DumpSortedCustomDB => {
            let mut custom_drainer = CustomWriter::new();
            let mut p = parser::parser::Parser::new(&mut custom_drainer);
            p.parse(args.dat_files_to_parse);
            sort_data();
        }
    }
}
