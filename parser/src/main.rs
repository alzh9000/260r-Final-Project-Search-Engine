use clap::{ArgEnum, Parser};
use parser::custom_format::{sort_and_write_data, CustomWriter};

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(arg_enum, short, long, default_value = "dump-distributed-custom-dbs")]
    operation: Operation,

    #[clap(short, long, default_value = "0")]
    for_num_workers: usize,

    #[clap(short, long, default_value = "1")]
    dat_files_to_parse: u32,
}

#[derive(Clone, ArgEnum, Debug)]
enum Operation {
    DumpSqlite,
    DumpUnsortedCustomDB,
    DumpDistributedCustomDbs,
}

fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    println!("Parsing with configuration {:?}", args);

    match args.operation {
        Operation::DumpSqlite => {
            if args.for_num_workers != 0 {
                panic!("for_num_workers specified but has no effect unless Operation chosen in DumpDistributedCustomDBs!")
            }
            let sqlite_connection = rusqlite::Connection::open("btc-test.db").unwrap();
            let mut sqlite_drainer = parser::sqlite::SQLiteDriver::new(&sqlite_connection);
            let mut p = parser::parser::Parser::new(&mut sqlite_drainer);
            p.parse(args.dat_files_to_parse);
        }
        Operation::DumpUnsortedCustomDB => {
            if args.for_num_workers != 0 {
                panic!("for_num_workers specified but has no effect unless Operation chosen in DumpDistributedCustomDBs!")
            }
            let mut custom_drainer = CustomWriter::new();
            let mut p = parser::parser::Parser::new(&mut custom_drainer);
            p.parse(args.dat_files_to_parse);
        }
        Operation::DumpDistributedCustomDbs => {
            if args.for_num_workers < 1 {
                panic!("for_num_workers less than 1 with DumpDistributedCustomDbs operation doesn't make much sense (note that default value is 0)!")
            }
            let mut custom_drainer = CustomWriter::new();
            let mut p = parser::parser::Parser::new(&mut custom_drainer);
            p.parse(args.dat_files_to_parse);
            sort_and_write_data(args.for_num_workers);
        }
    }
}
