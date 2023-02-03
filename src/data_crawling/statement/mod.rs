use std::collections::{HashMap, HashSet};

use polars::prelude::{LazyFrame, PolarsError};

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
        // Numbers are separated with comma. Remove them to parse it to u32.
        .map(|k| k.unwrap_or("0".to_string()).replace(",", "").parse())
        .collect()
}

fn get_statement_table(table_string: &str) -> Result<DataFrame, PolarsError> {
    let table = Html::parse_fragment(table_string);
    let row_selector = Selector::parse("tr").unwrap();
    let result = table
        .select(&row_selector)
        // We don't need first row.
        .skip(1)
        .enumerate()
        .map(|(i, row)| {
            let header = get_row_header(row).unwrap_or(format!("Untitled_field_{i}"));
            let data = get_row_data(row).unwrap_or(vec![0, 0, 0]);

            (header, data)
        })
        // Since we want to dedup data with same column name, use HashMap.
        .collect::<HashMap<String, Vec<u32>>>()
        .into_iter()
        .map(|(k, v)| Series::new(&k, &v))
        .collect::<Vec<_>>();

    DataFrame::new(result)
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
    fn extract_cashflow_statement_data() {
        // Arrange
        let input = CASHFLOW_TABLE;
        // Act
        let result = get_statement_table(input).unwrap();
        // Assert
        assert_snapshot!(result.get_column_names().join(", "), @"지분법관련이익, (현금유입이없는수익등차감), 반품(환불)부채전입액, 당기손익-공정가치측정 금융자산관련손실, 금융자산손상차손, 부채의증가(감소), 배출부채환입액, 배당금수입, 기타투자활동으로인한현금유출액, 사채의감소, 주식보상비환입, 미지급금의증가, 차입금의증가, 사채상환손실, 장기금융상품의감소, 현금유출이없는비용등가산, 재무활동으로인한현금흐름, 기타금융부채의감소, 생물자산의증가, 금융자산평가손실, 주식보상비, 기타투자활동으로인한현금흐름, 주식매입선택권의행사, 기타비유동금융자산의감소, 자산의감소(증가), 환율변동효과, 투자활동으로인한현금유입액, 금융원가, 자본구성항목의증가, 대손상각비, 투자자산평가손실, 배당금지급(-), 기타비유동금융자산의증가, 매출채권처분이익, 유동금융자산의감소, 금융수익, 금융자산평가이익, 법인세비용차감전계속사업이익, 무형자산의감소, 자산손상차손환입, 유상감자, 기타투자활동으로인한현금유입액, 유형자산의증가, 배당금수익, 관계기업관련손익, 현금및현금성자산의증가, 기초현금및현금성자산, 투자자산평가이익, 유동성장기부채의감소, 종업원급여, 중단영업관련현금흐름, 재고자산감모손실, 기타비용, 재고자산감모손실환입, 무형자산상각비, 매도가능금융자산의증가, (재무활동으로인한현금유출액), 만기보유금융자산의증가, 법인세환입, 지분법관련손실, 기타부채의감소, 외환이익, 자본구성항목의감소, 투자부동산의증가, 투자활동으로인한현금흐름, 충당부채전입액, 만기보유금융자산의감소, 재고자산폐기(처분)이익, 법인세수익, 영업투자재무활동기타현금흐름, 관계기업등지분관련투자자산의감소, 장기대여금의증가, 기타부채의증가, 배출부채전입액, *영업에서창출된현금흐름, 외환손실, 자산재평가이익, 기타운전자본의변동, 금융자산처분손실, 장기금융상품의증가, 반품(환불)부채환입액, 종속회사관련이익, 배당금지급, 미지급금의감소, 파생상품이익, 사채의증가, 기타금융부채의증가, 무형자산의증가, 영업활동으로인한현금흐름, 법인세비용, 자산손상차손, 퇴직급여충당부채환입액, 이자수입, 유동성장기부채의증가, 자산처분(폐기)이익, 기타재무활동으로인한현금흐름, 기타재무활동으로인한현금유출액, 대손충당금환입액, 이자비용, 매출채권처분손실, 이자수익, 기타수익, 관계기업등지분관련투자자산의증가, 유형자산의감소, 감가상각비, 자기주식의취득, 계약부채전입액, 퇴직급여, 사채상환이익, 재고자산폐기(처분)손실, 차입금의감소, 연결범위변동으로인한현금의증가, 법인세납부(-), 매도가능금융자산의감소, 기타의대손상각비, (투자활동으로인한현금유출액), 장기대여금의감소, 투자부동산의감소, 자산처분(폐기)손실, 자기주식의처분, 금융부채관련손실, 파생상품손실, 종속기업관련손실, 당기손익-공정가치측정 금융자산관련이익, 금융자산처분이익, 충당부채환입액, 이자지급(-), 유상증자, 유동금융자산의증가, 금융부채관련이익, 기타재무활동으로인한현금유입액, 자산재평가손실, 계약부채환입액, 생물자산의감소, 영업활동으로인한자산부채변동(운전자본변동), 기말현금및현금성자산, 당기순손익, 정부보조금등의변동, 파생상품의변동, 재무활동으로인한현금유입액, 기타영업활동으로인한현금흐름, 금융자산손상차손환입");
    }

    #[test]
    fn extract_comprehensive_income_statement_data() {
        // Arrange
        let input = COMP_INCOME_TABLE;
        // Act
        let result = get_statement_table(input).unwrap();
        // Assert
        assert_snapshot!(result.get_column_names().join(", "), @"재고자산폐기(처분)손실, 충당부채전입액, 영업이익(발표기준), 중단영업이익, 지배주주순이익, 기타원가성비용, 이자수익, 대손상각비, 인건비, 기타, 파생상품손실, 대손충당금환입, 금융자산처분손실, 재고자산감모손실, 자산처분(폐기)손실, 계속영업이익, 금융자산손상차손, 수수료수익, 영업이익, 종속기업,공동지배기업및관계기업투자주식처분손익, 법인세비용, 매출원가, 당기손익-공정가치측정 금융자산관련손실, 기타대손상각비, 재고자산감모손실환입, 기타비용, 외환손실, 로열티수익, 비지배주주순이익, 자산평가손실, 대손충당금환입액, 임대료수익, 세전계속사업이익, 매출액, 매출채권처분이익, 판매비와관리비, 매출채권처분손실, 기타금융수익, 광고선전비, 금융자산평가손실, 기타금융원가, 충당부채환입액, 지분법손익, 배당금수익, 연구개발비, 금융자산처분이익, 금융자산손상차손환입, 종속기업,공동지배기업및관계기업투자주식손상관련손익, 당기손익-공정가치측정 금융자산관련이익, 자산처분(폐기)이익, 이자비용, 금융수익, 금융원가, 기타수익, 재고자산폐기(처분)이익, 당기손익-공정가치측정 금융자산평가손실, 매출총이익, 파생상품이익, 판매비, 당기순이익, 종속기업,공동지배기업및관계기업관련손익, 관리비, 외환이익, 유무형자산상각비, 금융자산평가이익, 당기손익-공정가치측정 금융자산평가이익, 자산손상차손, 자산평가이익, 자산손상차손환입");
    }

    #[test]
    fn extract_financial_position_statement_data() {
        // Arrange
        let input = FIN_POSITION_TABLE;
        // Act
        let result = get_statement_table(input).unwrap();
        // Assert
        assert_snapshot!(result.get_column_names().join(", "), @"관계기업등지분관련투자자산, 기타비유동자산, 자산, 기타유동자산, 배출부채, 매입채무및기타유동채무, 배출권, 장기당기법인세자산, 유동자산, 사채, 기타금융업부채, 자본잉여금, 이연법인세자산, 현금및현금성자산, 반품(환불)부채, 유동성장기부채, 비유동부채, 투자부동산, 매출채권및기타유동채권, 비지배주주지분, 유동부채, 이연법인세부채, 이익잉여금(결손금), 자본금, 자본, 무형자산, 단기차입금, 계약부채, 유동종업원급여충당부채, 비유동금융부채, 기타유동부채, 장기매입채무및기타비유동채무, 당기법인세부채, 지배기업주주지분, 기타장기충당부채, 단기사채, 비유동자산, 반품(환불)자산, 장기당기법인세부채, 기타자본, 장기차입금, 유동생물자산, 부채, 유동금융부채, 당기법인세자산, 유동금융자산, 기타단기충당부채, 비유동종업원급여충당부채, 신종자본증권, 매각예정으로분류된처분자산집단에포함된부채, 계약자산, 비유동생물자산, 장기금융자산, 기타포괄손익누계액, 기타비유동부채, 재고자산, 장기매출채권및기타비유동채권, 기타금융업자산, 매각예정비유동자산및처분자산집단, 유형자산");
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
