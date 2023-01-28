use {polars::prelude::*, std::collections::HashSet};

pub fn get_common_columns<'a>(
    left_input: &'a str,
    right_input: &'a str,
) -> anyhow::Result<HashSet<String>> {
    // 1. read to parquets as dataframe
    let left_df = LazyFrame::scan_parquet(left_input, Default::default())?;
    let right_df = LazyFrame::scan_parquet(right_input, Default::default())?;
    // 2. get intersection
    let left_columns = left_df
        .schema()?
        .clone()
        .iter()
        .map(|(k, _)| k.to_owned())
        .collect::<HashSet<String>>();
    let right_columns = right_df
        .schema()?
        .clone()
        .iter()
        .map(|(k, _)| k.to_owned())
        .collect::<HashSet<String>>();
    // 3. return the set of intersection
    let result = left_columns
        .intersection(&right_columns)
        .collect::<HashSet<_>>();

    Ok(result.into_iter().map(|k| k.to_owned()).collect())
}

#[cfg(test)]
mod test {
    use {super::*, std::collections::HashSet};

    #[test]
    fn find_out_common_columns() {
        // Arrange
        let left_input = "examples/krx_sector_kospi.parquet";
        let right_input = "examples/krx_individual_kospi.parquet";
        // Act
        let result = get_common_columns(left_input, right_input).unwrap();
        // Assert
        assert_eq!(
            result,
            HashSet::from([
                "issue_name".to_string(),
                "compared".to_string(),
                "end_value".to_string(),
                "fluctuation_rate".to_string(),
                "issue_code".to_string(),
            ])
        );
    }

    #[test]
    fn merged_table_should_not_contain_esoteric_issues() {
        // Arrange
        let sector_path = "examples/krx_sector_kospi.parquet";
        let indi_path = "examples/krx_individual_kospi.parquet";
        // Act
        let result = merge_sector_individual(sector_path, indi_path).unwrap();
        // Assert
        assert!(!result.select([col("issues_name")]).contains("SK리츠"))
    }
}
