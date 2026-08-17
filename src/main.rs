use reqwest::{self, Error};
mod structs;
mod functions;
use functions::input;

#[tokio::main]
async fn main() -> Result<(), Error> {
   let url =  input();
    let page_request = reqwest::get(url).await?.text().await?;
    println!("{page_request:?}");
    Ok(())
}
