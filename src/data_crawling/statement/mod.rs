mod constants;
mod test_mock;

use {
    self::constants::*,
    polars::{
        prelude::{DataFrame, NamedFrom, ParquetWriter},
        series::Series,
    },
    scraper::{ElementRef, Html, Selector},
    std::num::ParseIntError,
};

/// Get first non-empty text from nested DOM structure.
fn get_first_text(element: ElementRef) -> Option<String> {
    match element.text().next() {
        // If the first child is non-empty text, that's the one we're looking for.
        Some(text_str) if text_str.trim().len() > 0 => Some(text_str.trim().to_string()),
        // If the first child was empty text, try again with the second child.
        Some(_) => element
            .children()
            .skip(1)
            .next()
            .and_then(|second_child| get_first_text(ElementRef::wrap(second_child)?)),
        // If the first child isn't a text, try the element inside the first child.
        _ => element
            .first_child()
            .and_then(|first_child| get_first_text(ElementRef::wrap(first_child)?)),
    }
}

fn get_row_header(row_element: ElementRef) -> Option<String> {
    let head_selector = Selector::parse("th").unwrap();
    row_element
        .select(&head_selector)
        .next()
        .and_then(get_first_text)
}

fn get_row_data(row_element: ElementRef) -> Result<Vec<u32>, ParseIntError> {
    let head_selector = Selector::parse("td").unwrap();
    row_element
        .select(&head_selector)
        .take(3)
        .map(get_first_text)
        .map(|k| k.unwrap_or("0".to_string()).replace(",", "").parse())
        .collect()
}

fn get_table_columns(table_string: &str) -> anyhow::Result<String> {
    let table = Html::parse_fragment(table_string);
    let row_selector = Selector::parse("tr").unwrap();
    let result = table
        .select(&row_selector)
        .enumerate()
        .map(|(i, row)| {
            let header = get_row_header(row).unwrap_or(format!("Untitled_field_{i}"));
            let data = get_row_data(row).unwrap_or(vec![]);
            format!(
                "{} - [{}]",
                header,
                data.into_iter()
                    .map(|datum| datum.to_string())
                    .reduce(|acc, curr| format!("{acc}, {curr}"))
                    .unwrap_or("".to_string())
            )
        })
        .collect::<Vec<_>>();
    Ok(result.join("\r\n"))
}

fn parse_statement_table(
    table_string: &str,
    years: &[&str],
    columns: &[&str],
    file_path: &str,
) -> anyhow::Result<()> {
    // should provide only 3 years
    if years.len().ne(&3) {
        return Err(anyhow::Error::msg("Only 3 years are acceptable."));
    }
    // extract table from given string
    let span_selector = Selector::parse("span").unwrap();
    match table_extract::Table::find_first(table_string) {
        Some(table) => {
            // iterate every row
            let series_vec = table
                .into_iter()
                .zip(columns)
                .map(|(row, col)| {
                    // 3. iterate every row data, and save these as u32
                    let parsed_row = row.into_iter().take(3).map(|val| -> u32 {
                        match Html::parse_fragment(&val).select(&span_selector).next() {
                            Some(neg_number) => {
                                neg_number.inner_html().to_string().parse().unwrap_or(0)
                            }
                            _ => {
                                if val.contains("&nbsp;") {
                                    return 0;
                                }
                                val.parse().unwrap_or(0)
                            }
                        }
                    });
                    Series::new(col, parsed_row.collect::<Vec<_>>())
                })
                .collect::<Vec<_>>();
            // create dataframe
            let mut df = DataFrame::new(series_vec)?;
            // save it as parquet
            let mut file = std::fs::File::create(file_path)?;
            ParquetWriter::new(&mut file).finish(&mut df)?;
            // if successful, return Ok(())
            Ok(())
        }
        _ => Err(anyhow::Error::msg("Invalid table string")),
    }
}

pub async fn get_financial_statement(
    query_client: &reqwest::Client,
    ticker: &str,
) -> anyhow::Result<()> {
    let document = query_client
        .get("http://comp.fnguide.com/SVO2/ASP/SVD_Finance.asp")
        .query(&[("pGB", "1"), ("gicode", &format!("A{ticker}"))])
				.header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.77 Safari/537.36")
        .send()
        .await?
        .text()
        .await?;

    // Get <table /> element using selector
    let document = Html::parse_document(&document);
    let table_selector = Selector::parse("table").unwrap();
    let tables = document
        .select(&table_selector)
        .map(|selected| selected.html())
        .collect::<Vec<_>>();
    let years = ["2019", "2020", "2021"];
    // Table 1. Comprehensive income statement (annual)
    parse_statement_table(
        &tables[0],
        &years,
        &COMP_INCOME_STATEMENT_COLS,
        &COMP_INCOME_STATEMENT_FILE_PATH,
    )?;
    // Table 3. Financial position statement (annual)
    parse_statement_table(
        &tables[2],
        &years,
        &FIN_POSITION_STATEMENT_COLS,
        &FIN_POSITION_STATEMENT_FILE_PATH,
    )?;
    // Table 5. Cashflow statement (annual)
    parse_statement_table(
        &tables[4],
        &years,
        &CASHFLOW_STATEMENT_COLS,
        &CASHFLOW_STATEMENT_FILE_PATH,
    )?;

    Ok(())
}

#[cfg(test)]
mod test {
    use {
        super::{test_mock::*, *},
        insta::assert_snapshot,
    };

    // #[tokio::test]
    // async fn has_tables() {
    //     // Arrange
    //     let client = reqwest::Client::new();
    //     let samsung_ticker = "005930";
    //     // Act
    //     let result = get_financial_statement(&client, samsung_ticker)
    //         .await
    //         .unwrap();
    //     // Assert
    //     // assert_eq!(result.len(), 6);
    //     // assert_snapshot!(result[0], @"")
    // }

    // #[test]
    // fn parsed_statement_table_should_have_only_3_years_data() {
    //     // Arrange
    //     let table_string = TABLE_STRING;
    //     let file_path = "examples/statement_table.parquet";
    //     // Act
    //     // let result = parse_statement_table(table_string, file_path).unwrap();
    //     // Assert
    //     // assert_eq!(result.len(), 3);
    // }

    #[test]
    fn should_extract_the_financial_position_table_columns() {
        // Arrange
        let input = FIN_POSITION_TABLE;
        // Act
        let result = get_table_columns(input).unwrap();
        // Assert
        assert_snapshot!(result, @r###"
        IFRS(연결) - []
        자산 - [3525645, 3782357, 4266212]
        유동자산 - [1813853, 1982156, 2181632]
        재고자산 - [267665, 320431, 413844]
        유동생물자산 - [0, 0, 0]
        유동금융자산 - [818937, 952703, 851188]
        매출채권및기타유동채권 - [393105, 345696, 452107]
        당기법인세자산 - [0, 0, 0]
        계약자산 - [0, 0, 0]
        반품(환불)자산 - [0, 0, 0]
        배출권 - [0, 0, 0]
        기타유동자산 - [65286, 60206, 74179]
        현금및현금성자산 - [268860, 293826, 390314]
        매각예정비유동자산및처분자산집단 - [0, 9294, 0]
        비유동자산 - [1711792, 1800201, 2084580]
        유형자산 - [1198255, 1289529, 1499285]
        무형자산 - [207035, 184685, 202362]
        비유동생물자산 - [0, 0, 0]
        투자부동산 - [0, 0, 0]
        장기금융자산 - [99697, 137782, 154912]
        관계기업등지분관련투자자산 - [75916, 80768, 89323]
        장기매출채권및기타비유동채권 - [7576, 4870, 12279]
        이연법인세자산 - [45050, 42750, 42612]
        장기당기법인세자산 - [0, 0, 0]
        계약자산 - [0, 0, 0]
        반품(환불)자산 - [0, 0, 0]
        배출권 - [0, 0, 0]
        기타비유동자산 - [78263, 59818, 83807]
        기타금융업자산 - [0, 0, 0]
        부채 - [896841, 1022877, 1217212]
        유동부채 - [637828, 756044, 881171]
        단기사채 - [0, 0, 0]
        단기차입금 - [143935, 165534, 136878]
        유동성장기부채 - [8461, 7161, 13300]
        유동금융부채 - [0, 0, 0]
        매입채무및기타유동채무 - [409777, 469431, 582603]
        유동종업원급여충당부채 - [0, 0, 0]
        기타단기충당부채 - [40686, 43496, 53729]
        당기법인세부채 - [13878, 44303, 67491]
        계약부채 - [0, 0, 0]
        반품(환불)부채 - [0, 0, 0]
        배출부채 - [0, 0, 0]
        기타유동부채 - [21091, 22731, 27171]
        매각예정으로분류된처분자산집단에포함된부채 - [0, 3387, 0]
        비유동부채 - [259013, 266834, 336041]
        사채 - [9753, 9481, 5082]
        장기차입금 - [0, 0, 15]
        비유동금융부채 - [21972, 19997, 28647]
        장기매입채무및기타비유동채무 - [21842, 16829, 29914]
        비유동종업원급여충당부채 - [4708, 4645, 4659]
        기타장기충당부채 - [6111, 10514, 23070]
        이연법인세부채 - [170538, 188108, 231982]
        장기당기법인세부채 - [0, 0, 0]
        계약부채 - [0, 0, 0]
        반품(환불)부채 - [0, 0, 0]
        배출부채 - [0, 0, 0]
        기타비유동부채 - [24089, 17259, 12672]
        기타금융업부채 - [0, 0, 0]
        자본 - [2628804, 2759480, 3048999]
        지배기업주주지분 - [2549155, 2676703, 2962377]
        자본금 - [8975, 8975, 8975]
        신종자본증권 - [0, 0, 0]
        자본잉여금 - [44039, 44039, 44039]
        기타자본 - [604, 267, 868]
        기타포괄손익누계액 - []
        이익잉여금(결손금) - [2545829, 2710682, 2930648]
        비지배주주지분 - [79649, 82777, 86622]
        "###)
    }

    #[test]
    fn extract_row_header() {
        let input = r#"
				<table>
					<tr>
						<th>Hello</th>
						<td>1</td>
						<td>2</td>
						<td>3</td>
					</tr>
				</table>
			"#;
        let tr_selector = Selector::parse("tr").unwrap();
        let fragment = Html::parse_fragment(input);
        let selected = fragment.select(&tr_selector);
        for row_element in selected {
            let result = get_row_header(row_element).unwrap();
            assert_eq!(result, "Hello")
        }
    }

    #[test]
    fn extract_row_data_as_vector() {
        let input = r#"
				<table>
					<tr>
						<th>Hello</th>
						<td>1</td>
						<td>2</td>
						<td>3</td>
					</tr>
				</table>
			"#;
        let tr_selector = Selector::parse("tr").unwrap();
        let fragment = Html::parse_fragment(input);
        let mut selected = fragment.select(&tr_selector);

        let result = get_row_data(selected.next().unwrap()).unwrap();

        assert_eq!(result, vec![1, 2, 3])
    }

    #[test]
    fn extract_nested_row_data_as_vector() {
        let input = r#"
				<table>
					<tr>
						<th>Hello</th>
						<td>1</td>
						<td><span>2</span></td>
						<td>3</td>
					</tr>
				</table>
			"#;
        let tr_selector = Selector::parse("tr").unwrap();
        let fragment = Html::parse_fragment(input);
        let mut selected = fragment.select(&tr_selector);

        let result = get_row_data(selected.next().unwrap()).unwrap();

        assert_eq!(result, vec![1, 2, 3])
    }

    #[test]
    fn should_extract_first_string() {
        let th_string = r#"
				<table>
					<th scope="col" class="clf tbold">
					<div>hello</div>
					</th>
				</table>
				"#;
        let th_selector = Selector::parse("th").unwrap();
        let fragment = Html::parse_fragment(th_string);
        for th_element in fragment.select(&th_selector) {
            let result = get_first_text(th_element).unwrap();
            assert_snapshot!(result, @"hello")
        }
    }
}
