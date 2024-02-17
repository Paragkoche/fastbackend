use diesel::{
    r2d2::{ConnectionManager, Pool},
    Connection, SqliteConnection,
};

use dotenv;
use std::env;
pub type SQLPOOL = Pool<ConnectionManager<SqliteConnection>>;

pub fn connection() -> SqliteConnection {
    dotenv::dotenv().ok().expect("msg");
    let db_url = env::var("DATABASE_URL").expect("msg");
    SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {} ", db_url))
}
