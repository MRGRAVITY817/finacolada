use {
    calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx},
    polars::prelude::*,
};

pub fn save_xlsx_to_parquet(path: &str) -> anyhow::Result<String> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet1'"))??;

    let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;

    while let Some(result) = iter.next() {
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
        let mut df = df!(
            "foo" => &[1, 2, 3],
            "bar" => &[None, Some("bak"), Some("baz")],
        );
    }

    Ok("result.parquet".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_read_converted_parquet_as_lazyframe() {
        // Arrange
        let path = "test.xlsx";
        // Act
        let result_path = save_xlsx_to_parquet(path).unwrap();
        // Assert
        assert!(LazyFrame::scan_parquet(result_path, Default::default()).is_ok())
    }
}
