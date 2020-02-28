// mod csr;
// mod csr;
mod csr;

use std::collections::HashMap;
// use est_rs::csr::Csr;
// use crate::csr::Csr
// use std::str;

pub use csr::Csr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let csr = Csr::new().unwrap();

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
