use {
    crate::data_crawling::krx::KrxSectorRow,
    calamine::{open_workbook, DeError, RangeDeserializerBuilder, Reader, Xlsx},
    polars::prelude::*,
};

pub fn convert_xlsx_to_parquet<'a>(
    input_xlsx_path: &'a str,
    output_parquet_path: &'a str,
) -> anyhow::Result<()> {
    let mut workbook: Xlsx<_> = open_workbook(input_xlsx_path)?;
    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

    let table = RangeDeserializerBuilder::new()
        .from_range(&range)?
        .collect::<Result<Vec<KrxSectorRow>, DeError>>()?;

    let issue_code_data: Vec<String> = table.iter().map(|row| row.0.clone()).collect();
    let issue_name_data: Vec<String> = table.iter().map(|row| row.1.clone()).collect();
    let market_type_data: Vec<String> = table.iter().map(|row| row.2.clone()).collect();
    let industry_name: Vec<String> = table.iter().map(|row| row.3.clone()).collect();
    let end_value_data: Vec<u32> = table.iter().map(|row| row.4).collect();
    let compared_data: Vec<i32> = table.iter().map(|row| row.5).collect();
    let fluctuation_rate_data: Vec<f32> = table.iter().map(|row| row.6).collect();
    let market_cap_data: Vec<u64> = table.iter().map(|row| row.7).collect();

    let mut df = df!(
        "issue_code" => issue_code_data,
        "issue_name" => issue_name_data,
        "market_type" => market_type_data,
        "industry" => industry_name,
        "end_value" => end_value_data,
        "compared" => compared_data,
        "fluctuation_rate" => fluctuation_rate_data,
        "market_cap" => market_cap_data
    )?;

    let mut file = std::fs::File::create(output_parquet_path)?;
    ParquetWriter::new(&mut file).finish(&mut df)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use {super::*, insta::assert_snapshot};

    #[test]
    fn can_read_converted_parquet_as_lazyframe() {
        // Arrange
        let input_path = "test.xlsx";
        let output_path = "converted.parquet";
        // Act
        convert_xlsx_to_parquet(input_path, output_path).unwrap();
        // Assert
        let lf = LazyFrame::scan_parquet(output_path, Default::default());
        assert!(lf.is_ok());
        assert_snapshot!(lf
            .unwrap()
            .filter(col("end_value").gt(lit(10000)))
            .collect()
            .unwrap()
            .to_string())
    }
}
