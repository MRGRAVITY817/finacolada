// symbol: 005930
// requestType: 1
// startTime: 20210821
// endTime: 20220506
// timeframe: day

pub async fn get_stock_price_by_ticker_and_date_range(
    query_client: &reqwest::Client,
    ticker: &str,
    start_date: &str,
    end_date: &str,
) -> anyhow::Result<String> {
    // start_date or end_date shouldn't be in future
    // start_date shouldn't be later than end_date
    // 1. Build the api call
    // 2. Return the text
    Ok("result".to_string())
}

#[cfg(test)]
mod test {
    use {super::*, insta::assert_snapshot};

    #[tokio::test]
    async fn get_3_days_of_samsung_stock_price() {
        // Arrange
        let client = reqwest::Client::new();
        let samsung_ticker = "005930";
        let start_date = "20221102";
        let end_date = "20221104";
        // Act
        let result =
            get_stock_price_by_ticker_and_date_range(&client, samsung_ticker, start_date, end_date)
                .await
                .unwrap();
        // Assert
        assert_snapshot!(result, @"")
    }
}
