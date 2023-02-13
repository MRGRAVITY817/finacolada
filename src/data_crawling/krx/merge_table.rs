use polars::prelude::*;

pub fn merge_sector_individual(
    sector_path: &str,
    indi_path: &str,
    output_path: &str,
) -> anyhow::Result<()> {
    // 1. Read parquets as LazyFrame
    let sector_df = LazyFrame::scan_parquet(sector_path, Default::default())?;
    let indi_df = LazyFrame::scan_parquet(indi_path, Default::default())?;
    // 2. Inner Join
    let mut merged = sector_df
        .inner_join(indi_df, col("issue_code"), col("issue_code"))
        .filter(col("issue_name").str().contains("스팩")) // filter out spac company
        .filter(col("issue_code").str().contains("0$")) // remove preferred stock (code ends with zero)
        .collect()?;
    // 3. Save as parquet file
    let mut file = std::fs::File::create(output_path)?;
    ParquetWriter::new(&mut file).finish(&mut merged)?;

    Ok(())
}

pub fn get_tickers() -> anyhow::Result<()> {
    todo!()
}

#[cfg(test)]
mod test {
    use {super::*, insta::assert_snapshot};

    #[test]
    fn get_ten_tickers() {
        let result = get_tickers().unwrap();

        assert_snapshot!(result[0..10], @"")
    }

    #[test]
    fn merged_table_should_not_contain_esoteric_issues() {
        // Arrange
        let sector_path = "examples/krx_sector_kospi.parquet";
        let indi_path = "examples/krx_individual_kospi.parquet";
        let output_path = "examples/krx_merged_kospi.parquet";
        // Act
        merge_sector_individual(sector_path, indi_path, output_path).unwrap();
        // Assert
        let result = LazyFrame::scan_parquet(output_path, Default::default()).unwrap();
        assert_snapshot!(result
            .select([col("issue_name").filter(col("issue_name").eq(lit("SK리츠")))])
            .collect()
            .unwrap().to_string(), 
            @r###"
        shape: (0, 1)
        ┌────────────┐
        │ issue_name │
        │ ---        │
        │ str        │
        ╞════════════╡
        └────────────┘
        "###)
    }

    #[test]
    fn merged_table_should_contain_all_rows() {
        // Arrange
        let sector_path = "examples/krx_sector_kospi.parquet";
        let indi_path = "examples/krx_individual_kospi.parquet";
        let output_path = "examples/krx_merged_kospi.parquet";
        // Act
        merge_sector_individual(sector_path, indi_path, output_path).unwrap();
        // Assert
        let result = LazyFrame::scan_parquet(output_path, Default::default()).unwrap();
        assert_eq!(
            result
                .schema()
                .unwrap()
                .clone()
                .iter()
                .map(|(k, _)| k.to_owned())
                .collect::<Vec<String>>(),
            vec![
                "issue_code",
                "issue_name",
                "market_type",
                "industry",
                "closing_price",
                "compared",
                "fluctuation_rate",
                "market_cap",
                "issue_name_right",
                "closing_price_right",
                "compared_right",
                "fluctuation_rate_right",
                "eps",
                "per",
                "leading_eps",
                "leading_per",
                "bps",
                "pbr",
                "dps",
                "dyr"
            ]
        )
    }
}
