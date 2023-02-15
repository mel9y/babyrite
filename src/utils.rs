use dotenvy::dotenv;
use std::env;

pub fn get_env(key: &str) -> String {
    dotenv().expect(".env file not found");
    env::var(key).unwrap_or_else(|_| panic!("{key} must be set"))
}
