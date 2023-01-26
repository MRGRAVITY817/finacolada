use {reqwest::Client, std::collections::HashMap};

pub enum InfoType {
    Sector,
    Individual,
}

pub enum MarketType {
    Kospi,
    Kosdaq,
}

pub const GEN_OTP_URL: &'static str = "http://data.krx.co.kr/comm/fileDn/GenerateOTP/generate.cmd";
pub const SECTOR_DOWNLOAD_URL: &'static str =
    "http://data.krx.co.kr/comm/fileDn/download_csv/download.cmd";

pub async fn generate_krx_otp(
    query_client: &Client,
    info_type: InfoType,
    market_type: MarketType,
    trading_date: &str,
) -> Result<String, reqwest::Error> {
    let market_id = match market_type {
        MarketType::Kospi => "STK",
        MarketType::Kosdaq => "KSQ",
    };

    let mut params = HashMap::new();
    match info_type {
        InfoType::Sector => {
            params.insert("mktId", market_id);
            params.insert("trdDd", trading_date);
            params.insert("money", "1");
            params.insert("csvxls_isNo", "false");
            params.insert("name", "fileDown");
            params.insert("url", "dbms/MDC/STAT/standard/MDCSTAT03901");
        }
        InfoType::Individual => {
            params.insert("mktId", market_id);
            params.insert("trdDd", trading_date);
            params.insert("searchType", "1");
            params.insert("csvxls_isNo", "false");
            params.insert("name", "fileDown");
            params.insert("url", "dbms/MDC/STAT/standard/MDCSTAT03501");
        }
    };

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
        .text()
        .await
}

#[cfg(test)]
mod test {
    use {super::*, insta::*, reqwest};
    const TEST_TRADING_DATE: &'static str = "20230120";

    #[tokio::test]
    async fn generate_otp_for_kospi_sector_data() {
        // Arrange
        let client = reqwest::Client::new();
        // Act
        let result = generate_krx_otp(
            &client,
            InfoType::Sector,
            MarketType::Kospi,
            TEST_TRADING_DATE,
        )
        .await
        .unwrap();
        // Assert
        assert!(result.len() > 10)
    }

    #[tokio::test]
    async fn generate_otp_for_kosdaq_sector_data() {
        // Arrange
        let client = reqwest::Client::new();
        // Act
        let result = generate_krx_otp(
            &client,
            InfoType::Sector,
            MarketType::Kosdaq,
            TEST_TRADING_DATE,
        )
        .await
        .unwrap();
        // Assert
        assert!(result.len() > 10)
    }

    #[tokio::test]
    async fn generate_otp_for_kospi_individual_data() {
        // Arrange
        let client = reqwest::Client::new();
        // Act
        let result = generate_krx_otp(
            &client,
            InfoType::Individual,
            MarketType::Kospi,
            TEST_TRADING_DATE,
        )
        .await
        .unwrap();
        // Assert
        assert!(result.len() > 10)
    }

    #[tokio::test]
    async fn generate_otp_for_kosdaq_individual_data() {
        // Arrange
        let client = reqwest::Client::new();
        // Act
        let result = generate_krx_otp(
            &client,
            InfoType::Individual,
            MarketType::Kosdaq,
            TEST_TRADING_DATE,
        )
        .await
        .unwrap();
        // Assert
        assert!(result.len() > 10)
    }

    #[tokio::test]
    async fn download_sector_data_from_krx_for_kospi() {
        // Arrange
        let client = reqwest::Client::new();
        let info_type = InfoType::Sector;
        let market_type = MarketType::Kospi;
        let otp = generate_krx_otp(&client, info_type, market_type, TEST_TRADING_DATE)
            .await
            .unwrap();
        // Act
        let result = get_sector_data(&otp, &client).await.unwrap();
        // Assert
        assert_yaml_snapshot!(result)
    }

    #[tokio::test]
    async fn download_sector_data_from_krx_for_kosdaq() {
        // Arrange
        let client = reqwest::Client::new();
        let info_type = InfoType::Sector;
        let market_type = MarketType::Kosdaq;
        let otp = generate_krx_otp(&client, info_type, market_type, TEST_TRADING_DATE)
            .await
            .unwrap();
        // Act
        let result = get_sector_data(&otp, &client).await.unwrap();
        // Assert
        assert_yaml_snapshot!(result)
    }

    #[tokio::test]
    async fn download_sector_data_twice_with_same_otp() {
        // Arrange
        let client = reqwest::Client::new();
        let otp = generate_krx_otp(
            &client,
            InfoType::Sector,
            MarketType::Kospi,
            TEST_TRADING_DATE,
        )
        .await
        .unwrap();
        // Act
        let first_result = get_sector_data(&otp, &client).await.unwrap();
        let second_result = get_sector_data(&otp, &client).await.unwrap();
        // Assert
        assert_yaml_snapshot!(first_result);
        assert_yaml_snapshot!(second_result);
        assert_eq!(first_result, second_result)
    }
}
