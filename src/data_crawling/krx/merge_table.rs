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
        .filter(col("issue_name").str().contains("스팩").not()) // filter out spac company
        .filter(col("issue_code").str().contains("0$").not()) // remove preferred stock (code ends with zero)
        .collect()?;
    // 3. Save as parquet file
    let mut file = std::fs::File::create(output_path)?;
    ParquetWriter::new(&mut file).finish(&mut merged)?;

    Ok(())
}

pub fn get_tickers(file_path: &str) -> anyhow::Result<Vec<String>> {
    let df = LazyFrame::scan_parquet(file_path, Default::default())?
        .select(&[col("issue_code")])
        .collect()?;

    let tickers_series = df
        .iter()
        .next()
        .ok_or(anyhow::Error::msg("Cannot get ticker value"))?;

    Ok(tickers_series.iter().map(|val| val.to_string()).collect())
}

#[cfg(test)]
mod test {
    use {super::*, insta::assert_snapshot};

    #[test]
    fn get_latest_tickers() {
        let output_path = "examples/krx_merged_kospi.parquet";

        let result = get_tickers(output_path).unwrap();

        assert_snapshot!(result.join(", "), @r###""001465", "00104K", "011155", "001045", "097955", "000995", "000215", "37550L", "37550K", "078935", "001067", "001065", "051905", "003555", "066575", "051915", "108675", "38380K", "005945", "004255", "010955", "001745", "006125", "03473K", "096775", "001515", "28513K", "012205", "014285", "002995", "011785", "004545", "008355", "003925", "005725", "002355", "090355", "000325", "005257", "019685", "00806K", "35320K", "001685", "084695", "003547", "003545", "006345", "001795", "003495", "004835", "005965", "001527", "001525", "014825", "000157", "000155", "33626K", "33626L", "00499K", "005305", "00680K", "006805", "001275", "006405", "02826K", "009155", "005935", "010145", "000815", "145995", "000075", "004415", "014915", "004985", "004365", "33637K", "33637L", "001725", "019175", "00279K", "002795", "090435", "003475", "000225", "000227", "000105", "003465", "007575", "002787", "002785", "007815", "00781K", "003075", "002025", "120115", "26490K", "005745", "004105", "009415", "36328K", "000087", "000145", "071055", "001755", "18064K", "00088K", "009835", "000885", "003535", "000725", "004565", "005387", "005389", "005385", "008775", "000547", "000545""###);
        assert_snapshot!(result.len().to_string(), @"117")
    }

    #[test]
    fn merged_table_should_contain_samsung_branches() {
        // Arrange
        let sector_path = "examples/krx_sector_kospi.parquet";
        let indi_path = "examples/krx_individual_kospi.parquet";
        let output_path = "examples/krx_merged_kospi.parquet";
        // Act
        merge_sector_individual(sector_path, indi_path, output_path).unwrap();
        // Assert
        let result = LazyFrame::scan_parquet(output_path, Default::default()).unwrap();
        assert_snapshot!(result
            .select([col("issue_name").filter(col("issue_name").str().contains("삼성"))])
            .collect()
            .unwrap().to_string(), 
            @r###"
        shape: (6, 1)
        ┌─────────────┐
        │ issue_name  │
        │ ---         │
        │ str         │
        ╞═════════════╡
        │ 삼성SDI우   │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ 삼성물산우B │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ 삼성전기우  │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ 삼성전자우  │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ 삼성중공우  │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ 삼성화재우  │
        └─────────────┘
        "###)
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
            .select([col("issue_name").filter(col("issue_name").str().contains("SK리츠"))])
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
