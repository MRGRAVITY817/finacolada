#[cfg(test)]
mod test {
    #[tokio::test]
    async fn should_have_valid_corp_code_zip_file() -> anyhow::Result<()> {
        let api_key = std::env::var("OPENDART_API_KEY").unwrap();

        let _ = get_corp_codes(api_key).await.unwrap();
    }
}
