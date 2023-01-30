#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    #[tokio::test]
    async fn has_4_tables() {
        // Arrange
        let client = reqwest::Client::new();
        let samsung_ticker = "005930";
        // Act
        let result = get_financial_statement(&client, samsung_ticker)
            .await
            .unwrap();
        // Assert
        assert_eq!(result.len(), 6);
        assert_snapshot!(result[0], @"");
        assert_snapshot!(result[1], @"");
        assert_snapshot!(result[2], @"");
        assert_snapshot!(result[3], @"");
        assert_snapshot!(result[4], @"");
        assert_snapshot!(result[5], @"");
    }
}
