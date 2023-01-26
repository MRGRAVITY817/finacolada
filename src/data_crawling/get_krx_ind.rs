#[cfg(test)]
mod test {
    #[tokio::test]
    async fn get_generated_otp_from_krx() {
        // Arrange
        // Act
        let result = get_krx_otp().await;
        // Assert
        assert!(result)
    }
}
