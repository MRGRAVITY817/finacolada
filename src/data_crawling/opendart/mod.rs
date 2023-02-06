async fn get_corp_codes(
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

    let output_path = output_path.unwrap_or("assets/corp_codes.zip");
    std::fs::write(output_path, &result)?;

    let fname = std::path::Path::new(output_path);
    let file = std::fs::File::open(fname)?;

    let mut archive = zip::ZipArchive::new(file)?;
    archive
        .extract(output_path.replace(".zip", ""))
        .map_err(|_| anyhow::Error::msg("Cannot extract corp code zip file"))
}

#[cfg(test)]
mod test {
    use {super::*, insta::assert_snapshot};

    #[tokio::test]
    async fn should_have_valid_corp_code_zip_file() {
        let client = reqwest::Client::new();
        let api_key = std::env::var("OPENDART_API_KEY").unwrap();
        let output_path = "examples/corp_codes.zip";
        let result_path = "examples/corp_codes/CORPCODE.xml";

        get_corp_codes(&client, &api_key, Some(output_path))
            .await
            .unwrap();

        let result = std::fs::read(result_path);
        assert!(result.is_ok());
    }
}
