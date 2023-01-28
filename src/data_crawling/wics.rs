const SECTOR_CODE_LIST: [&str; 10] = [
    "G25", "G35", "G50", "G40", "G10", "G20", "G55", "G30", "G15", "G45",
];

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

    #[tokio::test]
    async fn different_date_should_have_different_data() {
        // Arrange
        let client = reqwest::Client::new();
        let early_date = "20230126";
        let now_date = "20230127";
        let sector_code = "G10";
        // Act
        let early_result = get_wics(&client, early_date, sector_code).await.unwrap();
        let now_result = get_wics(&client, now_date, sector_code).await.unwrap();
        // Assert
        assert_ne!(early_result, now_result)
    }
}
