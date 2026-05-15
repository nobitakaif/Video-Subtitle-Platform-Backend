use std::env;

pub struct Config {
    pub connection_url : String
}

impl Default for Config {
    fn default() -> Self {
        let connection_url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL is missing in "));
        print!("connection url => {connection_url}");
        Self {
            connection_url 
        }
    }
}