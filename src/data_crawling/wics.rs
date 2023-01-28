use serde_json::Value;

use {super::biz_day::get_latest_biz_day, serde::Deserialize};

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

// pub async fn get_latest_wisc_data(query_client: &reqwest::Client) -> anyhow::Result<String> {
//     // 1. Get WICS data for every sector codes
//     let date = get_latest_biz_day(query_client).await?;
//     // 2. Deserialize JSON string as struct
//     let meta: Value = serde_json::from_str(&)
//     // 3. Get the "list" property
//     // 4. Create a DataFrame that contains each sector items.
//     let df = df![
//       "IDX_CD" => idx_cd,
//       "IDX_NM_KOR" => idx_nm_kor,
//       "ALL_MKT_VAL" => all_mkt_val,
//       "CMP_CD" => cmp_cd,
//       "CMP_KOR" => cmp_kor,
//       "MKT_VAL" => mkt_val,
//       "WGT" => wgt,
//       "S_WGT" => s_wgt,
//       "CAL_WGT" => cal_wgt,
//       "SEC_CD" => sec_cd,
//       "SEC_NM_KOR" => sec_nm_kor,
//       "SEQ" => seq,
//       "TOP60" => top60,
//       "APT_SHR_CNT" => apt_shr_cnt
//     ]?;
//     // 5. Save DataFrame as parquet file
//     let file_name = "assets/wics/latest_wics_data.parquet";
//     // 6. Return the parquet file name
//     Ok(file_name.to_string())
// }

#[cfg(test)]
mod test {
    use {super::*, insta::assert_yaml_snapshot};

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
}
