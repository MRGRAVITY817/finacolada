use {
    super::biz_day::get_latest_biz_day, polars::prelude::*, serde::Deserialize, serde_json::Value,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct WicsRow {
    idx_cd: String,
    idx_nm_kor: String,
    all_mkt_val: u64,
    cmp_cd: String,
    cmp_kor: String,
    mkt_val: u64,
    wgt: f32,
    s_wgt: f32,
    cal_wgt: f32,
    sec_cd: String,
    sec_nm_kor: String,
    seq: i32,
    top60: u32,
    apt_shr_cnt: u64,
}

pub const SECTOR_CODE_LIST: [&str; 10] = [
    "G25", "G35", "G50", "G40", "G10", "G20", "G55", "G30", "G15", "G45",
];

pub async fn get_wics(
    query_client: &reqwest::Client,
    date: &str,
    sector_code: &str,
) -> Result<String, reqwest::Error> {
    // 1. Request the WICS data with URL & params
    let url = "https://www.wiseindex.com/Index/GetIndexComponets";
    query_client
        .get(url)
        .query(&[("ceil_yn", "0"), ("dt", date), ("sec_cd", sector_code)])
        .send()
        .await?
        .text()
        .await
}

pub async fn get_daily_wics_list(
    query_client: &reqwest::Client,
    date: &str,
) -> anyhow::Result<Vec<WicsRow>> {
    let mut row_list: Vec<WicsRow> = vec![];
    for sector_code in SECTOR_CODE_LIST {
        let json_string = get_wics(query_client, date, sector_code).await?;
        let meta: Value = serde_json::from_str(&json_string)?;
        for wics in meta["list"].as_array().unwrap() {
            let item: WicsRow = serde_json::from_value(wics.to_owned())?;
            row_list.push(item);
        }
    }

    Ok(row_list)
}

pub async fn get_latest_wisc_data(query_client: &reqwest::Client) -> anyhow::Result<String> {
    // 1. Get WICS data for every sector codes
    let date = get_latest_biz_day(query_client).await?;
    // 2. Deserialize all WICS data
    let wics_list = get_daily_wics_list(query_client, &date).await?;
    // 3. Create a DataFrame that contains each sector items.
    let mut df = df!(
      "idx_cd" => wics_list.iter().map(|wics| wics.idx_cd.clone()).collect::<Vec<_>>(),
      "idx_nm_kor" => wics_list.iter().map(|wics| wics.idx_nm_kor.clone()).collect::<Vec<_>>(),
      "all_mkt_val" => wics_list.iter().map(|wics| wics.all_mkt_val).collect::<Vec<_>>(),
      "cmp_cd" => wics_list.iter().map(|wics| wics.cmp_cd.clone()).collect::<Vec<_>>(),
      "cmp_kor" => wics_list.iter().map(|wics| wics.cmp_kor.clone()).collect::<Vec<_>>(),
      "mkt_val" => wics_list.iter().map(|wics| wics.mkt_val).collect::<Vec<_>>(),
      "wgt" => wics_list.iter().map(|wics| wics.wgt).collect::<Vec<_>>(),
      "s_wgt" => wics_list.iter().map(|wics| wics.s_wgt).collect::<Vec<_>>(),
      "cal_wgt" => wics_list.iter().map(|wics| wics.cal_wgt).collect::<Vec<_>>(),
      "sec_cd" => wics_list.iter().map(|wics| wics.sec_cd.clone()).collect::<Vec<_>>(),
      "sec_nm_kor" => wics_list.iter().map(|wics| wics.sec_nm_kor.clone()).collect::<Vec<_>>(),
      "seq" => wics_list.iter().map(|wics| wics.seq).collect::<Vec<_>>(),
      "top60" => wics_list.iter().map(|wics| wics.top60).collect::<Vec<_>>(),
      "apt_shr_cnt" => wics_list.iter().map(|wics| wics.apt_shr_cnt).collect::<Vec<_>>()
    )?;
    // 5. Save DataFrame as parquet file
    let file_name = "assets/wics/latest_wics_data.parquet";
    let mut file = std::fs::File::create(file_name)?;
    ParquetWriter::new(&mut file).finish(&mut df)?;
    // 6. Return the parquet file name
    Ok(file_name.to_string())
}

#[cfg(test)]
mod test {
    use {super::*, insta::*};

    #[tokio::test]
    async fn should_have_json_text() {
        // Arrange
        let client = reqwest::Client::new();
        let date = "20230126";
        let sector_code = "G10";
        // Act
        let result = get_wics(&client, date, sector_code).await.unwrap();
        // Assert
        assert_yaml_snapshot!(result)
    }

    #[tokio::test]
    async fn different_date_should_have_different_data() {
        // Arrange
        let client = reqwest::Client::new();
        let early_date = "20230126";
        let now_date = "20230127";
        let sector_code = "G10";
        // Act
        let early_result = get_wics(&client, early_date, sector_code).await.unwrap();
        let now_result = get_wics(&client, now_date, sector_code).await.unwrap();
        // Assert
        assert_ne!(early_result, now_result)
    }

    #[tokio::test]
    async fn gets_wics_rows() {
        // Arrange
        let client = reqwest::Client::new();
        let date = "20230126";
        // Act
        let result = get_daily_wics_list(&client, date).await.unwrap();
        // Assert
        assert_yaml_snapshot!(format!("{:?}", result.iter().next()))
    }

    #[tokio::test]
    async fn can_read_wics_parquet_as_lazyframe() {
        // Arrange
        let client = reqwest::Client::new();
        // Act
        let result = get_latest_wisc_data(&client).await.unwrap();
        // Assert
        let lf = LazyFrame::scan_parquet(&result, Default::default());
        assert!(lf.is_ok());
        assert_snapshot!(lf
            .unwrap()
            .filter(col("seq").lt(lit(2)))
            .collect()
            .unwrap()
            .to_string(), @r###"
        shape: (10, 14)
        ┌────────┬─────────────────────────┬─────────────┬────────┬─────┬────────────────────┬─────┬───────┬─────────────┐
        │ idx_cd ┆ idx_nm_kor              ┆ all_mkt_val ┆ cmp_cd ┆ ... ┆ sec_nm_kor         ┆ seq ┆ top60 ┆ apt_shr_cnt │
        │ ---    ┆ ---                     ┆ ---         ┆ ---    ┆     ┆ ---                ┆ --- ┆ ---   ┆ ---         │
        │ str    ┆ str                     ┆ u64         ┆ str    ┆     ┆ str                ┆ i32 ┆ u32   ┆ u64         │
        ╞════════╪═════════════════════════╪═════════════╪════════╪═════╪════════════════════╪═════╪═══════╪═════════════╡
        │ G25    ┆ WICS 경기관련소비재     ┆ 127513928   ┆ 005380 ┆ ... ┆ 경기관련소비재     ┆ 1   ┆ 9     ┆ 141021003   │
        ├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ G35    ┆ WICS 건강관리           ┆ 106729517   ┆ 068270 ┆ ... ┆ 건강관리           ┆ 1   ┆ 25    ┆ 111200858   │
        ├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ G50    ┆ WICS 커뮤니케이션서비스 ┆ 107864411   ┆ 035420 ┆ ... ┆ 커뮤니케이션서비스 ┆ 1   ┆ 6     ┆ 127958286   │
        ├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ G40    ┆ WICS 금융               ┆ 124910046   ┆ 055550 ┆ ... ┆ 금융               ┆ 1   ┆ 7     ┆ 417203593   │
        ├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ ...    ┆ ...                     ┆ ...         ┆ ...    ┆ ... ┆ ...                ┆ ... ┆ ...   ┆ ...         │
        ├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ G55    ┆ WICS 유틸리티           ┆ 11624366    ┆ 015760 ┆ ... ┆ 유틸리티           ┆ 1   ┆ 2     ┆ 288883835   │
        ├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ G30    ┆ WICS 필수소비재         ┆ 28918599    ┆ 033780 ┆ ... ┆ 필수소비재         ┆ 1   ┆ 5     ┆ 108461073   │
        ├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ G15    ┆ WICS 소재               ┆ 112616619   ┆ 051910 ┆ ... ┆ 소재               ┆ 1   ┆ 6     ┆ 45179100    │
        ├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ G45    ┆ WICS IT                 ┆ 513513422   ┆ 005930 ┆ ... ┆ IT                 ┆ 1   ┆ 2     ┆ 4477336913  │
        └────────┴─────────────────────────┴─────────────┴────────┴─────┴────────────────────┴─────┴───────┴─────────────┘
        "###)
    }
}
