/// Return result of (trade_date, closing_value)
pub async fn get_stock_price_by_ticker_and_date_range(
    query_client: &reqwest::Client,
    ticker: &str,
    start_date: &str,
    end_date: &str,
) -> anyhow::Result<Vec<(String, u32)>> {
    let url = "https://api.finance.naver.com/siseJson.naver";
    // 1. Build the api call
    let result = query_client
        .get(url)
        .query(&[
            ("symbol", ticker),
            ("requestType", "1"),
            ("startTime", start_date),
            ("endTime", end_date),
            ("timeframe", "day"),
        ])
        .send()
        .await?
        .text()
        .await?;
    // 2. Pre-process the text
    let preprocessed = result
        .replace("\"", "'")
        .replace("[", "\"[")
        .replace("]", "]\"")
        .trim()
        .trim_matches('"')
        .to_string();
    // 3. Get string vectors
    let vectorized: Vec<String> = ron::from_str(&preprocessed)?;
    let result = vectorized[1..]
        .into_iter()
        .map(|row| -> anyhow::Result<(String, u32)> {
            let preprocessed = row.replace("[", "(").replace("]", ")").replace("'", "\"");
            let row_tuple: (String, u32, u32, u32, u32, u32, f32) = ron::from_str(&preprocessed)?;
            Ok((row_tuple.0, row_tuple.4))
        })
        .collect::<anyhow::Result<Vec<(String, u32)>>>()?;

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

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
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], ("20221102".to_string(), 59600));
        assert_eq!(result[1], ("20221103".to_string(), 59200));
        assert_eq!(result[2], ("20221104".to_string(), 59400));
    }
}
