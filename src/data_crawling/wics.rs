#[cfg(test)]
mod test {
    #[tokio::test]
    async fn should_have_valid_json() {
        // Arrange
        let client = reqwest::Client::new();
        // Act
        let result = get_wics(&client).await.unwrap();
        // Assert
        assert!(result.deserialize().is_ok())
    }
}
