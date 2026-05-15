use diesel::{Connection, ConnectionError, PgConnection};

use crate::config::Config;

pub struct Store{
    pub diesel : PgConnection
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let diesel = PgConnection::establish(&config.connection_url)?;
        Ok(Self{
            diesel
        })
    }
}