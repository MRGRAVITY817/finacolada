use {
    self::{
        download::generate_krx_otp,
        merge_table::{merge_kospi_kosdaq, merge_sector_individual},
        save::{convert_individual_xlsx_to_parquet, convert_sector_xlsx_to_parquet},
    },
    crate::data_crawling::krx::download::download_krx_data,
};

pub mod download;
pub mod merge_table;
pub mod save;

pub async fn get_latest_krx_data(
    query_client: &reqwest::Client,
    trading_date: &str,
) -> anyhow::Result<()> {
    let kospi_sector_otp = generate_krx_otp(
        query_client,
        InfoType::Sector,
        MarketType::Kospi,
        trading_date,
    )
    .await?;
    let kospi_individual_otp = generate_krx_otp(
        query_client,
        InfoType::Individual,
        MarketType::Kospi,
        trading_date,
    )
    .await?;
    let kosdaq_sector_otp = generate_krx_otp(
        query_client,
        InfoType::Sector,
        MarketType::Kosdaq,
        trading_date,
    )
    .await?;
    let kosdaq_individual_otp = generate_krx_otp(
        query_client,
        InfoType::Individual,
        MarketType::Kosdaq,
        trading_date,
    )
    .await?;

    tokio::try_join!(
        download_krx_data(
            query_client,
            &kospi_sector_otp,
            "assets/krx/kospi_sector.xlsx"
        ),
        download_krx_data(
            query_client,
            &kospi_individual_otp,
            "assets/krx/kospi_individual.xlsx"
        ),
        download_krx_data(
            query_client,
            &kosdaq_sector_otp,
            "assets/krx/kosdaq_sector.xlsx"
        ),
        download_krx_data(
            query_client,
            &kosdaq_individual_otp,
            "assets/krx/kosdaq_individual.xlsx"
        ),
    )?;

    convert_sector_xlsx_to_parquet(
        "assets/krx/kospi_sector.xlsx",
        "assets/krx/kospi_sector.parquet",
    )?;
    convert_individual_xlsx_to_parquet(
        "assets/krx/kospi_individual.xlsx",
        "assets/krx/kospi_individual.parquet",
    )?;
    convert_sector_xlsx_to_parquet(
        "assets/krx/kosdaq_sector.xlsx",
        "assets/krx/kosdaq_sector.parquet",
    )?;
    convert_individual_xlsx_to_parquet(
        "assets/krx/kosdaq_individual.xlsx",
        "assets/krx/kosdaq_individual.parquet",
    )?;

    merge_sector_individual(
        "assets/krx/kospi_sector.parquet",
        "assets/krx/kospi_individual.parquet",
        "assets/krx/kospi_sector_individual.parquet",
    )?;
    merge_sector_individual(
        "assets/krx/kosdaq_sector.parquet",
        "assets/krx/kosdaq_individual.parquet",
        "assets/krx/kosdaq_sector_individual.parquet",
    )?;

    merge_kospi_kosdaq(
        "assets/krx/kospi_sector_individual.parquet",
        "assets/krx/kosdaq_sector_individual.parquet",
        format!("assets/krx/krx_merged_{trading_date}.parquet").as_str(),
    )
}

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
    u32,    // closing price
    i32,    // compared price
    f32,    // fluctuation rate
    u64,    // market cap
);

pub type KrxIndividualRow = (
    String, // issue code
    String, // issue name
    u32,    // closing price
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
