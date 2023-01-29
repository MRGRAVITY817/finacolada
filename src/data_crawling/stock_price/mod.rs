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
    // 2. Return the text
    Ok(result)
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
        assert_snapshot!(result, @r###"

         [['날짜', '시가', '고가', '저가', '종가', '거래량', '외국인소진율'],

        	
        	
        		
        ["20221102", 59700, 60000, 59300, 59600, 13202919, 49.84],
        		
        ["20221103", 58600, 59800, 58100, 59200, 17492162, 49.87],
        		
        ["20221104", 59100, 59500, 58400, 59400, 12445841, 49.84]
        		
        	

        ]
        "###)
    }
}
