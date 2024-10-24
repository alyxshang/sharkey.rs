/*
Sharkey.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing this crate's
/// CLI function
#[cfg(feature="cli")]
use sharkey::cli;

/// The main point of
/// entry for the Rust
/// compiler.
#[tokio::main]
async fn main(){
    #[cfg(feature="cli")]
    match cli().await{
        Ok(feedback) => println!("{}", feedback),
        Err(e) => eprintln!("{}", e.to_string())
    };
}
