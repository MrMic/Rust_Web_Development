use std::env;

use clap::Parser;

// ______________________________________________________________________
/// Q&A web service API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "debug")]
    pub log_level: String,
    /// Which PORT the server is listening to
    #[clap(short, long, default_value = "3030")]
    pub port: u16,
    /// Username for the postgres database
    #[clap(long, default_value = "postgres")]
    pub db_user: String,
    /// Password for the postgres database
    #[clap(long)]
    pub db_password: String,
    /// URL for the postgres database
    #[clap(long, default_value = "172.23.0.3")]
    pub db_host: String,
    /// PORT number for the postgres database
    #[clap(long, default_value = "5432")]
    pub db_port: u16,
    /// Database name
    #[clap(long, default_value = "rustwebdev")]
    pub db_name: String,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        dotenv::dotenv().ok();
        let config = Config::parse();

        if let Err(_) = env::var("BAD_WORDS_API_KEY") {
            panic!("BAD_WORDS_API_KEY not set");
        }
        if let Err(_) = env::var("PASETO_KEY") {
            panic!("PASETO_KEY not set");
        }

        let port = std::env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(config.port))
            .map_err(|e| handle_errors::Error::ParseError(e))?;

        let db_user = std::env::var("POSTGRES_USER").unwrap_or(config.db_user.to_owned());
        let db_password = std::env::var("POSTGRES_PASSWORD").unwrap();
        let db_host = std::env::var("POSTGRES_HOST").unwrap_or(config.db_host.to_owned());
        let db_port = std::env::var("POSTGRES_PORT").unwrap_or(config.db_port.to_string());
        let db_name = std::env::var("POSTGRES_DB").unwrap_or(config.db_name.to_owned());

        Ok(Config {
            log_level: config.log_level,
            port,
            db_user,
            db_password,
            db_host,
            db_port: db_port
                .parse::<u16>()
                .map_err(|e| handle_errors::Error::ParseError(e))?,
            db_name,
        })
    }
}
