use dotenvy::dotenv;
use std::env;

/// Returns the value of the environment variable for the Key specified in the first argument.
/// - If the `.env` file does not exist, an error is returned.
/// - If the value does not exist, an error is raised and the Key for which the value does not exist is reported by cargo.
///
/// ### Example:
/// ```
/// async fn main() {
///    println!("{}", get_env("HOSTNAME"))
/// }
/// ```
pub fn get_env(key: &str) -> String {
    dotenv().expect(".env file not found");
    env::var(key).unwrap_or_else(|_| panic!("{key} must be set"))
}
