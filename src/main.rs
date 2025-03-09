
use std::io::Read;

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let client = reqwest::Client::new();
    
    let url = "https://developers.lotto.pl/api/open/v1/lotteries/info/game-jackpot";

    let res = client
        .get(url)
        .send()
        .await?;
    
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    
    Ok(())
}
