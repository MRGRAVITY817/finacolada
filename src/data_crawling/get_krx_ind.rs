use {reqwest::Client, std::collections::HashMap};

pub enum MarketType {
    Kospi,
    Kosdaq,
}

pub const GEN_OTP_URL: &'static str = "http://data.krx.co.kr/comm/fileDn/GenerateOTP/generate.cmd";
pub const SECTOR_DOWNLOAD_URL: &'static str =
    "http://data.krx.co.kr/comm/fileDn/download_csv/download.cmd";

pub async fn get_krx_otp(
    trading_date: &str,
    market_type: MarketType,
    query_client: &Client,
) -> Result<String, reqwest::Error> {
    let market_id = match market_type {
        MarketType::Kospi => "STK",
        MarketType::Kosdaq => "KSQ",
    };

    let mut params = HashMap::new();
    params.insert("mktId", market_id);
    params.insert("trdDd", trading_date);
    params.insert("money", "1");
    params.insert("csvxls_isNo", "false");
    params.insert("name", "fileDown");
    params.insert("url", "dbms/MDC/STAT/standard/MDCSTAT03901");

    query_client
        .post(GEN_OTP_URL)
        .form(&params)
        .send()
        .await?
        .text()
        .await
}

pub async fn get_sector_data(otp: &str, query_client: &Client) -> Result<String, reqwest::Error> {
    let mut params = HashMap::new();
    params.insert("code", otp);

    query_client
        .post(SECTOR_DOWNLOAD_URL)
        .form(&params)
        .header("referer", GEN_OTP_URL)
        .send()
        .await?
        .text_with_charset("EUC_KR")
        .await
}

#[cfg(test)]
mod test {
    use {super::*, insta::*, reqwest};

    #[tokio::test]
    async fn get_generated_otp_from_krx_for_kospi() {
        // Arrange
        let trading_date = "20210108";
        let client = reqwest::Client::new();
        let market_type = MarketType::Kospi;
        // Act
        let result = get_krx_otp(trading_date, market_type, &client)
            .await
            .unwrap();
        // Assert
        assert!(result.len() > 10)
    }

    #[tokio::test]
    async fn get_generated_otp_from_krx_for_kosdaq() {
        // Arrange
        let trading_date = "20210108";
        let client = reqwest::Client::new();
        let market_type = MarketType::Kosdaq;
        // Act
        let result = get_krx_otp(trading_date, market_type, &client)
            .await
            .unwrap();
        // Assert
        assert!(result.len() > 10)
    }

    #[tokio::test]
    async fn download_sector_data_from_krx_for_kospi() {
        // Arrange
        let trading_date = "20210108";
        let client = reqwest::Client::new();
        let market_type = MarketType::Kospi;
        let otp = get_krx_otp(trading_date, market_type, &client)
            .await
            .unwrap();
        // Act
        let result = get_sector_data(&otp, &client).await.unwrap();
        // Assert
        assert_yaml_snapshot!(result)
    }
}
