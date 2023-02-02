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
    fn extract_cashflow_statement_date() {
        // Arrange
        let input = CASHFLOW_TABLE;
        // Act
        let result = get_table_columns(input).unwrap();
        // Assert
        assert_snapshot!(result, @r###"
        IFRS(연결) - []
        영업활동으로인한현금흐름 - [453829, 652870, 651054]
        당기순손익 - [217389, 264078, 399075]
        법인세비용차감전계속사업이익 - [0, 0, 0]
        현금유출이없는비용등가산 - [424268, 461506, 527991]
        퇴직급여 - [11716, 12902, 13603]
        종업원급여 - [0, 0, 0]
        주식보상비 - [0, 0, 0]
        대손상각비 - [0, 400, 180]
        감가상각비 - [265738, 271157, 312852]
        무형자산상각비 - [30238, 32199, 29622]
        계약부채전입액 - [0, 0, 0]
        반품(환불)부채전입액 - [0, 0, 0]
        배출부채전입액 - [0, 0, 0]
        충당부채전입액 - [0, 0, 0]
        외환손실 - [0, 0, 0]
        기타의대손상각비 - [0, 0, 0]
        금융원가 - [20345, 23068, 19177]
        이자비용 - [0, 0, 0]
        배당금지급 - [0, 0, 0]
        매출채권처분손실 - [0, 0, 0]
        당기손익-공정가치측정 금융자산관련손실 - [0, 0, 0]
        금융자산처분손실 - [0, 0, 0]
        금융자산평가손실 - [0, 0, 0]
        금융자산손상차손 - [0, 0, 0]
        파생상품손실 - [0, 0, 0]
        금융부채관련손실 - [0, 0, 0]
        사채상환손실 - [0, 0, 0]
        재고자산감모손실 - [0, 0, 0]
        재고자산폐기(처분)손실 - [0, 0, 0]
        자산처분(폐기)손실 - [1445, 877, 756]
        투자자산평가손실 - [0, 0, 0]
        자산재평가손실 - [0, 0, 0]
        자산손상차손 - [7852, 10008, 17357]
        지분법관련손실 - [0, 0, 0]
        종속기업관련손실 - [0, 0, 0]
        관계기업관련손익 - [0, 0, 0]
        법인세비용 - [86933, 99373, 134444]
        기타비용 - [0, 11523, 0]
        (현금유입이없는수익등차감) - [49842, 45321, 37435]
        외환이익 - [0, 0, 0]
        대손충당금환입액 - [1834, 0, 0]
        금융수익 - [38314, 37188, 24857]
        이자수익 - [0, 0, 0]
        배당금수익 - [1547, 1524, 1358]
        매출채권처분이익 - [0, 0, 0]
        당기손익-공정가치측정 금융자산관련이익 - [0, 0, 0]
        금융자산처분이익 - [0, 0, 0]
        금융자산평가이익 - [0, 0, 0]
        금융자산손상차손환입 - [0, 0, 0]
        파생상품이익 - [0, 0, 0]
        금융부채관련이익 - [0, 0, 0]
        사채상환이익 - [0, 0, 0]
        퇴직급여충당부채환입액 - [0, 0, 0]
        계약부채환입액 - [0, 0, 0]
        반품(환불)부채환입액 - [0, 0, 0]
        배출부채환입액 - [0, 0, 0]
        충당부채환입액 - [0, 0, 0]
        주식보상비환입 - [0, 0, 0]
        재고자산감모손실환입 - [0, 0, 0]
        재고자산폐기(처분)이익 - [0, 0, 0]
        자산처분(폐기)이익 - [3524, 1542, 3404]
        투자자산평가이익 - [0, 0, 0]
        자산재평가이익 - [0, 0, 0]
        자산손상차손환입 - [0, 0, 0]
        지분법관련이익 - [4130, 5065, 7296]
        종속회사관련이익 - [0, 0, 0]
        관계기업관련손익 - [0, 0, 0]
        법인세수익 - [0, 0, 0]
        기타수익 - [493, 0, 519]
        영업활동으로인한자산부채변동(운전자본변동) - []
        자산의감소(증가) - []
        부채의증가(감소) - []
        정부보조금등의변동 - [0, 0, 0]
        기타운전자본의변동 - []
        *영업에서창출된현금흐름 - [566358, 681488, 726762]
        기타영업활동으로인한현금흐름 - []
        이자수입 - [23064, 22202, 14067]
        이자지급(-) - []
        배당금수입 - [2418, 2437, 2990]
        배당금지급(-) - [0, 0, 0]
        법인세환입 - [0, 0, 0]
        법인세납부(-) - []
        중단영업관련현금흐름 - [0, 0, 0]
        투자활동으로인한현금흐름 - []
        투자활동으로인한현금유입액 - [63008, 155517, 254825]
        유동금융자산의감소 - [11355, 29836, 142179]
        장기금융상품의감소 - [45866, 121843, 102161]
        매도가능금융자산의감소 - [0, 0, 0]
        만기보유금융자산의감소 - [0, 0, 0]
        장기대여금의감소 - [0, 0, 0]
        파생상품의변동 - [0, 0, 0]
        기타비유동금융자산의감소 - [0, 0, 0]
        관계기업등지분관련투자자산의감소 - [121, 0, 192]
        유형자산의감소 - [5133, 3767, 3583]
        무형자산의감소 - [72, 70, 18]
        생물자산의감소 - [0, 0, 0]
        투자부동산의감소 - [0, 0, 0]
        기타투자활동으로인한현금유입액 - [460, 0, 6693]
        (투자활동으로인한현금유출액) - [462489, 691803, 585303]
        유동금융자산의증가 - [38736, 206993, 16664]
        장기금융상품의증가 - [127255, 80193, 69818]
        매도가능금융자산의증가 - [0, 0, 0]
        만기보유금융자산의증가 - [0, 0, 0]
        장기대여금의증가 - [0, 0, 0]
        파생상품의변동 - [0, 0, 0]
        기타비유동금융자산의증가 - [0, 0, 0]
        관계기업등지분관련투자자산의증가 - [128, 833, 471]
        유형자산의증가 - [253678, 375920, 471221]
        무형자산의증가 - [32499, 26798, 27069]
        생물자산의증가 - [0, 0, 0]
        투자부동산의증가 - [0, 0, 0]
        기타투자활동으로인한현금유출액 - [10194, 1066, 59]
        기타투자활동으로인한현금흐름 - [0, 0, 0]
        이자수입 - [0, 0, 0]
        이자지급(-) - [0, 0, 0]
        배당금수입 - [0, 0, 0]
        배당금지급(-) - [0, 0, 0]
        법인세환입 - [0, 0, 0]
        법인세납부(-) - [0, 0, 0]
        중단영업관련현금흐름 - [0, 0, 0]
        재무활동으로인한현금흐름 - []
        재무활동으로인한현금유입액 - [8658, 22139, 583]
        사채의증가 - [0, 0, 0]
        차입금의증가 - [8658, 22057, 583]
        미지급금의증가 - [0, 0, 0]
        유동성장기부채의증가 - [0, 0, 0]
        기타금융부채의증가 - [0, 0, 0]
        기타부채의증가 - [0, 0, 0]
        유상증자 - [0, 0, 0]
        자기주식의처분 - [0, 0, 0]
        주식매입선택권의행사 - [0, 0, 0]
        자본구성항목의증가 - [0, 82, 0]
        기타재무활동으로인한현금유입액 - [0, 0, 0]
        (재무활동으로인한현금유출액) - [7111, 8649, 35390]
        사채의감소 - [7094, 8649, 8947]
        차입금의감소 - [0, 0, 26169]
        미지급금의감소 - [0, 0, 0]
        유동성장기부채의감소 - [0, 0, 0]
        기타금융부채의감소 - [0, 0, 0]
        기타부채의감소 - [0, 0, 0]
        유상감자 - [0, 0, 0]
        자기주식의취득 - [0, 0, 0]
        자본구성항목의감소 - [17, 0, 273]
        기타재무활동으로인한현금유출액 - [0, 0, 0]
        기타재무활동으로인한현금흐름 - []
        이자수입 - [0, 0, 0]
        이자지급(-) - [0, 0, 0]
        배당금수입 - [0, 0, 0]
        배당금지급(-) - []
        법인세환입 - [0, 0, 0]
        법인세납부(-) - [0, 0, 0]
        중단영업관련현금흐름 - [0, 0, 0]
        영업투자재무활동기타현금흐름 - []
        연결범위변동으로인한현금의증가 - [0, 0, 0]
        환율변동효과 - []
        현금및현금성자산의증가 - []
        기초현금및현금성자산 - [303405, 268860, 293826]
        기말현금및현금성자산 - [268860, 293826, 390314]
        "###);
    }

    #[test]
    fn extract_comprehensive_income_statement_data() {
        // Arrange
        let input = COMP_INCOME_TABLE;
        // Act
        let result = get_table_columns(input).unwrap();
        // Assert
        assert_snapshot!(result, @r###"
        IFRS(연결) - []
        매출액 - [2304009, 2368070, 2796048]
        매출원가 - [1472395, 1444883, 1664113]
        매출총이익 - [831613, 923187, 1131935]
        판매비와관리비 - [553928, 563248, 615596]
        인건비 - [64226, 70429, 75568]
        유무형자산상각비 - [20408, 20857, 20731]
        연구개발비 - [199072, 211115, 224017]
        광고선전비 - [46145, 42690, 53760]
        판매비 - [117222, 114488, 131185]
        관리비 - [55131, 56787, 61926]
        기타원가성비용 - [0, 0, 0]
        기타 - [51724, 46883, 48409]
        영업이익 - [277685, 359939, 516339]
        영업이익(발표기준) - [277685, 359939, 516339]
        금융수익 - [101616, 122676, 85432]
        이자수익 - [26600, 19745, 12783]
        배당금수익 - [0, 0, 0]
        외환이익 - [67690, 92700, 65257]
        대손충당금환입액 - [0, 0, 0]
        매출채권처분이익 - [0, 0, 0]
        당기손익-공정가치측정 금융자산관련이익 - [0, 0, 0]
        금융자산처분이익 - [0, 0, 0]
        금융자산평가이익 - [0, 0, 0]
        금융자산손상차손환입 - [0, 0, 0]
        파생상품이익 - [7326, 10231, 7392]
        기타금융수익 - [0, 0, 0]
        금융원가 - [82749, 113181, 77046]
        이자비용 - [6864, 5830, 4315]
        외환손실 - [68524, 98686, 64861]
        대손상각비 - [0, 0, 0]
        당기손익-공정가치측정 금융자산관련손실 - [0, 0, 0]
        매출채권처분손실 - [0, 0, 0]
        금융자산처분손실 - [0, 0, 0]
        금융자산평가손실 - [0, 0, 0]
        금융자산손상차손 - [0, 0, 0]
        파생상품손실 - [7361, 8665, 7869]
        기타금융원가 - [0, 0, 0]
        기타수익 - [17787, 13841, 22057]
        이자수익 - [0, 0, 0]
        배당금수익 - [1547, 1524, 1358]
        외환이익 - [0, 0, 0]
        재고자산감모손실환입 - [0, 0, 0]
        재고자산폐기(처분)이익 - [0, 0, 0]
        당기손익-공정가치측정 금융자산평가이익 - [0, 0, 0]
        자산처분(폐기)이익 - [3524, 1542, 3404]
        자산평가이익 - [0, 0, 0]
        자산손상차손환입 - [0, 0, 0]
        파생상품이익 - [0, 0, 0]
        임대료수익 - [1534, 1471, 1328]
        로열티수익 - [0, 0, 0]
        수수료수익 - [0, 0, 0]
        대손충당금환입 - [0, 0, 0]
        충당부채환입액 - [0, 0, 0]
        기타 - [11182, 9303, 15967]
        기타비용 - [14147, 24889, 20560]
        이자비용 - [0, 0, 0]
        외환손실 - [0, 0, 0]
        재고자산감모손실 - [0, 0, 0]
        재고자산폐기(처분)손실 - [0, 0, 0]
        당기손익-공정가치측정 금융자산평가손실 - [0, 0, 0]
        자산처분(폐기)손실 - [1445, 877, 756]
        자산평가손실 - [0, 0, 0]
        자산손상차손 - [0, 0, 0]
        파생상품손실 - [0, 0, 0]
        기타대손상각비 - [0, 0, 0]
        충당부채전입액 - [0, 0, 0]
        기타 - [12702, 24012, 19804]
        종속기업,공동지배기업및관계기업관련손익 - [4130, 5065, 7296]
        지분법손익 - [4130, 5065, 7296]
        종속기업,공동지배기업및관계기업투자주식처분손익 - [0, 0, 0]
        종속기업,공동지배기업및관계기업투자주식손상관련손익 - [0, 0, 0]
        기타 - [0, 0, 0]
        세전계속사업이익 - [304322, 363451, 533518]
        법인세비용 - [86933, 99373, 134444]
        계속영업이익 - [217389, 264078, 399075]
        중단영업이익 - [0, 0, 0]
        당기순이익 - [217389, 264078, 399075]
        지배주주순이익 - [215051, 260908, 392438]
        비지배주주순이익 - [2338, 3170, 6637]
        "###);
    }

    #[test]
    fn extract_financial_position_statement_data() {
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
