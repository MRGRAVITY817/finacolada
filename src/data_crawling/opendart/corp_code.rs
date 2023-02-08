use {
    polars::{
        df,
        prelude::{NamedFrom, ParquetWriter},
    },
    quick_xml::de::from_str as xml_from_str,
    serde::{Deserialize, Serialize},
    zip::ZipArchive,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "result")]
struct CorpCodeResult {
    list: Vec<CorpCodeItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CorpCodeItem {
    corp_code: String,
    corp_name: String,
    stock_code: String,
    modify_date: String,
}

fn save_corp_code_list(output_path: &str, corp_code_list: &[CorpCodeItem]) -> anyhow::Result<()> {
    let mut df = df![
        "corp_code" => corp_code_list.into_iter().map(|item| item.corp_code.clone()).collect::<Vec<String>>(),
        "corp_name" => corp_code_list.into_iter().map(|item| item.corp_name.clone()).collect::<Vec<String>>(),
        "stock_code" => corp_code_list.into_iter().map(|item| item.stock_code.clone()).collect::<Vec<String>>(),
        "modify_date" => corp_code_list.into_iter().map(|item| item.modify_date.clone()).collect::<Vec<String>>(),
    ]?;
    let mut file = std::fs::File::create(output_path)?;
    ParquetWriter::new(&mut file).finish(&mut df)?;

    Ok(())
}

async fn save_corp_codes_xml(
    query_client: &reqwest::Client,
    api_key: &str,
    output_path: Option<&str>,
) -> anyhow::Result<()> {
    let result = query_client
        .get("https://opendart.fss.or.kr/api/corpCode.xml")
        .query(&[("crtfc_key", api_key)])
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.77 Safari/537.36")
        .send()
        .await?
        .bytes()
        .await?;

    let output_path = output_path.unwrap_or("assets/corp_codes.zip");
    std::fs::write(output_path, &result)?;

    let fname = std::path::Path::new(output_path);
    let file = std::fs::File::open(fname)?;

    let mut archive = ZipArchive::new(file)?;
    archive
        .extract(output_path.replace(".zip", ""))
        .map_err(|_| anyhow::Error::msg("Cannot extract corp code zip file"))
}

fn extract_corp_codes(input_path: &str) -> anyhow::Result<Vec<CorpCodeItem>> {
    let xml_string = std::fs::read_to_string(input_path)?;
    let result: Vec<CorpCodeItem> = xml_from_str::<CorpCodeResult>(&xml_string)?
        .list
        .into_iter()
        .collect();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use polars::prelude::*;

    #[tokio::test]
    async fn should_have_valid_corp_code_xml_file() {
        let client = reqwest::Client::new();
        let api_key = std::env::var("OPENDART_API_KEY").unwrap();
        let output_path = "examples/corp_codes.zip";
        let result_path = "examples/corp_codes/CORPCODE.xml";

        save_corp_codes_xml(&client, &api_key, Some(output_path))
            .await
            .unwrap();

        let result = std::fs::read(result_path);
        assert!(result.is_ok());
    }

    #[test]
    fn extracted_corp_codes() {
        let input_path = "examples/corp_codes/CORPCODE.xml";

        let result: Vec<CorpCodeItem> = extract_corp_codes(input_path).unwrap();

        assert_eq!(
            result[0..5],
            vec![
                CorpCodeItem {
                    corp_code: "00434003".to_string(),
                    corp_name: "다코".to_string(),
                    stock_code: "".to_string(),
                    modify_date: "20170630".to_string()
                },
                CorpCodeItem {
                    corp_code: "00434456".to_string(),
                    corp_name: "일산약품".to_string(),
                    stock_code: "".to_string(),
                    modify_date: "20170630".to_string()
                },
                CorpCodeItem {
                    corp_code: "00430964".to_string(),
                    corp_name: "굿앤엘에스".to_string(),
                    stock_code: "".to_string(),
                    modify_date: "20170630".to_string()
                },
                CorpCodeItem {
                    corp_code: "00432403".to_string(),
                    corp_name: "한라판지".to_string(),
                    stock_code: "".to_string(),
                    modify_date: "20170630".to_string()
                },
                CorpCodeItem {
                    corp_code: "00388953".to_string(),
                    corp_name: "크레디피아제이십오차유동화전문회사".to_string(),
                    stock_code: "".to_string(),
                    modify_date: "20170630".to_string()
                }
            ]
        )
    }

    #[test]
    fn read_saved_corp_code_list() {
        let output_path = "examples/corp_code_list.parquet";
        let corp_code_list = vec![
            CorpCodeItem {
                corp_code: "00434003".to_string(),
                corp_name: "다코".to_string(),
                stock_code: "".to_string(),
                modify_date: "20170630".to_string(),
            },
            CorpCodeItem {
                corp_code: "00434456".to_string(),
                corp_name: "일산약품".to_string(),
                stock_code: "".to_string(),
                modify_date: "20170630".to_string(),
            },
            CorpCodeItem {
                corp_code: "00430964".to_string(),
                corp_name: "굿앤엘에스".to_string(),
                stock_code: "".to_string(),
                modify_date: "20170630".to_string(),
            },
            CorpCodeItem {
                corp_code: "00432403".to_string(),
                corp_name: "한라판지".to_string(),
                stock_code: "".to_string(),
                modify_date: "20170630".to_string(),
            },
            CorpCodeItem {
                corp_code: "00388953".to_string(),
                corp_name: "크레디피아제이십오차유동화전문회사".to_string(),
                stock_code: "".to_string(),
                modify_date: "20170630".to_string(),
            },
        ];

        save_corp_code_list(output_path, &corp_code_list).unwrap();

        let result = LazyFrame::scan_parquet(output_path, Default::default());
        assert!(result.is_ok());
        assert!(result
            .unwrap()
            .select(&[col("corp_code"), col("corp_name")])
            .fetch(5)
            .is_ok())
    }
}
