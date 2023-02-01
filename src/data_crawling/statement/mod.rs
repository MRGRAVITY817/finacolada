mod constants;
mod test_mock;

use {
    self::constants::*,
    polars::{
        prelude::{DataFrame, NamedFrom, ParquetWriter},
        series::Series,
    },
    scraper::{ElementRef, Html, Selector},
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

fn get_table_columns(table_string: &str) -> anyhow::Result<String> {
    let table = Html::parse_fragment(table_string);
    let row_selector = Selector::parse("tr").unwrap();
    let head_selector = Selector::parse("th").unwrap();
    let result = table
        .select(&row_selector)
        .map(|row| {
            row.select(&head_selector)
                .next()
                .and_then(get_first_text)
                .unwrap_or("".to_string())
        })
        .collect::<Vec<_>>();
    Ok(result.join(", "))
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
        assert_snapshot!(result, @"IFRS(연결), 자산, 유동자산, 재고자산, 유동생물자산, 유동금융자산, 매출채권및기타유동채권, 당기법인세자산, 계약자산, 반품(환불)자산, 배출권, 기타유동자산, 현금및현금성자산, 매각예정비유동자산및처분자산집단, 비유동자산, 유형자산, 무형자산, 비유동생물자산, 투자부동산, 장기금융자산, 관계기업등지분관련투자자산, 장기매출채권및기타비유동채권, 이연법인세자산, 장기당기법인세자산, 계약자산, 반품(환불)자산, 배출권, 기타비유동자산, 기타금융업자산, 부채, 유동부채, 단기사채, 단기차입금, 유동성장기부채, 유동금융부채, 매입채무및기타유동채무, 유동종업원급여충당부채, 기타단기충당부채, 당기법인세부채, 계약부채, 반품(환불)부채, 배출부채, 기타유동부채, 매각예정으로분류된처분자산집단에포함된부채, 비유동부채, 사채, 장기차입금, 비유동금융부채, 장기매입채무및기타비유동채무, 비유동종업원급여충당부채, 기타장기충당부채, 이연법인세부채, 장기당기법인세부채, 계약부채, 반품(환불)부채, 배출부채, 기타비유동부채, 기타금융업부채, 자본, 지배기업주주지분, 자본금, 신종자본증권, 자본잉여금, 기타자본, 기타포괄손익누계액, 이익잉여금(결손금), 비지배주주지분")
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

        let result = get_row_data(selected.next().unwrap());

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
