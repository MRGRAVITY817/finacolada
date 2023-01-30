use scraper::{Html, Selector};

pub async fn get_financial_statement(
    query_client: &reqwest::Client,
    ticker: &str,
) -> anyhow::Result<Vec<String>> {
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
    let selector = Selector::parse("table").unwrap();
    let tables = document
        .select(&selector)
        .map(|selected| {
            let table_html = selected.html();
            match table_extract::Table::find_first(&table_html) {
                Some(table) => table
                    .into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(ToOwned::to_owned)
                            .reduce(|acc, val| format!("{acc} | {val}"))
                            .unwrap_or("".to_string())
                    })
                    .reduce(|rows_above, row| format!("{rows_above} \n {row}"))
                    .unwrap_or("".to_string()),
                _ => "".to_string(),
            }
        })
        .collect::<Vec<_>>();

    // Table 1. Comprehensive income statement (annual)
    // Table 2. Comprehensive income statement (quarterly)
    // Table 3. Financial position statement (annual)
    // Table 4. Financial position statement (quarterly)
    // Table 5. Cashflow statement (annual)
    // Table 6. Cashflow statement (quarterly)
    Ok(tables)
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
        assert_snapshot!(result[0], @r###"
        2,304,009 | 2,368,070 | 2,796,048 | 2,317,668 | 2,030,393 | 14.1 
         1,472,395 | 1,444,883 | 1,664,113 | 1,414,140 | 1,214,648 | 16.4 
         831,613 | 923,187 | 1,131,935 | 903,527 | 815,745 | 10.8 
         553,928 | 563,248 | 615,596 | 512,822 | 438,073 | 17.1 
         64,226 | 70,429 | 75,568 | 60,863 | 54,634 | 11.4 
         20,408 | 20,857 | 20,731 | 16,627 | 15,505 | 7.2 
         199,072 | 211,115 | 224,017 | 184,467 | 159,330 | 15.8 
         46,145 | 42,690 | 53,760 | 46,446 | 37,671 | 23.3 
         117,222 | 114,488 | 131,185 | 109,634 | 92,626 | 18.4 
         55,131 | 56,787 | 61,926 | 51,124 | 43,751 | 16.9 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         51,724 | 46,883 | 48,409 | 43,662 | 34,558 | 26.3 
         277,685 | 359,939 | 516,339 | 390,705 | 377,671 | 3.5 
         277,685 | 359,939 | 516,339 | 390,705 | 377,671 | 3.5 
         101,616 | 122,676 | 85,432 | 156,330 | 66,867 | 133.8 
         26,600 | 19,745 | 12,783 | 16,557 | 9,240 | 79.2 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         67,690 | 92,700 | 65,257 | 126,563 | 52,577 | 140.7 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         7,326 | 10,231 | 7,392 | 13,211 | 5,050 | 161.6 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         82,749 | 113,181 | 77,046 | 142,659 | 60,072 | 137.5 
         6,864 | 5,830 | 4,315 | 4,888 | 3,077 | 58.9 
         68,524 | 98,686 | 64,861 | 126,106 | 51,055 | 147.0 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         7,361 | 8,665 | 7,869 | 11,665 | 5,941 | 96.3 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         17,787 | 13,841 | 22,057 | 14,803 | 14,656 | 1.0 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         1,547 | 1,524 | 1,358 | 3,735 | 987 | 278.3 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         3,524 | 1,542 | 3,404 | 1,359 | 3,196 | <span class="tcr">-57.5</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         1,534 | 1,471 | 1,328 | 1,050 | 994 | 5.6 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         11,182 | 9,303 | 15,967 | 8,659 | 9,478 | <span class="tcr">-8.6</span> 
         14,147 | 24,889 | 20,560 | 13,370 | 15,239 | <span class="tcr">-12.3</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         1,445 | 877 | 756 | 348 | 696 | <span class="tcr">-50.0</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         12,702 | 24,012 | 19,804 | 13,022 | 14,543 | <span class="tcr">-10.5</span> 
         4,130 | 5,065 | 7,296 | 8,046 | 6,008 | 33.9 
         4,130 | 5,065 | 7,296 | 8,046 | 6,008 | 33.9 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         304,322 | 363,451 | 533,518 | 413,856 | 389,889 | 6.1 
         86,933 | 99,373 | 134,444 | 95,729 | 99,194 | <span class="tcr">-3.5</span> 
         217,389 | 264,078 | 399,075 | 318,126 | 290,695 | 9.4 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         217,389 | 264,078 | 399,075 | 318,126 | 290,695 | 9.4 
         215,051 | 260,908 | 392,438 | 312,275 | 286,007 | 9.2 
         2,338 | 3,170 | 6,637 | 5,851 | 4,688 | 24.8
        "###);
        assert_snapshot!(result[1], @r###"
        765,655 | 777,815 | 772,036 | 767,817 | 739,792 | 3.8 
         449,466 | 470,721 | 462,697 | 480,722 | 428,989 | 12.1 
         316,190 | 307,094 | 309,339 | 287,094 | 310,803 | <span class="tcr">-7.6</span> 
         177,523 | 165,880 | 168,368 | 178,574 | 152,628 | 17.0 
         20,934 | 20,726 | 19,572 | 20,564 | 18,634 | 10.4 
         5,226 | 5,472 | 5,459 | 5,696 | 5,194 | 9.7 
         64,688 | 59,222 | 62,549 | 62,696 | 51,099 | 22.7 
         16,090 | 14,677 | 13,975 | 17,794 | 15,377 | 15.7 
         38,559 | 35,746 | 33,581 | 40,307 | 35,628 | 13.1 
         18,175 | 17,304 | 18,069 | 15,751 | 14,455 | 9.0 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         13,851 | 12,733 | 15,164 | 15,766 | 12,241 | 28.8 
         138,667 | 141,214 | 140,970 | 108,520 | 158,175 | <span class="tcr">-31.4</span> 
         138,667 | 141,214 | 140,970 | 108,520 | 158,175 | <span class="tcr">-31.4</span> 
         18,565 | 35,022 | 53,719 | 67,590 | 26,609 | 154.0 
         3,543 | 3,690 | 4,872 | 7,994 | 3,373 | 137.0 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         12,680 | 26,086 | 44,908 | 55,569 | 21,407 | 159.6 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         2,343 | 5,246 | 3,939 | 4,026 | 1,829 | 120.1 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         16,973 | 30,333 | 51,310 | 61,016 | 24,906 | 145.0 
         1,239 | 1,492 | 1,231 | 2,165 | 1,370 | 58.0 
         13,806 | 24,607 | 46,756 | 54,743 | 21,844 | 150.6 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         1,928 | 4,234 | 3,323 | 4,107 | 1,693 | 142.7 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         7,401 | 7,002 | 4,640 | 3,162 | 4,252 | <span class="tcr">-25.6</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         371 | 2,685 | 624 | 426 | 217 | 96.7 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         208 | 504 | 638 | 217 | 1,136 | <span class="tcr">-80.9</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         334 | 346 | 349 | 355 | 326 | 8.9 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         6,489 | 3,467 | 3,029 | 2,163 | 2,573 | <span class="tcr">-15.9</span> 
         5,320 | 4,531 | 6,004 | 2,835 | 3,228 | <span class="tcr">-12.2</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         60 | 42 | 199 | 107 | 191 | <span class="tcr">-44.1</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         5,261 | 4,489 | 5,805 | 2,728 | 3,037 | <span class="tcr">-10.2</span> 
         1,289 | 2,325 | 2,592 | 3,128 | 2,657 | 17.7 
         1,289 | 2,325 | 2,592 | 3,128 | 2,657 | 17.7 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         143,629 | 150,698 | 144,608 | 118,550 | 163,559 | <span class="tcr">-27.5</span> 
         35,249 | 37,452 | 33,620 | 24,658 | 40,625 | <span class="tcr">-39.3</span> 
         108,379 | 113,246 | 110,988 | 93,892 | 122,933 | <span class="tcr">-23.6</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         108,379 | 113,246 | 110,988 | 93,892 | 122,933 | <span class="tcr">-23.6</span> 
         106,431 | 111,291 | 109,545 | 91,439 | 120,572 | <span class="tcr">-24.2</span> 
         1,948 | 1,955 | 1,443 | 2,453 | 2,361 | 3.9
        "###);
        assert_snapshot!(result[2], @r###"
        3,525,645 | 3,782,357 | 4,266,212 | 4,702,784 
         1,813,853 | 1,982,156 | 2,181,632 | 2,508,806 
         267,665 | 320,431 | 413,844 | 573,198 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         818,937 | 952,703 | 851,188 | 843,007 
         393,105 | 345,696 | 452,107 | 533,932 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         65,286 | 60,206 | 74,179 | 113,514 
         268,860 | 293,826 | 390,314 | 445,154 
         &nbsp; | 9,294 | &nbsp; | &nbsp; 
         1,711,792 | 1,800,201 | 2,084,580 | 2,193,978 
         1,198,255 | 1,289,529 | 1,499,285 | 1,603,436 
         207,035 | 184,685 | 202,362 | 214,849 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         99,697 | 137,782 | 154,912 | 127,871 
         75,916 | 80,768 | 89,323 | 108,528 
         7,576 | 4,870 | 12,279 | &nbsp; 
         45,050 | 42,750 | 42,612 | 54,882 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         78,263 | 59,818 | 83,807 | 84,413 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         896,841 | 1,022,877 | 1,217,212 | 1,253,715 
         637,828 | 756,044 | 881,171 | 852,857 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         143,935 | 165,534 | 136,878 | 76,161 
         8,461 | 7,161 | 13,300 | 10,558 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         409,777 | 469,431 | 582,603 | 628,845 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         40,686 | 43,496 | 53,729 | 59,660 
         13,878 | 44,303 | 67,491 | 42,806 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         21,091 | 22,731 | 27,171 | 34,826 
         &nbsp; | 3,387 | &nbsp; | &nbsp; 
         259,013 | 266,834 | 336,041 | 400,859 
         9,753 | 9,481 | 5,082 | 6,141 
         &nbsp; | &nbsp; | 15 | 85 
         21,972 | 19,997 | 28,647 | 31,674 
         21,842 | 16,829 | 29,914 | 31,288 
         4,708 | 4,645 | 4,659 | 5,757 
         6,111 | 10,514 | 23,070 | 23,672 
         170,538 | 188,108 | 231,982 | 288,963 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         24,089 | 17,259 | 12,672 | 13,278 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         2,628,804 | 2,759,480 | 3,048,999 | 3,449,069 
         2,549,155 | 2,676,703 | 2,962,377 | 3,354,702 
         8,975 | 8,975 | 8,975 | 8,975 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         44,039 | 44,039 | 44,039 | 44,039 
         604 | 267 | 868 | 876 
         <span class="tcr">-50,293</span> | <span class="tcr">-87,260</span> | <span class="tcr">-22,152</span> | 131,908 
         2,545,829 | 2,710,682 | 2,930,648 | 3,168,903 
         79,649 | 82,777 | 86,622 | 94,367
        "###);
        assert_snapshot!(result[3], @r###"
        4,266,212 | 4,393,270 | 4,480,407 | 4,702,784 
         2,181,632 | 2,323,691 | 2,362,875 | 2,508,806 
         413,844 | 475,907 | 520,922 | 573,198 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         851,188 | 769,297 | 857,375 | 843,007 
         452,107 | 501,174 | 497,167 | 533,932 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         74,179 | 87,863 | 91,579 | 113,514 
         390,314 | 489,450 | 395,831 | 445,154 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         2,084,580 | 2,069,579 | 2,117,532 | 2,193,978 
         1,499,285 | 1,496,180 | 1,542,546 | 1,603,436 
         202,362 | 200,107 | 200,969 | 214,849 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         154,912 | 146,977 | 129,928 | 127,871 
         89,323 | 88,416 | 103,829 | 108,528 
         12,279 | &nbsp; | &nbsp; | &nbsp; 
         42,612 | 41,790 | 51,143 | 54,882 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         83,807 | 96,109 | 89,116 | 84,413 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         1,217,212 | 1,240,360 | 1,201,340 | 1,253,715 
         881,171 | 904,637 | 833,623 | 852,857 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         136,878 | 138,859 | 129,892 | 76,161 
         13,300 | 8,649 | 9,528 | 10,558 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         582,603 | 594,700 | 542,259 | 628,845 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         53,729 | 59,108 | 59,958 | 59,660 
         67,491 | 71,938 | 60,677 | 42,806 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         27,171 | 31,383 | 31,310 | 34,826 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         336,041 | 335,723 | 367,717 | 400,859 
         5,082 | 5,189 | 5,538 | 6,141 
         15 | 25 | 85 | 85 
         28,647 | 27,782 | 29,347 | 31,674 
         29,914 | 25,666 | 28,720 | 31,288 
         4,659 | 4,769 | 5,171 | 5,757 
         23,070 | 22,496 | 22,782 | 23,672 
         231,982 | 237,054 | 263,414 | 288,963 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         12,672 | 12,743 | 12,660 | 13,278 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         3,048,999 | 3,152,909 | 3,279,067 | 3,449,069 
         2,962,377 | 3,063,919 | 3,188,306 | 3,354,702 
         8,975 | 8,975 | 8,975 | 8,975 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         44,039 | 44,039 | 44,039 | 44,039 
         868 | 931 | 947 | 876 
         <span class="tcr">-22,152</span> | <span class="tcr">-7,458</span> | 32,177 | 131,908 
         2,930,648 | 3,017,432 | 3,102,168 | 3,168,903 
         86,622 | 88,990 | 90,761 | 94,367
        "###);
        assert_snapshot!(result[4], @r###"
        453,829 | 652,870 | 651,054 | 435,684 
         217,389 | 264,078 | 399,075 | 318,126 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         424,268 | 461,506 | 527,991 | 486,690 
         11,716 | 12,902 | 13,603 | 10,402 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | 400 | 180 | 398 
         265,738 | 271,157 | 312,852 | 271,090 
         30,238 | 32,199 | 29,622 | 23,496 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         20,345 | 23,068 | 19,177 | 50,116 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         1,445 | 877 | 756 | 348 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         7,852 | 10,008 | 17,357 | 33,361 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         86,933 | 99,373 | 134,444 | 95,729 
         &nbsp; | 11,523 | &nbsp; | 1,750 
         49,842 | 45,321 | 37,435 | 70,341 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         1,834 | &nbsp; | &nbsp; | &nbsp; 
         38,314 | 37,188 | 24,857 | 57,201 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         1,547 | 1,524 | 1,358 | 3,735 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         3,524 | 1,542 | 3,404 | 1,359 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         4,130 | 5,065 | 7,296 | 8,046 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         493 | &nbsp; | 519 | &nbsp; 
         <span class="tcr">-25,458</span> | 1,224 | <span class="tcr">-162,869</span> | <span class="tcr">-218,391</span> 
         38,981 | <span class="tcr">-58,161</span> | <span class="tcr">-174,243</span> | <span class="tcr">-139,856</span> 
         <span class="tcr">-57,967</span> | 76,164 | 45,248 | <span class="tcr">-66,470</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-6,472</span> | <span class="tcr">-16,779</span> | <span class="tcr">-33,874</span> | <span class="tcr">-12,064</span> 
         566,358 | 681,488 | 726,762 | 516,085 
         <span class="tcr">-112,529</span> | <span class="tcr">-28,618</span> | <span class="tcr">-75,708</span> | <span class="tcr">-80,401</span> 
         23,064 | 22,202 | 14,067 | 12,295 
         <span class="tcr">-5,800</span> | <span class="tcr">-5,553</span> | <span class="tcr">-4,344</span> | <span class="tcr">-4,810</span> 
         2,418 | 2,437 | 2,990 | 4,900 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-132,211</span> | <span class="tcr">-47,704</span> | <span class="tcr">-88,420</span> | <span class="tcr">-92,785</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-399,482</span> | <span class="tcr">-536,286</span> | <span class="tcr">-330,478</span> | <span class="tcr">-279,528</span> 
         63,008 | 155,517 | 254,825 | 134,745 
         11,355 | 29,836 | 142,179 | 63,509 
         45,866 | 121,843 | 102,161 | 68,964 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         121 | &nbsp; | 192 | 132 
         5,133 | 3,767 | 3,583 | 1,905 
         72 | 70 | 18 | 235 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         460 | &nbsp; | 6,693 | &nbsp; 
         462,489 | 691,803 | 585,303 | 414,273 
         38,736 | 206,993 | 16,664 | 1,557 
         127,255 | 80,193 | 69,818 | 43,940 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         128 | 833 | 471 | 9,038 
         253,678 | 375,920 | 471,221 | 324,713 
         32,499 | 26,798 | 27,069 | 31,895 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         10,194 | 1,066 | 59 | 3,131 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-94,845</span> | <span class="tcr">-83,278</span> | <span class="tcr">-239,910</span> | <span class="tcr">-146,848</span> 
         8,658 | 22,139 | 583 | 1,435 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         8,658 | 22,057 | 583 | 1,435 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | 82 | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         7,111 | 8,649 | 35,390 | 74,689 
         7,094 | 8,649 | 8,947 | &nbsp; 
         &nbsp; | &nbsp; | 26,169 | 74,681 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         17 | &nbsp; | 273 | 8 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-96,392</span> | <span class="tcr">-96,768</span> | <span class="tcr">-205,104</span> | <span class="tcr">-73,593</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-96,392</span> | <span class="tcr">-96,768</span> | <span class="tcr">-205,104</span> | <span class="tcr">-73,593</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | <span class="tcr">-1</span> | 1 | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         5,953 | <span class="tcr">-8,339</span> | 15,820 | 45,531 
         <span class="tcr">-34,545</span> | 24,966 | 96,488 | 54,840 
         303,405 | 268,860 | 293,826 | 390,314 
         268,860 | 293,826 | 390,314 | 445,154
        "###);
        assert_snapshot!(result[5], @r###"
        206,345 | 104,531 | 141,361 | 189,793 
         108,379 | 113,246 | 110,988 | 93,892 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         140,618 | 162,030 | 158,247 | 166,413 
         3,512 | 3,509 | 3,415 | 3,478 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         131 | 114 | 130 | 153 
         88,646 | 90,090 | 90,688 | 90,311 
         7,373 | 7,705 | 7,726 | 8,066 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         932 | 14,629 | 10,533 | 24,955 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         60 | 42 | 199 | 107 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         4,715 | 7,720 | 11,625 | 14,016 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         35,249 | 37,452 | 33,620 | 24,658 
         &nbsp; | 768 | 312 | 669 
         3,107 | 22,972 | 16,727 | 30,642 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         2,770 | 17,458 | 12,873 | 26,870 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         371 | 2,685 | 624 | 426 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         208 | 504 | 638 | 217 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         1,289 | 2,325 | 2,592 | 3,128 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-1,531</span> | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-25,767</span> | <span class="tcr">-142,333</span> | <span class="tcr">-51,939</span> | <span class="tcr">-24,119</span> 
         <span class="tcr">-22,379</span> | <span class="tcr">-98,648</span> | <span class="tcr">-12,056</span> | <span class="tcr">-29,153</span> 
         15,897 | <span class="tcr">-35,659</span> | <span class="tcr">-42,149</span> | 11,338 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-19,285</span> | <span class="tcr">-8,026</span> | 2,266 | <span class="tcr">-6,304</span> 
         220,124 | 109,970 | 200,570 | 205,545 
         <span class="tcr">-13,779</span> | <span class="tcr">-5,440</span> | <span class="tcr">-59,209</span> | <span class="tcr">-15,752</span> 
         3,676 | 3,340 | 3,727 | 5,228 
         <span class="tcr">-1,529</span> | <span class="tcr">-1,591</span> | <span class="tcr">-1,058</span> | <span class="tcr">-2,162</span> 
         382 | 501 | 1,797 | 2,602 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-16,308</span> | <span class="tcr">-7,690</span> | <span class="tcr">-63,676</span> | <span class="tcr">-21,419</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-103,513</span> | <span class="tcr">-6,365</span> | <span class="tcr">-192,928</span> | <span class="tcr">-80,236</span> 
         42,353 | 119,965 | <span class="tcr">-36,806</span> | 51,586 
         21,365 | 78,865 | <span class="tcr">-50,463</span> | 35,107 
         20,729 | 40,359 | 13,081 | 15,525 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         183 | 53 | 16 | 63 
         <span class="tcr">-5</span> | 687 | 371 | 847 
         &nbsp; | 1 | 189 | 44 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         81 | &nbsp; | &nbsp; | &nbsp; 
         145,866 | 126,330 | 156,122 | 131,821 
         6,320 | 728 | 9,410 | <span class="tcr">-8,581</span> 
         11,040 | 28,434 | 14,835 | 671 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         136 | 61 | 8,977 | &nbsp; 
         123,726 | 87,068 | 114,509 | 123,136 
         4,929 | 8,534 | 7,229 | 16,131 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-284</span> | 1,504 | 1,162 | 465 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-38,003</span> | <span class="tcr">-4,953</span> | <span class="tcr">-59,623</span> | <span class="tcr">-82,271</span> 
         240 | 2,140 | <span class="tcr">-1,408</span> | 702 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         240 | 2,140 | <span class="tcr">-1,408</span> | 702 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         13,670 | 7,089 | 9,154 | 58,446 
         8,947 | &nbsp; | &nbsp; | &nbsp; 
         4,606 | 7,087 | 9,153 | 58,441 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         116 | 2 | 1 | 5 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-24,573</span> | <span class="tcr">-5</span> | <span class="tcr">-49,061</span> | <span class="tcr">-24,528</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-24,573</span> | <span class="tcr">-5</span> | <span class="tcr">-49,061</span> | <span class="tcr">-24,528</span> 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         &nbsp; | &nbsp; | &nbsp; | &nbsp; 
         <span class="tcr">-1,264</span> | 5,924 | 17,571 | 22,036 
         63,564 | 99,136 | <span class="tcr">-93,618</span> | 49,323 
         326,750 | 390,314 | 489,450 | 395,831 
         390,314 | 489,450 | 395,831 | 445,154
        "###);
    }
}
