mod constants;

use {
    self::constants::*,
    polars::{
        prelude::{DataFrame, NamedFrom, ParquetWriter},
        series::Series,
    },
    scraper::{Html, Selector},
};

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
    use {super::*, insta::assert_snapshot};

    #[tokio::test]
    async fn has_tables() {
        // Arrange
        let client = reqwest::Client::new();
        let samsung_ticker = "005930";
        // Act
        let result = get_financial_statement(&client, samsung_ticker)
            .await
            .unwrap();
        // Assert
        assert_eq!(result.len(), 6);
        assert_snapshot!(result[0], @"")
    }

    #[test]
    fn parsed_statement_table_should_have_only_3_years_data() {
        // Arrange
        let table_string = TABLE_STRING;
        let file_path = "examples/statement_table.parquet";
        // Act
        let result = parse_statement_table(table_string, file_path).unwrap();
        // Assert
        assert_eq!(result.len(), 3);
    }

    const TABLE_STRING: &'static str = r#"
<table  class="us_table_ty1 h_fix zigbg_no" >
<caption class="cphidden">재무상태표</caption>
<colgroup>
		<col style="width: 24%;">
				
						<col >
				
						<col >
				
						<col >
				
						<col >
				
</colgroup>
<thead>
		<tr>
				<th scope="col" class="clf tbold">IFRS(연결)</th>
				
						<th scope="col">2019/12</th>
				
						<th scope="col">2020/12</th>
				
						<th scope="col">2021/12</th>
				
						<th scope="col">2022/09</th>
				
				
		</tr>
</thead>
<tbody>
		
								<tr class="rwf rowBold">
										
														<th scope="row" class="l clf">
															<div class="th_b">자산</div>
														</th>
												
														<td class="r" title="3,525,644.97">3,525,645</td>
												
														<td class="r" title="3,782,357.18">3,782,357</td>
												
														<td class="r" title="4,266,211.58">4,266,212</td>
												
														<td class="r cle"  title="4,702,784.09">4,702,784</td>
												
								</tr>
		
								<tr id="p_grid2_2" class="rwf acd_dep_start_close ">
										
														<th scope="row" class="l clf">
															<div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;유동자산</span><a id="grid2_2" href="javascript:foldOpen('grid2_2');" class=" btn_acdopen"><span class="blind" id="span_grid2_2">계산에 참여한 계정 펼치기</span></a></div>
														</th>
												
														<td class="r" title="1,813,852.60">1,813,853</td>
												
														<td class="r" title="1,982,155.79">1,982,156</td>
												
														<td class="r" title="2,181,631.85">2,181,632</td>
												
														<td class="r cle"  title="2,508,806.37">2,508,806</td>
												
								</tr>
		
								<tr  class="c_grid2_2 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;재고자산</th>
												
														<td class="r" title="267,664.64">267,665</td>
												
														<td class="r" title="320,431.45">320,431</td>
												
														<td class="r" title="413,844.04">413,844</td>
												
														<td class="r cle"  title="573,198.48">573,198</td>
												
								</tr>
		
								<tr  class="c_grid2_2 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동생물자산</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_2 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동금융자산</th>
												
														<td class="r" title="818,937.04">818,937</td>
												
														<td class="r" title="952,702.65">952,703</td>
												
														<td class="r" title="851,187.77">851,188</td>
												
														<td class="r cle"  title="843,006.78">843,007</td>
												
								</tr>
		
								<tr  class="c_grid2_2 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매출채권및기타유동채권</th>
												
														<td class="r" title="393,104.63">393,105</td>
												
														<td class="r" title="345,695.97">345,696</td>
												
														<td class="r" title="452,106.72">452,107</td>
												
														<td class="r cle"  title="533,932.43">533,932</td>
												
								</tr>
		
								<tr  class="c_grid2_2 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;당기법인세자산</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_2 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약자산</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_2 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)자산</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_2 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출권</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_2 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타유동자산</th>
												
														<td class="r" title="65,286.30">65,286</td>
												
														<td class="r" title="60,205.62">60,206</td>
												
														<td class="r" title="74,179.17">74,179</td>
												
														<td class="r cle"  title="113,514.47">113,514</td>
												
								</tr>
		
								<tr  class="c_grid2_2 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;현금및현금성자산</th>
												
														<td class="r" title="268,859.99">268,860</td>
												
														<td class="r" title="293,825.78">293,826</td>
												
														<td class="r" title="390,314.15">390,314</td>
												
														<td class="r cle"  title="445,154.21">445,154</td>
												
								</tr>
		
								<tr  class="c_grid2_2 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매각예정비유동자산및처분자산집단</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="9,294.32">9,294</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr id="p_grid2_3" class="rwf acd_dep_start_close ">
										
														<th scope="row" class="l clf">
															<div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;비유동자산</span><a id="grid2_3" href="javascript:foldOpen('grid2_3');" class=" btn_acdopen"><span class="blind" id="span_grid2_3">계산에 참여한 계정 펼치기</span></a></div>
														</th>
												
														<td class="r" title="1,711,792.37">1,711,792</td>
												
														<td class="r" title="1,800,201.39">1,800,201</td>
												
														<td class="r" title="2,084,579.73">2,084,580</td>
												
														<td class="r cle"  title="2,193,977.72">2,193,978</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유형자산</th>
												
														<td class="r" title="1,198,254.74">1,198,255</td>
												
														<td class="r" title="1,289,528.92">1,289,529</td>
												
														<td class="r" title="1,499,285.39">1,499,285</td>
												
														<td class="r cle"  title="1,603,435.68">1,603,436</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;무형자산</th>
												
														<td class="r" title="207,035.04">207,035</td>
												
														<td class="r" title="184,685.02">184,685</td>
												
														<td class="r" title="202,362.44">202,362</td>
												
														<td class="r cle"  title="214,848.57">214,849</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;비유동생물자산</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;투자부동산</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기금융자산</th>
												
														<td class="r" title="99,697.16">99,697</td>
												
														<td class="r" title="137,781.85">137,782</td>
												
														<td class="r" title="154,911.83">154,912</td>
												
														<td class="r cle"  title="127,871.00">127,871</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;관계기업등지분관련투자자산</th>
												
														<td class="r" title="75,916.12">75,916</td>
												
														<td class="r" title="80,767.79">80,768</td>
												
														<td class="r" title="89,322.51">89,323</td>
												
														<td class="r cle"  title="108,527.74">108,528</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기매출채권및기타비유동채권</th>
												
														<td class="r" title="7,576.23">7,576</td>
												
														<td class="r" title="4,869.53">4,870</td>
												
														<td class="r" title="12,278.53">12,279</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이연법인세자산</th>
												
														<td class="r" title="45,050.49">45,050</td>
												
														<td class="r" title="42,750.00">42,750</td>
												
														<td class="r" title="42,612.14">42,612</td>
												
														<td class="r cle"  title="54,881.63">54,882</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기당기법인세자산</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약자산</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)자산</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출권</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_3 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타비유동자산</th>
												
														<td class="r" title="78,262.59">78,263</td>
												
														<td class="r" title="59,818.28">59,818</td>
												
														<td class="r" title="83,806.89">83,807</td>
												
														<td class="r cle"  title="84,413.10">84,413</td>
												
								</tr>
		
								<tr class="rwf ">
										
														<th scope="row" class="l clf">
															<div class="">&nbsp;&nbsp;&nbsp;기타금융업자산</div>
														</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr class="rwf rowBold">
										
														<th scope="row" class="l clf">
															<div class="th_b">부채</div>
														</th>
												
														<td class="r" title="896,840.76">896,841</td>
												
														<td class="r" title="1,022,877.02">1,022,877</td>
												
														<td class="r" title="1,217,212.27">1,217,212</td>
												
														<td class="r cle"  title="1,253,715.20">1,253,715</td>
												
								</tr>
		
								<tr id="p_grid2_6" class="rwf acd_dep_start_close ">
										
														<th scope="row" class="l clf">
															<div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;유동부채</span><a id="grid2_6" href="javascript:foldOpen('grid2_6');" class=" btn_acdopen"><span class="blind" id="span_grid2_6">계산에 참여한 계정 펼치기</span></a></div>
														</th>
												
														<td class="r" title="637,827.64">637,828</td>
												
														<td class="r" title="756,043.51">756,044</td>
												
														<td class="r" title="881,171.33">881,171</td>
												
														<td class="r cle"  title="852,856.69">852,857</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;단기사채</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;단기차입금</th>
												
														<td class="r" title="143,934.68">143,935</td>
												
														<td class="r" title="165,534.29">165,534</td>
												
														<td class="r" title="136,877.93">136,878</td>
												
														<td class="r cle"  title="76,161.43">76,161</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동성장기부채</th>
												
														<td class="r" title="8,460.90">8,461</td>
												
														<td class="r" title="7,160.99">7,161</td>
												
														<td class="r" title="13,299.68">13,300</td>
												
														<td class="r cle"  title="10,557.74">10,558</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동금융부채</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매입채무및기타유동채무</th>
												
														<td class="r" title="409,777.14">409,777</td>
												
														<td class="r" title="469,431.04">469,431</td>
												
														<td class="r" title="582,603.00">582,603</td>
												
														<td class="r cle"  title="628,845.26">628,845</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동종업원급여충당부채</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타단기충당부채</th>
												
														<td class="r" title="40,686.27">40,686</td>
												
														<td class="r" title="43,495.63">43,496</td>
												
														<td class="r" title="53,728.72">53,729</td>
												
														<td class="r cle"  title="59,659.63">59,660</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;당기법인세부채</th>
												
														<td class="r" title="13,877.73">13,878</td>
												
														<td class="r" title="44,302.72">44,303</td>
												
														<td class="r" title="67,491.49">67,491</td>
												
														<td class="r cle"  title="42,806.32">42,806</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약부채</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)부채</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출부채</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타유동부채</th>
												
														<td class="r" title="21,090.92">21,091</td>
												
														<td class="r" title="22,731.42">22,731</td>
												
														<td class="r" title="27,170.51">27,171</td>
												
														<td class="r cle"  title="34,826.31">34,826</td>
												
								</tr>
		
								<tr  class="c_grid2_6 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매각예정으로분류된처분자산집단에포함된부채</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="3,387.42">3,387</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr id="p_grid2_7" class="rwf acd_dep_start_close ">
										
														<th scope="row" class="l clf">
															<div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;비유동부채</span><a id="grid2_7" href="javascript:foldOpen('grid2_7');" class=" btn_acdopen"><span class="blind" id="span_grid2_7">계산에 참여한 계정 펼치기</span></a></div>
														</th>
												
														<td class="r" title="259,013.12">259,013</td>
												
														<td class="r" title="266,833.51">266,834</td>
												
														<td class="r" title="336,040.94">336,041</td>
												
														<td class="r cle"  title="400,858.51">400,859</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;사채</th>
												
														<td class="r" title="9,752.98">9,753</td>
												
														<td class="r" title="9,481.37">9,481</td>
												
														<td class="r" title="5,082.32">5,082</td>
												
														<td class="r cle"  title="6,141.40">6,141</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기차입금</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="15.00">15</td>
												
														<td class="r cle"  title="85.00">85</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;비유동금융부채</th>
												
														<td class="r" title="21,971.81">21,972</td>
												
														<td class="r" title="19,997.16">19,997</td>
												
														<td class="r" title="28,646.56">28,647</td>
												
														<td class="r cle"  title="31,674.48">31,674</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기매입채무및기타비유동채무</th>
												
														<td class="r" title="21,842.49">21,842</td>
												
														<td class="r" title="16,829.10">16,829</td>
												
														<td class="r" title="29,914.40">29,914</td>
												
														<td class="r cle"  title="31,287.81">31,288</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;비유동종업원급여충당부채</th>
												
														<td class="r" title="4,707.80">4,708</td>
												
														<td class="r" title="4,644.58">4,645</td>
												
														<td class="r" title="4,658.84">4,659</td>
												
														<td class="r cle"  title="5,757.45">5,757</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타장기충당부채</th>
												
														<td class="r" title="6,111.00">6,111</td>
												
														<td class="r" title="10,514.28">10,514</td>
												
														<td class="r" title="23,069.94">23,070</td>
												
														<td class="r cle"  title="23,671.73">23,672</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이연법인세부채</th>
												
														<td class="r" title="170,538.08">170,538</td>
												
														<td class="r" title="188,108.45">188,108</td>
												
														<td class="r" title="231,982.05">231,982</td>
												
														<td class="r cle"  title="288,962.79">288,963</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기당기법인세부채</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약부채</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)부채</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출부채</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_7 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타비유동부채</th>
												
														<td class="r" title="24,088.96">24,089</td>
												
														<td class="r" title="17,258.57">17,259</td>
												
														<td class="r" title="12,671.83">12,672</td>
												
														<td class="r cle"  title="13,277.85">13,278</td>
												
								</tr>
		
								<tr class="rwf ">
										
														<th scope="row" class="l clf">
															<div class="">&nbsp;&nbsp;&nbsp;기타금융업부채</div>
														</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr class="rwf rowBold">
										
														<th scope="row" class="l clf">
															<div class="th_b">자본</div>
														</th>
												
														<td class="r" title="2,628,804.21">2,628,804</td>
												
														<td class="r" title="2,759,480.16">2,759,480</td>
												
														<td class="r" title="3,048,999.31">3,048,999</td>
												
														<td class="r cle"  title="3,449,068.89">3,449,069</td>
												
								</tr>
		
								<tr id="p_grid2_10" class="rwf acd_dep_start_close ">
										
														<th scope="row" class="l clf">
															<div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;지배기업주주지분</span><a id="grid2_10" href="javascript:foldOpen('grid2_10');" class=" btn_acdopen"><span class="blind" id="span_grid2_10">계산에 참여한 계정 펼치기</span></a></div>
														</th>
												
														<td class="r" title="2,549,154.72">2,549,155</td>
												
														<td class="r" title="2,676,703.31">2,676,703</td>
												
														<td class="r" title="2,962,376.97">2,962,377</td>
												
														<td class="r cle"  title="3,354,701.76">3,354,702</td>
												
								</tr>
		
								<tr  class="c_grid2_10 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자본금</th>
												
														<td class="r" title="8,975.14">8,975</td>
												
														<td class="r" title="8,975.14">8,975</td>
												
														<td class="r" title="8,975.14">8,975</td>
												
														<td class="r cle"  title="8,975.14">8,975</td>
												
								</tr>
		
								<tr  class="c_grid2_10 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;신종자본증권</th>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r" title="">&nbsp;</td>
												
														<td class="r cle"  title="">&nbsp;</td>
												
								</tr>
		
								<tr  class="c_grid2_10 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자본잉여금</th>
												
														<td class="r" title="44,038.93">44,039</td>
												
														<td class="r" title="44,038.93">44,039</td>
												
														<td class="r" title="44,038.93">44,039</td>
												
														<td class="r cle"  title="44,038.93">44,039</td>
												
								</tr>
		
								<tr  class="c_grid2_10 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타자본</th>
												
														<td class="r" title="604.29">604</td>
												
														<td class="r" title="267.26">267</td>
												
														<td class="r" title="867.63">868</td>
												
														<td class="r cle"  title="876.49">876</td>
												
								</tr>
		
								<tr  class="c_grid2_10 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타포괄손익누계액</th>
												
														<td class="r" title="-50,292.58"><span class='tcr'>-50,293</span></td>
												
														<td class="r" title="-87,260.13"><span class='tcr'>-87,260</span></td>
												
														<td class="r" title="-22,152.36"><span class='tcr'>-22,152</span></td>
												
														<td class="r cle"  title="131,908.48">131,908</td>
												
								</tr>
		
								<tr  class="c_grid2_10 rwf acd_dep2_sub" style="display:none;">
										
														<th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이익잉여금(결손금)</th>
												
														<td class="r" title="2,545,828.94">2,545,829</td>
												
														<td class="r" title="2,710,682.11">2,710,682</td>
												
														<td class="r" title="2,930,647.63">2,930,648</td>
												
														<td class="r cle"  title="3,168,902.72">3,168,903</td>
												
								</tr>
		
								<tr class="rwf ">
										
														<th scope="row" class="l clf">
															<div class="">&nbsp;&nbsp;&nbsp;비지배주주지분</div>
														</th>
												
														<td class="r" title="79,649.49">79,649</td>
												
														<td class="r" title="82,776.85">82,777</td>
												
														<td class="r" title="86,622.34">86,622</td>
												
														<td class="r cle"  title="94,367.13">94,367</td>
												
								</tr>
		

</tbody>
</table>
"#;
}
