// symbol: 005930
// requestType: 1
// startTime: 20210821
// endTime: 20220506
// timeframe: day

#[cfg(test)]
mod test {
    use {super::*, insta::assert_snapshot};

    #[tokio::test]
    async fn get_3_days_of_samsung_stock_price() {
        // Arrange
        let samsung_ticker = "005930";
        let start_date = Some("20221102");
        let end_date = Some("20221104");
        // Act
        let result = get_stock_price_by_ticker_and_date_range(samsung_ticker, start_date, end_date)
            .await
            .unwrap();
        // Assert
        assert_snapshot!(result, @"")
    }
}
