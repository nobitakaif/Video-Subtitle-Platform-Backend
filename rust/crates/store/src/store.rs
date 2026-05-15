

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, ConnectionError, PgConnection};


use crate::config::Config;

#[derive(Clone)]
pub struct Store{
    pub pool : Pool<ConnectionManager<PgConnection>>
}

impl Store {
    // pub fn default() -> Result<Self, ConnectionError> {
    //     let config = Config::default();
    //     let diesel = PgConnection::establish(&config.connection_url)?;
    //     Ok(Self{
    //         pool : diesel
    //     })
    // }
    pub fn new(database_url : &str) -> Self{
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
                        .build(manager).expect("Failed to create Pool");
        Self { pool }
    }
}