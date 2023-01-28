use data_crawling::wics::{get_wics, SECTOR_CODE_LIST};

mod data_crawling;
mod utils;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let date = "20230127";
    for sector in SECTOR_CODE_LIST {
        let result = get_wics(&client, date, sector).await.unwrap();
        std::fs::write(format!("assets/wics/wics_{sector}_{date}.json"), result).unwrap();
    }
}
