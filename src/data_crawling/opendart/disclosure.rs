async fn get_disclosure_by_date_range(
    query_client: &reqwest::Client,
    api_key: &str,
    begin_date: &str,
    end_date: &str,
    max_records: u32,
) -> anyhow::Result<String> {
    let response = query_client.get("https://opendart.fss.or.kr/api/list.json")
				.query(&[("crtfc_key", api_key),("bgn_de", begin_date), ("end_de", end_date), ("page_no", "1"), ("page_count", &max_records.to_string())])
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.77 Safari/537.36")
				.send().await?.text().await?;

    Ok(response)
}

#[cfg(test)]
mod test {
    use {super::*, insta::assert_snapshot};

    #[tokio::test]
    async fn get_latest_at_most_hundred_disclosure_json_data() {
        let client = reqwest::Client::new();
        let api_key = std::env::var("OPENDART_API_KEY").unwrap();
        let begin_date = "20230101";
        let end_date = "20230208";
        let max_records = 100;

        let result =
            get_disclosure_by_date_range(&client, &api_key, begin_date, end_date, max_records)
                .await
                .unwrap();

        assert_snapshot!(result, @r###"{"status":"000","message":"정상","page_no":1,"page_count":100,"total_count":15202,"total_page":153,"list":[{"corp_code":"01343665","corp_name":"RF머트리얼즈","stock_code":"327260","corp_cls":"K","report_nm":"현금ㆍ현물배당결정","rcept_no":"20230208901127","flr_nm":"RF머트리얼즈","rcept_dt":"20230208","rm":"코"},{"corp_code":"01343665","corp_name":"RF머트리얼즈","stock_code":"327260","corp_cls":"K","report_nm":"현금ㆍ현물배당을위한주주명부폐쇄(기준일)결정","rcept_no":"20230208901126","flr_nm":"RF머트리얼즈","rcept_dt":"20230208","rm":"코"},{"corp_code":"01440481","corp_name":"IBKS제13호스팩","stock_code":"351340","corp_cls":"K","report_nm":"주권매매거래정지(상장폐지 사유발생)","rcept_no":"20230208901121","flr_nm":"코스닥시장본부","rcept_dt":"20230208","rm":"코"},{"corp_code":"00995993","corp_name":"DXVX","stock_code":"180400","corp_cls":"K","report_nm":"연결재무제표기준영업(잠정)실적(공정공시)","rcept_no":"20230208901079","flr_nm":"DXVX","rcept_dt":"20230208","rm":"코"},{"corp_code":"00995993","corp_name":"DXVX","stock_code":"180400","corp_cls":"K","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변동","rcept_no":"20230208901063","flr_nm":"DXVX","rcept_dt":"20230208","rm":"코"},{"corp_code":"00852087","corp_name":"시디즈","stock_code":"134790","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경","rcept_no":"20230208801116","flr_nm":"시디즈","rcept_dt":"20230208","rm":"유"},{"corp_code":"00684714","corp_name":"풍산","stock_code":"103140","corp_cls":"Y","report_nm":"연결재무제표기준영업(잠정)실적(공정공시)","rcept_no":"20230208801115","flr_nm":"풍산","rcept_dt":"20230208","rm":"유"},{"corp_code":"01089378","corp_name":"디와이디","stock_code":"219550","corp_cls":"K","report_nm":"최대주주변경을수반하는주식담보제공계약체결","rcept_no":"20230208901096","flr_nm":"디와이디","rcept_dt":"20230208","rm":"코"},{"corp_code":"01142400","corp_name":"이루다","stock_code":"164060","corp_cls":"K","report_nm":"신주인수권행사(제4회차)","rcept_no":"20230208901076","flr_nm":"이루다","rcept_dt":"20230208","rm":"코"},{"corp_code":"00155531","corp_name":"풍산홀딩스","stock_code":"005810","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경(자회사의 주요경영사항)","rcept_no":"20230208801090","flr_nm":"풍산홀딩스","rcept_dt":"20230208","rm":"유"},{"corp_code":"00426068","corp_name":"아이에스이커머스","stock_code":"069920","corp_cls":"K","report_nm":"[기재정정]최대주주변경을수반하는주식양수도계약체결","rcept_no":"20230208901053","flr_nm":"아이에스이커머스","rcept_dt":"20230208","rm":"코"},{"corp_code":"00117276","corp_name":"네이처셀","stock_code":"007390","corp_cls":"K","report_nm":"최대주주변경","rcept_no":"20230208901086","flr_nm":"네이처셀","rcept_dt":"20230208","rm":"코"},{"corp_code":"00756163","corp_name":"볼빅","stock_code":"206950","corp_cls":"N","report_nm":"신규시설투자등","rcept_no":"20230208601082","flr_nm":"볼빅","rcept_dt":"20230208","rm":"넥"},{"corp_code":"00389013","corp_name":"한국남동발전","stock_code":"","corp_cls":"E","report_nm":"일괄신고추가서류","rcept_no":"20230208000663","flr_nm":"한국남동발전","rcept_dt":"20230208","rm":""},{"corp_code":"00397191","corp_name":"팬엔터테인먼트","stock_code":"068050","corp_cls":"K","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변동","rcept_no":"20230208901059","flr_nm":"팬엔터테인먼트","rcept_dt":"20230208","rm":"코"},{"corp_code":"00946030","corp_name":"로보티즈","stock_code":"108490","corp_cls":"K","report_nm":"주식등의대량보유상황보고서(일반)","rcept_no":"20230208000662","flr_nm":"김병수","rcept_dt":"20230208","rm":""},{"corp_code":"00684714","corp_name":"풍산","stock_code":"103140","corp_cls":"Y","report_nm":"영업(잠정)실적(공정공시)","rcept_no":"20230208801080","flr_nm":"풍산","rcept_dt":"20230208","rm":"유"},{"corp_code":"00603278","corp_name":"현대커머셜","stock_code":"","corp_cls":"E","report_nm":"주요사항보고서(자본으로인정되는채무증권발행결정)","rcept_no":"20230208000660","flr_nm":"현대커머셜","rcept_dt":"20230208","rm":""},{"corp_code":"00155531","corp_name":"풍산홀딩스","stock_code":"005810","corp_cls":"Y","report_nm":"현금ㆍ현물배당결정(자회사의 주요경영사항)","rcept_no":"20230208801070","flr_nm":"풍산홀딩스","rcept_dt":"20230208","rm":"유"},{"corp_code":"00117629","corp_name":"동양철관","stock_code":"008970","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경","rcept_no":"20230208801069","flr_nm":"동양철관","rcept_dt":"20230208","rm":"유"},{"corp_code":"00243553","corp_name":"신한자산운용","stock_code":"","corp_cls":"E","report_nm":"투자설명서(집합투자증권)(신한지수연계증권투자신탁290호(온라인전용)[ELS-파생형])","rcept_no":"20230206000182","flr_nm":"신한자산운용","rcept_dt":"20230206","rm":""},{"corp_code":"00243553","corp_name":"신한자산운용","stock_code":"","corp_cls":"E","report_nm":"투자설명서(집합투자증권)(신한지수연계증권투자신탁SEK-7호(온라인전용)[ELS-파생형])","rcept_no":"20230206000188","flr_nm":"신한자산운용","rcept_dt":"20230206","rm":""},{"corp_code":"00243553","corp_name":"신한자산운용","stock_code":"","corp_cls":"E","report_nm":"투자설명서(집합투자증권)(신한지수연계증권투자신탁SEN-33호[ELS-파생형])","rcept_no":"20230206000211","flr_nm":"신한자산운용","rcept_dt":"20230206","rm":""},{"corp_code":"00684714","corp_name":"풍산","stock_code":"103140","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경","rcept_no":"20230208801060","flr_nm":"풍산","rcept_dt":"20230208","rm":"유"},{"corp_code":"00243553","corp_name":"신한자산운용","stock_code":"","corp_cls":"E","report_nm":"투자설명서(집합투자증권)(신한지수연계증권투자신탁288호[ELS-파생형])","rcept_no":"20230206000219","flr_nm":"신한자산운용","rcept_dt":"20230206","rm":""},{"corp_code":"00155531","corp_name":"풍산홀딩스","stock_code":"005810","corp_cls":"Y","report_nm":"주식소각결정","rcept_no":"20230208801039","flr_nm":"풍산홀딩스","rcept_dt":"20230208","rm":"유"},{"corp_code":"00670766","corp_name":"엠투아이","stock_code":"347890","corp_cls":"K","report_nm":"주식등의대량보유상황보고서(일반)","rcept_no":"20230208000657","flr_nm":"코메스인베스트먼트","rcept_dt":"20230208","rm":""},{"corp_code":"01042979","corp_name":"휴마시스","stock_code":"205470","corp_cls":"K","report_nm":"주주총회소집공고","rcept_no":"20230208000656","flr_nm":"휴마시스","rcept_dt":"20230208","rm":""},{"corp_code":"01042979","corp_name":"휴마시스","stock_code":"205470","corp_cls":"K","report_nm":"의결권대리행사권유참고서류","rcept_no":"20230208000654","flr_nm":"휴마시스","rcept_dt":"20230208","rm":""},{"corp_code":"00389998","corp_name":"이엔코퍼레이션","stock_code":"066980","corp_cls":"K","report_nm":"주요사항보고서(교환사채권발행결정)","rcept_no":"20230208000633","flr_nm":"이엔코퍼레이션","rcept_dt":"20230208","rm":""},{"corp_code":"00426086","corp_name":"티케이지휴켐스","stock_code":"069260","corp_cls":"Y","report_nm":"기업설명회(IR)개최(안내공시)","rcept_no":"20230208801064","flr_nm":"티케이지휴켐스","rcept_dt":"20230208","rm":"유"},{"corp_code":"01168143","corp_name":"골든센츄리","stock_code":"900280","corp_cls":"K","report_nm":"[기재정정]주요사항보고서(전환사채권발행결정)","rcept_no":"20230208000646","flr_nm":"골든센츄리","rcept_dt":"20230208","rm":""},{"corp_code":"01042979","corp_name":"휴마시스","stock_code":"205470","corp_cls":"K","report_nm":"[기재정정]주주총회소집결의","rcept_no":"20230208901046","flr_nm":"휴마시스","rcept_dt":"20230208","rm":"코"},{"corp_code":"01168143","corp_name":"골든센츄리","stock_code":"900280","corp_cls":"K","report_nm":"주주총회소집결의","rcept_no":"20230208901001","flr_nm":"골든센츄리","rcept_dt":"20230208","rm":"코"},{"corp_code":"00261443","corp_name":"엔씨소프트","stock_code":"036570","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경","rcept_no":"20230208801000","flr_nm":"엔씨소프트","rcept_dt":"20230208","rm":"유"},{"corp_code":"01472930","corp_name":"오브젠","stock_code":"417860","corp_cls":"K","report_nm":"임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230208000649","flr_nm":"하태식","rcept_dt":"20230208","rm":""},{"corp_code":"00684714","corp_name":"풍산","stock_code":"103140","corp_cls":"Y","report_nm":"영업실적등에대한전망(공정공시)","rcept_no":"20230208801022","flr_nm":"풍산","rcept_dt":"20230208","rm":"유"},{"corp_code":"00684714","corp_name":"풍산","stock_code":"103140","corp_cls":"Y","report_nm":"현금ㆍ현물배당결정","rcept_no":"20230208801014","flr_nm":"풍산","rcept_dt":"20230208","rm":"유"},{"corp_code":"00120182","corp_name":"NH투자증권","stock_code":"005940","corp_cls":"Y","report_nm":"투자설명서(일괄신고)","rcept_no":"20230208000648","flr_nm":"NH투자증권","rcept_dt":"20230208","rm":""},{"corp_code":"00670766","corp_name":"엠투아이","stock_code":"347890","corp_cls":"K","report_nm":"주식등의대량보유상황보고서(일반)","rcept_no":"20230208000647","flr_nm":"피티에이에쿼티파트너스","rcept_dt":"20230208","rm":""},{"corp_code":"01343665","corp_name":"RF머트리얼즈","stock_code":"327260","corp_cls":"K","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변동","rcept_no":"20230208900919","flr_nm":"RF머트리얼즈","rcept_dt":"20230208","rm":"코"},{"corp_code":"01168143","corp_name":"골든센츄리","stock_code":"900280","corp_cls":"K","report_nm":"[기재정정]주요사항보고서(유상증자결정)","rcept_no":"20230208000629","flr_nm":"골든센츄리","rcept_dt":"20230208","rm":""},{"corp_code":"01168143","corp_name":"골든센츄리","stock_code":"900280","corp_cls":"K","report_nm":"주주명부폐쇄기간또는기준일설정","rcept_no":"20230208901006","flr_nm":"골든센츄리","rcept_dt":"20230208","rm":"코"},{"corp_code":"00261443","corp_name":"엔씨소프트","stock_code":"036570","corp_cls":"Y","report_nm":"연결재무제표기준영업(잠정)실적(공정공시)","rcept_no":"20230208800979","flr_nm":"엔씨소프트","rcept_dt":"20230208","rm":"유"},{"corp_code":"00373021","corp_name":"테라사이언스","stock_code":"073640","corp_cls":"K","report_nm":"[첨부정정]주요사항보고서(유상증자결정)","rcept_no":"20230208000615","flr_nm":"테라사이언스","rcept_dt":"20230208","rm":""},{"corp_code":"00120182","corp_name":"NH투자증권","stock_code":"005940","corp_cls":"Y","report_nm":"일괄신고추가서류(파생결합사채-주가연계파생결합사채)","rcept_no":"20230208000645","flr_nm":"NH투자증권","rcept_dt":"20230208","rm":""},{"corp_code":"01520909","corp_name":"후레쉬미트","stock_code":"","corp_cls":"E","report_nm":"동일인등출자계열회사와의상품ㆍ용역거래변경","rcept_no":"20230208000644","flr_nm":"후레쉬미트","rcept_dt":"20230208","rm":"공"},{"corp_code":"00155531","corp_name":"풍산홀딩스","stock_code":"005810","corp_cls":"Y","report_nm":"현금ㆍ현물배당결정","rcept_no":"20230208801017","flr_nm":"풍산홀딩스","rcept_dt":"20230208","rm":"유"},{"corp_code":"01124680","corp_name":"보라티알","stock_code":"250000","corp_cls":"K","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변동","rcept_no":"20230208901028","flr_nm":"보라티알","rcept_dt":"20230208","rm":"코"},{"corp_code":"00287788","corp_name":"정상제이엘에스","stock_code":"040420","corp_cls":"K","report_nm":"현금ㆍ현물배당결정","rcept_no":"20230208901042","flr_nm":"정상제이엘에스","rcept_dt":"20230208","rm":"코"},{"corp_code":"00500254","corp_name":"GS","stock_code":"078930","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경(자회사의 주요경영사항)","rcept_no":"20230208801049","flr_nm":"GS","rcept_dt":"20230208","rm":"유"},{"corp_code":"00500254","corp_name":"GS","stock_code":"078930","corp_cls":"Y","report_nm":"현금ㆍ현물배당결정(자회사의 주요경영사항)","rcept_no":"20230208801026","flr_nm":"GS","rcept_dt":"20230208","rm":"유"},{"corp_code":"01520909","corp_name":"후레쉬미트","stock_code":"","corp_cls":"E","report_nm":"동일인등출자계열회사와의상품ㆍ용역거래변경","rcept_no":"20230208000641","flr_nm":"후레쉬미트","rcept_dt":"20230208","rm":"공"},{"corp_code":"00117744","corp_name":"메리츠화재","stock_code":"000060","corp_cls":"Y","report_nm":"임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230208000640","flr_nm":"메리츠금융지주","rcept_dt":"20230208","rm":""},{"corp_code":"01472930","corp_name":"오브젠","stock_code":"417860","corp_cls":"K","report_nm":"임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230208000639","flr_nm":"김기석","rcept_dt":"20230208","rm":""},{"corp_code":"00656021","corp_name":"시티랩스","stock_code":"139050","corp_cls":"K","report_nm":"타법인주식및출자증권취득결정","rcept_no":"20230208901027","flr_nm":"시티랩스","rcept_dt":"20230208","rm":"코"},{"corp_code":"00117179","corp_name":"디와이","stock_code":"013570","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경","rcept_no":"20230208801019","flr_nm":"디와이","rcept_dt":"20230208","rm":"유"},{"corp_code":"00670766","corp_name":"엠투아이","stock_code":"347890","corp_cls":"K","report_nm":"주식등의대량보유상황보고서(일반)","rcept_no":"20230208000637","flr_nm":"노틱인베스트먼트","rcept_dt":"20230208","rm":""},{"corp_code":"00684714","corp_name":"풍산","stock_code":"103140","corp_cls":"Y","report_nm":"주주총회소집결의","rcept_no":"20230208800963","flr_nm":"풍산","rcept_dt":"20230208","rm":"유"},{"corp_code":"00164779","corp_name":"SK하이닉스","stock_code":"000660","corp_cls":"Y","report_nm":"[발행조건확정]증권신고서(채무증권)","rcept_no":"20230208000635","flr_nm":"SK하이닉스","rcept_dt":"20230208","rm":""},{"corp_code":"01189438","corp_name":"노터스","stock_code":"278650","corp_cls":"K","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변동","rcept_no":"20230208901041","flr_nm":"노터스","rcept_dt":"20230208","rm":"코"},{"corp_code":"00852087","corp_name":"시디즈","stock_code":"134790","corp_cls":"Y","report_nm":"주주총회소집결의","rcept_no":"20230208801035","flr_nm":"시디즈","rcept_dt":"20230208","rm":"유"},{"corp_code":"00117179","corp_name":"디와이","stock_code":"013570","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경(자회사의 주요경영사항)","rcept_no":"20230208801009","flr_nm":"디와이","rcept_dt":"20230208","rm":"유"},{"corp_code":"00426086","corp_name":"티케이지휴켐스","stock_code":"069260","corp_cls":"Y","report_nm":"현금ㆍ현물배당결정","rcept_no":"20230208801040","flr_nm":"티케이지휴켐스","rcept_dt":"20230208","rm":"유"},{"corp_code":"00140177","corp_name":"GS리테일","stock_code":"007070","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경","rcept_no":"20230208801018","flr_nm":"GS리테일","rcept_dt":"20230208","rm":"유"},{"corp_code":"00117744","corp_name":"메리츠화재","stock_code":"000060","corp_cls":"Y","report_nm":"주식등의대량보유상황보고서(일반)","rcept_no":"20230208000632","flr_nm":"메리츠금융지주","rcept_dt":"20230208","rm":""},{"corp_code":"01275665","corp_name":"리메드","stock_code":"302550","corp_cls":"K","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변동","rcept_no":"20230208901034","flr_nm":"리메드","rcept_dt":"20230208","rm":"코"},{"corp_code":"00296290","corp_name":"키움증권","stock_code":"039490","corp_cls":"Y","report_nm":"[발행조건확정]증권신고서(채무증권)","rcept_no":"20230208000630","flr_nm":"키움증권","rcept_dt":"20230208","rm":""},{"corp_code":"00120562","corp_name":"롯데지주","stock_code":"004990","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경(자회사의 주요경영사항)","rcept_no":"20230208801008","flr_nm":"롯데지주","rcept_dt":"20230208","rm":"유"},{"corp_code":"00120562","corp_name":"롯데지주","stock_code":"004990","corp_cls":"Y","report_nm":"현금ㆍ현물배당결정(자회사의 주요경영사항)","rcept_no":"20230208800956","flr_nm":"롯데지주","rcept_dt":"20230208","rm":"유"},{"corp_code":"00356839","corp_name":"메디콕스","stock_code":"054180","corp_cls":"K","report_nm":"[기재정정]주요사항보고서(유상증자결정)","rcept_no":"20230208000609","flr_nm":"메디콕스","rcept_dt":"20230208","rm":""},{"corp_code":"00141149","corp_name":"영진약품","stock_code":"003520","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경","rcept_no":"20230208800968","flr_nm":"영진약품","rcept_dt":"20230208","rm":"유"},{"corp_code":"01335790","corp_name":"바이젠셀","stock_code":"308080","corp_cls":"K","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변동","rcept_no":"20230208900983","flr_nm":"바이젠셀","rcept_dt":"20230208","rm":"코"},{"corp_code":"00268118","corp_name":"코맥스","stock_code":"036690","corp_cls":"K","report_nm":"현금ㆍ현물배당결정","rcept_no":"20230208901011","flr_nm":"코맥스","rcept_dt":"20230208","rm":"코"},{"corp_code":"00117179","corp_name":"디와이","stock_code":"013570","corp_cls":"Y","report_nm":"연결재무제표기준영업실적등에대한전망(공정공시)","rcept_no":"20230208801038","flr_nm":"디와이","rcept_dt":"20230208","rm":"유"},{"corp_code":"00694942","corp_name":"이미지스","stock_code":"115610","corp_cls":"K","report_nm":"주식등의대량보유상황보고서(일반)","rcept_no":"20230208000627","flr_nm":"김정철","rcept_dt":"20230208","rm":""},{"corp_code":"00113058","corp_name":"한화생명","stock_code":"088350","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경","rcept_no":"20230208800986","flr_nm":"한화생명","rcept_dt":"20230208","rm":"유"},{"corp_code":"00117744","corp_name":"메리츠화재","stock_code":"000060","corp_cls":"Y","report_nm":"임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230208000626","flr_nm":"김정진","rcept_dt":"20230208","rm":""},{"corp_code":"00155531","corp_name":"풍산홀딩스","stock_code":"005810","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경","rcept_no":"20230208801007","flr_nm":"풍산홀딩스","rcept_dt":"20230208","rm":"유"},{"corp_code":"00117744","corp_name":"메리츠화재","stock_code":"000060","corp_cls":"Y","report_nm":"임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230208000624","flr_nm":"박호경","rcept_dt":"20230208","rm":""},{"corp_code":"00164779","corp_name":"SK하이닉스","stock_code":"000660","corp_cls":"Y","report_nm":"[기재정정]증권신고서(채무증권)","rcept_no":"20230208000623","flr_nm":"SK하이닉스","rcept_dt":"20230208","rm":"정"},{"corp_code":"00117179","corp_name":"디와이","stock_code":"013570","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경(자회사의 주요경영사항)","rcept_no":"20230208800964","flr_nm":"디와이","rcept_dt":"20230208","rm":"유"},{"corp_code":"00117179","corp_name":"디와이","stock_code":"013570","corp_cls":"Y","report_nm":"현금ㆍ현물배당결정","rcept_no":"20230208801030","flr_nm":"디와이","rcept_dt":"20230208","rm":"유"},{"corp_code":"00117744","corp_name":"메리츠화재","stock_code":"000060","corp_cls":"Y","report_nm":"임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230208000622","flr_nm":"정학수","rcept_dt":"20230208","rm":""},{"corp_code":"00933504","corp_name":"메리츠캐피탈","stock_code":"","corp_cls":"E","report_nm":"투자설명서(일괄신고)","rcept_no":"20230208000621","flr_nm":"메리츠캐피탈","rcept_dt":"20230208","rm":""},{"corp_code":"00140177","corp_name":"GS리테일","stock_code":"007070","corp_cls":"Y","report_nm":"현금ㆍ현물배당결정","rcept_no":"20230208801025","flr_nm":"GS리테일","rcept_dt":"20230208","rm":"유"},{"corp_code":"00957568","corp_name":"에스엘바이오닉스","stock_code":"214310","corp_cls":"K","report_nm":"기타시장안내(개선계획서 제출)","rcept_no":"20230208900988","flr_nm":"코스닥시장본부","rcept_dt":"20230208","rm":"코"},{"corp_code":"00155531","corp_name":"풍산홀딩스","stock_code":"005810","corp_cls":"Y","report_nm":"주주총회소집결의","rcept_no":"20230208800989","flr_nm":"풍산홀딩스","rcept_dt":"20230208","rm":"유"},{"corp_code":"00117744","corp_name":"메리츠화재","stock_code":"000060","corp_cls":"Y","report_nm":"임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230208000620","flr_nm":"은상영","rcept_dt":"20230208","rm":""},{"corp_code":"00117179","corp_name":"디와이","stock_code":"013570","corp_cls":"Y","report_nm":"현금ㆍ현물배당결정(자회사의 주요경영사항)","rcept_no":"20230208801023","flr_nm":"디와이","rcept_dt":"20230208","rm":"유"},{"corp_code":"00146427","corp_name":"일야","stock_code":"058450","corp_cls":"K","report_nm":"[기재정정]주요사항보고서(전환사채권발행결정)","rcept_no":"20230208000574","flr_nm":"일야","rcept_dt":"20230208","rm":""},{"corp_code":"00165413","corp_name":"롯데케미칼","stock_code":"011170","corp_cls":"Y","report_nm":"신탁계약해지결과보고서","rcept_no":"20230208000619","flr_nm":"롯데케미칼","rcept_dt":"20230208","rm":""},{"corp_code":"00656021","corp_name":"시티랩스","stock_code":"139050","corp_cls":"K","report_nm":"주권관련사채권의취득결정","rcept_no":"20230208900925","flr_nm":"시티랩스","rcept_dt":"20230208","rm":"코"},{"corp_code":"00117744","corp_name":"메리츠화재","stock_code":"000060","corp_cls":"Y","report_nm":"임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230208000617","flr_nm":"이국진","rcept_dt":"20230208","rm":""},{"corp_code":"01472930","corp_name":"오브젠","stock_code":"417860","corp_cls":"K","report_nm":"[기재정정]임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230208000616","flr_nm":"차준호","rcept_dt":"20230208","rm":""},{"corp_code":"00101628","corp_name":"경방","stock_code":"000050","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경","rcept_no":"20230208800971","flr_nm":"경방","rcept_dt":"20230208","rm":"유"},{"corp_code":"00117744","corp_name":"메리츠화재","stock_code":"000060","corp_cls":"Y","report_nm":"임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230208000614","flr_nm":"김중현","rcept_dt":"20230208","rm":""},{"corp_code":"00117744","corp_name":"메리츠화재","stock_code":"000060","corp_cls":"Y","report_nm":"임원ㆍ주요주주특정증권등소유상황보고서","rcept_no":"20230208000613","flr_nm":"장진우","rcept_dt":"20230208","rm":""},{"corp_code":"00120526","corp_name":"롯데쇼핑","stock_code":"023530","corp_cls":"Y","report_nm":"매출액또는손익구조30%(대규모법인은15%)이상변경","rcept_no":"20230208800942","flr_nm":"롯데쇼핑","rcept_dt":"20230208","rm":"유"},{"corp_code":"00261443","corp_name":"엔씨소프트","stock_code":"036570","corp_cls":"Y","report_nm":"주주총회소집결의","rcept_no":"20230208800934","flr_nm":"엔씨소프트","rcept_dt":"20230208","rm":"유"}]}"###)
    }
}
