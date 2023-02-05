mod constants;
mod test_mock;

use {
    polars::{prelude::*, series::Series},
    scraper::{ElementRef, Html, Selector},
    std::{collections::HashMap, num::ParseIntError},
};

async fn get_latest_stock_price_and_share(
    query_client: &reqwest::Client,
    ticker: &str,
) -> anyhow::Result<(u32, u64)> {
    let html_text= query_client
        .get("http://comp.fnguide.com/SVO2/ASP/SVD_main.asp")
        .query(&[("pGB", "1"), ("gicode", &format!("A{ticker}"))])
				.header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.77 Safari/537.36")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&html_text);

    // 1. Stock price
    let selector = Selector::parse("#svdMainChartTxt11").unwrap();
    let stock_price = match document.select(&selector).next() {
        Some(price) => price
            .inner_html()
            .replace(",", "")
            .parse::<u32>()
            .map_err(|_| anyhow::Error::msg("Cannot parse the stock price as number.")),
        None => Err(anyhow::Error::msg(
            "Cannot find stock price of given ticker.",
        )),
    }?;

    // 2. Stock shares
    let selector =
        Selector::parse("#svdMainGrid1 > table > tbody > tr:nth-child(7) > td:nth-child(2)")
            .unwrap();
    let stock_share = match document.select(&selector).next() {
        Some(share) => share
            .inner_html()
            .split("/")
            .nth(0)
            .unwrap_or("0")
            .replace(",", "")
            .parse::<u64>()
            .map_err(|_| anyhow::Error::msg("Cannot parse the stock share as number.")),
        None => Err(anyhow::Error::msg(
            "Cannot find stock share of given ticker.",
        )),
    }?;

    Ok((stock_price, stock_share))
}

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
    let years = Series::new("연도", ["2019", "2020", "2021"]);
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
        .chain([years])
        .collect::<Vec<_>>();

    DataFrame::new(result)
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
    let comp_income = get_statement_table(&tables[0])?;
    let fin_position = get_statement_table(&tables[2])?;
    let cashflow = get_statement_table(&tables[4])?;

    let mut financial_statement = comp_income
        .inner_join(&fin_position, ["연도"], ["연도"])?
        .inner_join(&cashflow, ["연도"], ["연도"])?;

    let mut file =
        std::fs::File::create(format!("assets/financial_statements/{ticker}_FS.parquet"))?;
    ParquetWriter::new(&mut file).finish(&mut financial_statement)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use {
        super::{test_mock::*, *},
        insta::assert_snapshot,
    };

    #[tokio::test]
    async fn samsung_financial_statement() {
        // Arrange
        let client = reqwest::Client::new();
        let samsung_ticker = "005930";
        // Act
        get_financial_statement(&client, samsung_ticker)
            .await
            .unwrap();
        // Assert
        let samsung_fs = LazyFrame::scan_parquet(
            "assets/financial_statements/005930_FS.parquet",
            Default::default(),
        );
        assert!(samsung_fs.is_ok());
        assert_snapshot!(
            samsung_fs
            .unwrap()
            .filter(col("자본금").lt(lit(20000)))
            .select(&[col("매출액"),col("매출원가"),col("매출총이익"),col("판매비와관리비"),col("인건비"),col("유무형자산상각비")])
            .collect()
            .unwrap()
            .to_string(), 
            @r###"
        shape: (3, 6)
        ┌─────────┬────────────┬────────────┬────────────────┬─────────┬──────────────────┐
        │ 매출액  ┆ 매출원가   ┆ 매출총이익 ┆ 판매비와관리비 ┆ 인건비  ┆ 유무형자산상각비 │
        │ ---     ┆ ---        ┆ ---        ┆ ---            ┆ ---     ┆ ---              │
        │ u32     ┆ u32        ┆ u32        ┆ u32            ┆ u32     ┆ u32              │
        ╞═════════╪════════════╪════════════╪════════════════╪═════════╪══════════════════╡
        │ 2304009 ┆ 1472395    ┆ 831613     ┆ 553928         ┆ 64226   ┆ 20408            │
        ├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ 2368070 ┆ 1444883    ┆ 923187     ┆ 563248         ┆ 70429   ┆ 20857            │
        ├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
        │ 2796048 ┆ 1664113    ┆ 1131935    ┆ 615596         ┆ 75568   ┆ 20731            │
        └─────────┴────────────┴────────────┴────────────────┴─────────┴──────────────────┘
        "###)
    }

    #[test]
    fn extract_cashflow_statement_data() {
        // Arrange
        let input = CASHFLOW_TABLE;
        // Act
        let result = get_statement_table(input).unwrap();
        // Assert
        assert_snapshot!(result.get_column_names().join(", "), @"대손상각비, 관계기업등지분관련투자자산의증가, 기타비용, 투자활동으로인한현금유입액, 기타비유동금융자산의증가, 기타투자활동으로인한현금유출액, 유형자산의감소, 지분법관련손실, 자산재평가손실, 금융부채관련이익, 현금및현금성자산의증가, 기타의대손상각비, 반품(환불)부채환입액, 외환이익, 기타수익, 기타비유동금융자산의감소, (투자활동으로인한현금유출액), 이자수입, 미지급금의감소, 반품(환불)부채전입액, 유상증자, 당기순손익, 생물자산의감소, 주식보상비, 재무활동으로인한현금흐름, 재고자산감모손실환입, 투자부동산의감소, 재무활동으로인한현금유입액, 영업활동으로인한자산부채변동(운전자본변동), 자본구성항목의감소, 장기대여금의증가, 관계기업관련손익, 감가상각비, 유동성장기부채의증가, 이자비용, 종업원급여, 현금유출이없는비용등가산, 미지급금의증가, 배출부채전입액, 자산처분(폐기)이익, 금융자산처분이익, 금융수익, 이자지급(-), 법인세비용차감전계속사업이익, 법인세환입, 자본구성항목의증가, 파생상품이익, 만기보유금융자산의증가, 무형자산의증가, 매출채권처분손실, 투자부동산의증가, 법인세비용, 지분법관련이익, 자산손상차손환입, 파생상품손실, 금융원가, 종속회사관련이익, 차입금의증가, 기타투자활동으로인한현금유입액, 금융자산손상차손환입, 배당금지급(-), 장기금융상품의감소, 기타부채의감소, 배당금수입, 차입금의감소, 계약부채전입액, 외환손실, 배당금수익, 투자자산평가이익, 금융자산손상차손, 관계기업등지분관련투자자산의감소, 계약부채환입액, 영업활동으로인한현금흐름, (재무활동으로인한현금유출액), 파생상품의변동, 기타금융부채의감소, 기초현금및현금성자산, 법인세납부(-), 정부보조금등의변동, 매도가능금융자산의감소, 무형자산의감소, 유상감자, 자산의감소(증가), 금융자산평가이익, 대손충당금환입액, 기타영업활동으로인한현금흐름, 배당금지급, 유동금융자산의증가, 재고자산폐기(처분)이익, 자산손상차손, 기타운전자본의변동, 생물자산의증가, 유동금융자산의감소, 사채상환이익, 자산재평가이익, 만기보유금융자산의감소, 기타부채의증가, 재고자산감모손실, 장기금융상품의증가, 기타재무활동으로인한현금유입액, 투자자산평가손실, 매출채권처분이익, 투자활동으로인한현금흐름, 유동성장기부채의감소, 기타재무활동으로인한현금흐름, 퇴직급여충당부채환입액, 자산처분(폐기)손실, 법인세수익, 기타재무활동으로인한현금유출액, 사채의증가, 환율변동효과, 연결범위변동으로인한현금의증가, 금융자산평가손실, 배출부채환입액, 기타금융부채의증가, 사채상환손실, 자기주식의처분, 유형자산의증가, 사채의감소, 당기손익-공정가치측정 금융자산관련손실, 장기대여금의감소, 기말현금및현금성자산, 주식매입선택권의행사, 영업투자재무활동기타현금흐름, 중단영업관련현금흐름, (현금유입이없는수익등차감), 종속기업관련손실, 충당부채환입액, 매도가능금융자산의증가, 퇴직급여, 당기손익-공정가치측정 금융자산관련이익, 주식보상비환입, 이자수익, *영업에서창출된현금흐름, 무형자산상각비, 재고자산폐기(처분)손실, 부채의증가(감소), 충당부채전입액, 기타투자활동으로인한현금흐름, 자기주식의취득, 금융부채관련손실, 금융자산처분손실, 연도");
    }

    #[test]
    fn extract_comprehensive_income_statement_data() {
        // Arrange
        let input = COMP_INCOME_TABLE;
        // Act
        let result = get_statement_table(input).unwrap();
        // Assert
        assert_snapshot!(result.get_column_names().join(", "), @"충당부채전입액, 판매비, 금융자산손상차손환입, 파생상품이익, 자산손상차손, 법인세비용, 중단영업이익, 당기순이익, 비지배주주순이익, 파생상품손실, 매출채권처분이익, 금융자산평가손실, 금융자산손상차손, 유무형자산상각비, 자산평가손실, 대손충당금환입액, 자산처분(폐기)손실, 수수료수익, 배당금수익, 매출액, 금융원가, 재고자산폐기(처분)손실, 기타비용, 기타, 대손충당금환입, 매출총이익, 자산처분(폐기)이익, 외환이익, 대손상각비, 종속기업,공동지배기업및관계기업투자주식처분손익, 관리비, 종속기업,공동지배기업및관계기업투자주식손상관련손익, 당기손익-공정가치측정 금융자산평가손실, 충당부채환입액, 당기손익-공정가치측정 금융자산관련손실, 광고선전비, 연구개발비, 금융자산처분손실, 지배주주순이익, 매출채권처분손실, 기타원가성비용, 지분법손익, 기타금융수익, 재고자산감모손실환입, 금융수익, 임대료수익, 자산손상차손환입, 당기손익-공정가치측정 금융자산관련이익, 금융자산평가이익, 기타대손상각비, 판매비와관리비, 금융자산처분이익, 영업이익, 기타금융원가, 재고자산감모손실, 종속기업,공동지배기업및관계기업관련손익, 계속영업이익, 자산평가이익, 이자수익, 세전계속사업이익, 외환손실, 당기손익-공정가치측정 금융자산평가이익, 매출원가, 영업이익(발표기준), 재고자산폐기(처분)이익, 로열티수익, 인건비, 이자비용, 기타수익, 연도");
    }

    #[test]
    fn extract_financial_position_statement_data() {
        // Arrange
        let input = FIN_POSITION_TABLE;
        // Act
        let result = get_statement_table(input).unwrap();
        // Assert
        assert_snapshot!(result.get_column_names().join(", "), @"장기금융자산, 계약부채, 비유동금융부채, 사채, 배출권, 관계기업등지분관련투자자산, 이연법인세자산, 당기법인세자산, 투자부동산, 기타유동자산, 자산, 기타단기충당부채, 장기차입금, 자본금, 매출채권및기타유동채권, 유동금융자산, 이익잉여금(결손금), 유동종업원급여충당부채, 매각예정으로분류된처분자산집단에포함된부채, 단기사채, 비유동생물자산, 반품(환불)부채, 장기매출채권및기타비유동채권, 매입채무및기타유동채무, 비지배주주지분, 유동생물자산, 당기법인세부채, 신종자본증권, 단기차입금, 기타자본, 기타장기충당부채, 지배기업주주지분, 장기당기법인세자산, 기타금융업자산, 기타비유동자산, 재고자산, 유형자산, 부채, 기타금융업부채, 자본잉여금, 무형자산, 자본, 계약자산, 유동부채, 기타비유동부채, 기타포괄손익누계액, 장기당기법인세부채, 유동자산, 비유동종업원급여충당부채, 배출부채, 유동금융부채, 유동성장기부채, 비유동부채, 장기매입채무및기타비유동채무, 이연법인세부채, 반품(환불)자산, 비유동자산, 매각예정비유동자산및처분자산집단, 현금및현금성자산, 기타유동부채, 연도");
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
    fn extract_hello_as_first_text() {
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

    #[tokio::test]
    async fn latest_samsung_stock_price() {
        let ticker = "005930";
        let client = reqwest::Client::new();

        let result = get_latest_stock_price_and_share(&client, ticker)
            .await
            .unwrap();

        assert_snapshot!(result.0.to_string(), @"63800");
    }

    #[tokio::test]
    async fn latest_samsung_stock_share() {
        let ticker = "005930";
        let client = reqwest::Client::new();

        let result = get_latest_stock_price_and_share(&client, ticker)
            .await
            .unwrap();

        assert_snapshot!(result.1.to_string(), @"5969782550");
    }

    #[tokio::test]
    async fn latest_metrics_for_samsung() {
        let ticker = "005930";
        let client = reqwest::Client::new();

        let result: Metrics = get_latest_computed_metrics(&client, ticker).await.unwrap();

        assert_eq!(
            result,
            Metrics {
                per: 0,
                pbr: 0,
                pcr: 0,
                psr: 0
            }
        )
    }
}
