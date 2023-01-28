use data_crawling::wics::get_daily_wics_list;

mod data_crawling;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let dates = ["20230123", "20230124", "20230125", "20230126", "20230127"];
    for date in dates {
        get_daily_wics_list(&client, date).await?;
    }

    Ok(())
}
