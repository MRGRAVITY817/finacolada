use crate::utils::save_df_as_parquet;

use {
    super::{KrxIndividualRow, KrxSectorRow},
    calamine::{open_workbook, DeError, RangeDeserializerBuilder, Reader, Xlsx},
    polars::prelude::*,
};

pub fn convert_sector_xlsx_to_parquet<'a>(
    input_xlsx_path: &'a str,
    output_path: &'a str,
) -> anyhow::Result<()> {
    let mut workbook: Xlsx<_> = open_workbook(input_xlsx_path)?;
    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

    let table = RangeDeserializerBuilder::new()
        .from_range(&range)?
        .collect::<Result<Vec<KrxSectorRow>, DeError>>()?;

    let mut df = df!(
        "issue_code" => table.iter().map(|row| row.0.clone()).collect::<Vec<_>>(),
        "issue_name" => table.iter().map(|row| row.1.clone()).collect::<Vec<_>>(),
        "market_type" => table.iter().map(|row| row.2.clone()).collect::<Vec<_>>(),
        "industry" => table.iter().map(|row| row.3.clone()).collect::<Vec<_>>(),
        "closing_price" => table.iter().map(|row| row.4).collect::<Vec<_>>(),
        "compared" => table.iter().map(|row| row.5).collect::<Vec<_>>(),
        "fluctuation_rate" => table.iter().map(|row| row.6).collect::<Vec<_>>(),
        "market_cap" => table.iter().map(|row| row.7).collect::<Vec<_>>()
    )?;

    save_df_as_parquet(output_path, &mut df)
}

pub fn convert_individual_xlsx_to_parquet<'a>(
    input_xlsx_path: &'a str,
    output_path: &'a str,
) -> anyhow::Result<()> {
    let mut workbook: Xlsx<_> = open_workbook(input_xlsx_path)?;
    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

    let table = RangeDeserializerBuilder::new()
        .from_range(&range)?
        .collect::<Result<Vec<KrxIndividualRow>, DeError>>()?;

    let mut df = df!(
        "issue_code" => table.iter().map(|row| row.0.clone()).collect::<Vec<_>>(),
        "issue_name" => table.iter().map(|row| row.1.clone()).collect::<Vec<_>>(),
        "closing_price" => table.iter().map(|row| row.2).collect::<Vec<_>>(),
        "compared" => table.iter().map(|row| row.3).collect::<Vec<_>>(),
        "fluctuation_rate" => table.iter().map(|row| row.4).collect::<Vec<_>>(),
        "eps" => table.iter().map(|row| row.5.clone()).collect::<Vec<_>>(),
        "per" => table.iter().map(|row| row.6.clone()).collect::<Vec<_>>(),
        "leading_eps" => table.iter().map(|row| row.7.clone()).collect::<Vec<_>>(),
        "leading_per" => table.iter().map(|row| row.8.clone()).collect::<Vec<_>>(),
        "bps" => table.iter().map(|row| row.9.clone()).collect::<Vec<_>>(),
        "pbr" => table.iter().map(|row| row.10.clone()).collect::<Vec<_>>(),
        "dps" => table.iter().map(|row| row.11).collect::<Vec<_>>(),
        "dyr" => table.iter().map(|row| row.12).collect::<Vec<_>>()
    )?;

    save_df_as_parquet(output_path, &mut df)
}

#[cfg(test)]
mod test {
    use {super::*, rstest::rstest};

    #[rstest]
    #[case("examples/krx_sector_kospi.xlsx", "examples/krx_sector_kospi.parquet")]
    #[case(
        "examples/krx_sector_kosdaq.xlsx",
        "examples/krx_sector_kosdaq.parquet"
    )]
    fn saves_sector_xlsx_into_parquet(
        // Arrange
        #[case] input_path: &str,
        #[case] output_path: &str,
    ) {
        // Act
        convert_sector_xlsx_to_parquet(input_path, output_path).unwrap();
        // Assert
        let lf = LazyFrame::scan_parquet(output_path, Default::default());
        assert!(lf.is_ok());
    }

    #[rstest]
    #[case(
        "examples/krx_individual_kospi.xlsx",
        "examples/krx_individual_kospi.parquet"
    )]
    #[case(
        "examples/krx_individual_kosdaq.xlsx",
        "examples/krx_individual_kosdaq.parquet"
    )]
    #[test]
    fn saves_individual_xlsx_to_parquet(
        // Arrange
        #[case] input_path: &str,
        #[case] output_path: &str,
    ) {
        // Act
        convert_individual_xlsx_to_parquet(input_path, output_path).unwrap();
        // Assert
        let lf = LazyFrame::scan_parquet(output_path, Default::default());
        assert!(lf.is_ok());
    }
}
