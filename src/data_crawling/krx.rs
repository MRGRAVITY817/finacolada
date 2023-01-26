use {reqwest::Client, std::collections::HashMap};

pub enum InfoType {
    Sector,
    Individual,
}

pub enum MarketType {
    Kospi,
    Kosdaq,
}

pub type KrxSectorRow = (
    String, // issue code
    String, // issue name
    String, // market type: Kospi or Kosdaq
    String, // industry type
    u32,    // end value
    i32,    // compared price
    f32,    // fluctuation rate
    u64,    // market cap
);

pub type KrxIndividualRow = (
    String, // issue code
    String, // issue name
    u32,    // end value
    i32,    // compared price
    f32,    // fluctuation rate
    String, // EPS (number but contains `-`)
    String, // PER (number but contains `-`)
    String, // Leading EPS (number but contains `-`)
    String, // Leading PER (number but contains `-`)
    String, // BPS (number but contains `-`)
    String, // PBR (number but contains `-`)
    u32,    // Dividend Per Share
    f32,    // Dividend Yield Ratio
);

pub const GEN_OTP_URL: &'static str = "http://data.krx.co.kr/comm/fileDn/GenerateOTP/generate.cmd";
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

pub async fn download_krx_data(
    query_client: &Client,
    otp: &str,
    output_path: &str,
) -> Result<(), reqwest::Error> {
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

    Ok(std::fs::write(output_path, &result).unwrap())
}

#[cfg(test)]
mod test {
    use {
        super::*,
        calamine::{open_workbook, DeError, RangeDeserializerBuilder, Reader, Xlsx},
        insta::*,
        reqwest,
    };

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

    fn test_xlsx_sector(input_xlsx_path: &str) -> anyhow::Result<String> {
        let mut workbook: Xlsx<_> = open_workbook(input_xlsx_path)?;
        let range = workbook
            .worksheet_range("Sheet1")
            .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

        let table = RangeDeserializerBuilder::new()
            .from_range(&range)?
            .collect::<Result<Vec<KrxSectorRow>, DeError>>()?;

        Ok(format!(
            "{} {} {} {} {} {}",
            table[0].0, table[0].1, table[0].2, table[0].3, table[0].4, table[0].5
        ))
    }

    fn test_xlsx_individual(input_xlsx_path: &str) -> anyhow::Result<String> {
        let mut workbook: Xlsx<_> = open_workbook(input_xlsx_path)?;
        let range = workbook
            .worksheet_range("Sheet1")
            .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

        let table = RangeDeserializerBuilder::new()
            .from_range(&range)?
            .collect::<Result<Vec<KrxIndividualRow>, DeError>>()?;

        Ok(format!(
            "{} {} {} {} {} {} {} {} {} {}",
            table[0].0,
            table[0].1,
            table[0].2,
            table[0].3,
            table[0].4,
            table[0].5,
            table[0].6,
            table[0].7,
            table[0].8,
            table[0].9
        ))
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
        let output_path = "examples/krx_sector_kospi.xlsx";
        // Act
        download_krx_data(&client, &otp, output_path).await.unwrap();
        // Assert
        assert_snapshot!(
            test_xlsx_sector(output_path).unwrap(),
            @"095570 AJ네트웍스 KOSPI 서비스업 6050 -10"
        );
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
        let output_path = "examples/krx_sector_kosdaq.xlsx";
        // Act
        download_krx_data(&client, &otp, output_path).await.unwrap();
        // Assert
        assert_snapshot!(
            test_xlsx_sector(output_path).unwrap(),
            @"060310 3S KOSDAQ 기계·장비 2235 15"
        );
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
        let output_path = "examples/krx_individual_kospi.xlsx";
        // Act
        download_krx_data(&client, &otp, output_path).await.unwrap();
        // Assert
        assert_snapshot!(
            test_xlsx_individual(output_path).unwrap(),
            @"095570 AJ네트웍스  6050 -10 -0.17 1707 3.54 977 6.19 8075"
        );
    }

    #[tokio::test]
    async fn download_individual_kosdaq_data() {
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
        let output_path = "examples/krx_individual_kosdaq.xlsx";
        // Act
        download_krx_data(&client, &otp, output_path).await.unwrap();
        // Assert
        assert_snapshot!(
            test_xlsx_individual(output_path).unwrap(),
            @"060310 3S  2235 15 0.68 33 67.73 - - 829"
        );
    }

    // #[tokio::test]
    // async fn download_sector_data_twice_with_same_otp() {
    //     // Arrange
    //     let client = reqwest::Client::new();
    //     let otp = generate_krx_otp(
    //         &client,
    //         InfoType::Sector,
    //         MarketType::Kospi,
    //         TEST_TRADING_DATE,
    //     )
    //     .await
    //     .unwrap();
    //     // Act
    //     let first_result = download_krx_data(&otp, &client).await.unwrap();
    //     let second_result = download_krx_data(&otp, &client).await.unwrap();
    //     // Assert
    //     assert_yaml_snapshot!(first_result);
    //     assert_yaml_snapshot!(second_result);
    //     assert_eq!(first_result, second_result)
    // }

    // #[tokio::test]
    // async fn download_individual_data_thrice_with_same_otp() {
    //     // Arrange
    //     let client = reqwest::Client::new();
    //     let otp = generate_krx_otp(
    //         &client,
    //         InfoType::Sector,
    //         MarketType::Kospi,
    //         TEST_TRADING_DATE,
    //     )
    //     .await
    //     .unwrap();
    //     // Act
    //     let first_result = download_krx_data(&otp, &client).await.unwrap();
    //     let second_result = download_krx_data(&otp, &client).await.unwrap();
    //     let third_result = download_krx_data(&otp, &client).await.unwrap();
    //     // Assert
    //     assert_yaml_snapshot!(first_result);
    //     assert_yaml_snapshot!(second_result);
    //     assert_yaml_snapshot!(third_result);
    //     assert_eq!(first_result, second_result);
    //     assert_eq!(first_result, third_result);
    // }
}
