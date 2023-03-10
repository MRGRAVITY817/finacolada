use {
    anyhow::Result,
    regex::Regex,
    reqwest::Client,
    scraper::{Html, Selector},
};

pub async fn get_latest_biz_day(query_client: &Client) -> Result<String> {
    let url = "https://finance.naver.com/sise/sise_deposit.naver";
    let html_text = query_client
        .get(url)
        .send()
        .await?
        .text_with_charset("EUC-KR")
        .await?;

    // Selector match
    let document = Html::parse_document(&html_text);
    let selector = Selector::parse(r#"#type_0 > div > ul.subtop_chart_note > li > span"#).unwrap();
    for selected in document.select(&selector) {
        let result = selected.inner_html();
        if result.len() > 0 {
            let date_regex = Regex::new("[0-9]+.[0-9]+.[0-9]+")?;
            if let Some(matched) = date_regex.find(&result) {
                return Ok(matched.as_str().replace(".", ""));
            }
        }
        return Ok(result);
    }

    Ok("".to_string())
}

#[cfg(test)]
mod test {
    use {super::*, chrono::NaiveDate, insta::assert_snapshot};

    #[tokio::test]
    async fn fetched_biz_day_is_valid_date() {
        // Arrange
        let client = reqwest::Client::new();
        // Act
        let result = get_latest_biz_day(&client).await.unwrap();
        // Assert
        assert_snapshot!(result, @"20230228");
        assert_eq!(result.len(), 8);
        assert!(NaiveDate::from_ymd_opt(
            result[0..4].parse::<i32>().unwrap(),
            result[4..6].parse::<u32>().unwrap(),
            result[6..8].parse::<u32>().unwrap()
        )
        .is_some())
    }

    // #[tokio::test]
    // async fn workdate_is_within_5_days() {}
}
