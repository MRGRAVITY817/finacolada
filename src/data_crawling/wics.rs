pub async fn get_wics(
    query_client: &reqwest::Client,
    date: &str,
    sector_code: &str,
) -> Result<String, reqwest::Error> {
    // 1. Request the WICS data with URL & params
    let url = "https://www.wiseindex.com/Index/GetIndexComponets";
    query_client
        .get(url)
        .query(&[("ceil_yn", "0"), ("dt", date), ("sec_cd", sector_code)])
        .send()
        .await?
        .text()
        .await
}

#[cfg(test)]
mod test {
    use {super::*, insta::assert_snapshot};

    #[tokio::test]
    async fn should_have_json_text() {
        // Arrange
        let client = reqwest::Client::new();
        let date = "20230126";
        let sector_code = "G10";
        // Act
        let result = get_wics(&client, date, sector_code).await.unwrap();
        // Assert
        assert_snapshot!(result)
    }
}
