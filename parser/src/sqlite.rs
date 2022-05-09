use sqlite;

pub fn initialize() -> () {
    let connection = sqlite::open("/Volumes/SavvyT7Red/Sqlite/btc.db").unwrap();

    connection
        .execute(
            "
        CREATE TABLE blocks (block_hash BLOB, version UNSIGNED INT4);
        INSERT INTO blocks VALUES ('sdfs', 33);
        ",
        )
        .unwrap();

    connection
        .iterate("SELECT * FROM blocks WHERE version > 30", |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{} = {}", column, value.unwrap());
            }
            true
        })
        .unwrap();
}
