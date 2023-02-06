pub async fn get_corp_codes(
    query_client: &reqwest::Client,
    api_key: &str,
    output_path: Option<&str>,
) -> anyhow::Result<()> {
    let result = query_client
        .get("https://opendart.fss.or.kr/api/corpCode.xml")
        .query(&[("crtfc_key", api_key)])
				.header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.77 Safari/537.36")
        .send()
        .await?
        .bytes()
        .await?;

    match output_path {
        Some(path) => std::fs::write(path, &result),
        None => std::fs::write("assets/corp_codes/corp_code.zip", &result),
    }
    .map_err(|_| anyhow::Error::msg("Cannot save the corporate codes"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn should_have_valid_corp_code_zip_file() {
        let client = reqwest::Client::new();
        let api_key = std::env::var("OPENDART_API_KEY").unwrap();
        let output_path = "examples/corp_codes.zip";

        get_corp_codes(&client, &api_key, Some(output_path))
            .await
            .unwrap();

        assert!(std::fs::read(output_path).is_ok())
    }
}
