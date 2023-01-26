// #type_0 > div > ul.subtop_chart_note > li > span
#[cfg(test)]
mod test {
    use std::str::FromStr;

    use chrono::NaiveDate;

    #[tokio::test]
    async fn fetched_working_date_is_valid_date() {
        // Arrange
        let client = reqwest::Client::new();
        // Act
        let result = get_latest_working_date().await.unwrap();
        // Assert
        assert!(NaiveDate::from_str(result).is_ok())
    }

    // #[tokio::test]
    // async fn workdate_is_within_5_days() {}
}
