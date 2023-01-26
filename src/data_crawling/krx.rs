use {
    calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx},
    reqwest::Client,
    std::collections::HashMap,
};

pub enum InfoType {
    Sector,
    Individual,
}

pub enum MarketType {
    Kospi,
    Kosdaq,
}

pub const GEN_OTP_URL: &'static str = "http://data.krx.co.kr/comm/fileDn/GenerateOTP/generate.cmd";
pub const CSV_DOWNLOAD_URL: &'static str =
    "http://data.krx.co.kr/comm/fileDn/download_csv/download.cmd";
pub const EXCEL_DOWNLOAD_URL: &'static str =
    "http://data.krx.co.kr/comm/fileDn/download_excel/download.cmd";

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
            params.insert("locale", "ko_KR");
            params.insert("mktId", market_id);
            params.insert("trdDd", trading_date);
            params.insert("money", "1");
            params.insert("csvxls_isNo", "false");
            params.insert("name", "fileDown");
            params.insert("url", "dbms/MDC/STAT/standard/MDCSTAT03901");
        }
        InfoType::Individual => {
            params.insert("locale", "ko_KR");
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

pub async fn download_krx_data(otp: &str, query_client: &Client) -> Result<String, reqwest::Error> {
    let mut params = HashMap::new();
    params.insert("code", otp);

    query_client
        .post(CSV_DOWNLOAD_URL)
        .form(&params)
        .header("referer", GEN_OTP_URL)
        .send()
        .await?
        .text_with_charset("euc-kr")
        .await
}

pub async fn download_excel_data(otp: &str, query_client: &Client) -> Result<(), reqwest::Error> {
    let mut params = HashMap::new();
    params.insert("code", otp);

    let result = query_client
        .post(EXCEL_DOWNLOAD_URL)
        .form(&params)
        .header("referer", GEN_OTP_URL)
        .send()
        .await?
        .bytes()
        .await?;

    Ok(std::fs::write("test.xlsx", &result).unwrap())
}

pub fn parse_xlsx_file(path: &str) -> anyhow::Result<String> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

    let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;

    if let Some(result) = iter.next() {
        let (code, name, market_type, sector, end_value, relative, change_rate, net_value): (
            String,
            String,
            String,
            String,
            u32,
            i32,
            f32,
            u64,
        ) = result?;
        Ok(format!(
            "{code} {name} {market_type} {sector} {end_value} {relative} {change_rate} {net_value}"
        ))
    } else {
        Err(calamine::Error::Msg("Cannot parse 'Sheet1'").into())
    }
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
    async fn download_sector_kospi_data() {
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
        let result = download_krx_data(&otp, &client).await.unwrap();
        // Assert
        assert_yaml_snapshot!(result)
    }

    #[tokio::test]
    async fn download_sector_kosdaq_data() {
        // Arrange
        let client = reqwest::Client::new();
        let otp = generate_krx_otp(
            &client,
            InfoType::Sector,
            MarketType::Kosdaq,
            TEST_TRADING_DATE,
        )
        .await
        .unwrap();
        // Act
        let result = download_krx_data(&otp, &client).await.unwrap();
        // Assert
        assert_yaml_snapshot!(result)
    }

    #[tokio::test]
    async fn download_individual_kospi_data() {
        // Arrange
        let client = reqwest::Client::new();
        let otp = generate_krx_otp(
            &client,
            InfoType::Individual,
            MarketType::Kospi,
            TEST_TRADING_DATE,
        )
        .await
        .unwrap();
        // Act
        let result = download_krx_data(&otp, &client).await.unwrap();
        // Assert
        assert_yaml_snapshot!(result)
    }

    #[tokio::test]
    async fn download_individual_kosdaq() {
        // Arrange
        let client = reqwest::Client::new();
        let otp = generate_krx_otp(
            &client,
            InfoType::Individual,
            MarketType::Kosdaq,
            TEST_TRADING_DATE,
        )
        .await
        .unwrap();
        // Act
        let result = download_krx_data(&otp, &client).await.unwrap();
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
        let first_result = download_krx_data(&otp, &client).await.unwrap();
        let second_result = download_krx_data(&otp, &client).await.unwrap();
        // Assert
        assert_yaml_snapshot!(first_result);
        assert_yaml_snapshot!(second_result);
        assert_eq!(first_result, second_result)
    }

    #[tokio::test]
    async fn download_individual_data_thrice_with_same_otp() {
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
        let first_result = download_krx_data(&otp, &client).await.unwrap();
        let second_result = download_krx_data(&otp, &client).await.unwrap();
        let third_result = download_krx_data(&otp, &client).await.unwrap();
        // Assert
        assert_yaml_snapshot!(first_result);
        assert_yaml_snapshot!(second_result);
        assert_yaml_snapshot!(third_result);
        assert_eq!(first_result, second_result);
        assert_eq!(first_result, third_result);
    }

    #[tokio::test]
    async fn download_excel_data_for_sector() {
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
        let _ = download_excel_data(&otp, &client).await.unwrap();
        let result = parse_xlsx_file("test.xlsx").unwrap();
        // Assert
        assert_snapshot!(result, @"095570 AJ네트웍스 KOSPI 서비스업 6050 -10 -0.17 283274884750")
    }
}
