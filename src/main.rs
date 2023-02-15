use data_crawling::{biz_day::get_latest_biz_day, krx::get_latest_krx_data};

mod data_crawling;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let query_client = reqwest::Client::new();
    let trading_date = get_latest_biz_day(&query_client).await?;

    get_latest_krx_data(&query_client, &trading_date).await
}
