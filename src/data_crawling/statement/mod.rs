pub async fn get_financial_statement(
    query_client: &reqwest::Client,
    ticker: &str,
) -> anyhow::Result<String> {
    let result = query_client
        .get("http://comp.fnguide.com/SVO2/ASP/SVD_Finance.asp")
        .query(&[("pGB", "1"), ("gicode", &format!("A{ticker}"))])
				.header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.77 Safari/537.36")
        .send()
        .await?
        .text()
        .await?;

    std::fs::write("assets/statement.html", &result)?;
    // TODO: Parse HTML table!

    Ok(result)
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
        assert_snapshot!(result, @r###"

        <!DOCTYPE html>
        <html lang="ko">
        <head>
            
        <link rel="stylesheet" href="http://cdn.fnguide.com/SVO2/css/compeach.css?v=1.0.3">
        <!--[if lte IE 9]><link rel="stylesheet" href="http://cdn.fnguide.com/SVO2/css/ie9.css?ver3"><![endif]-->
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta charset="utf-8">
        <meta content="width=device-width" name="viewport">
        <!-- <meta content="width=device-width,initial-scale=1.0,minimum-scale=1.0,maximum-scale=1.0" name="viewport"> -->
        <script type="text/javascript" src="http://cdn.fnguide.com/SVO2/js/lib/jquery-1.11.1.min.js?v=1.0.6"></script>
        <script type="text/javascript" src="http://cdn.fnguide.com/SVO2/js/comp_load.js?v=1.0.6"></script>
        <script type="text/javascript" src="http://cdn.fnguide.com/SVO2/js/lib/jquery.autocomplete.js?v=1.0.6"></script>
        <link rel="stylesheet" type="text/css" href="http://cdn.fnguide.com/SVO2/css/jquery.autocomplete.css?v=1.0.3" />
        <LINK REL="SHORTCUT ICON" HREF="http://cdn.fnguide.com/SVO2/img/CompanyGuide.ico">
        <script type="text/javascript" src="http://cdn.fnguide.com/SVO2/js/autocomplete.js?v=1.0.6"></script>

            <meta name="author" content="">
            <meta name="description" content="">
        	<title>삼성전자(A005930) | 재무제표 | 기업정보 | Company Guide</title>
            <script>
              ///SVO2/xml/Finance_C_IFRS/005930.xml
                $(function(){
                    if ($('#stkGb').val() !='751' && $('#gv_stkGb').val() !='754'){
                        //$('#divData').show();
                        //탭클릭이벤트 
        				if ($('#ReportGB').val()=="B" || $('#ReportGB').val()=="A" ) {
                            $('div#divReportGb > a').eq(1).addClass('ac');  
                            $('div#divReportGb > a').eq(0).removeClass('ac');
        					$('#ReportGB').val("B");
                        }else{
                            $('div#divReportGb > a').eq(1).removeClass('ac');   
                            $('div#divReportGb > a').eq(0).addClass('ac'); 
        					$('#ReportGB').val("D");
                        }
        				
        				$('#sonkiYQ').val($.cookie('finance_cook_sonkiYQ'));
        				$('#daechaYQ').val($.cookie('finance_cook_daechaYQ'));
        				$('#cashYQ').val($.cookie('finance_cook_cashYQ'));
        				
        				if ($('#sonkiYQ').val() =="Q"){	
        					$('#btnSonik_Q').addClass("ac");
        					$('#btnSonikYQText').html("[분기]");
        					$("#sonikChart1").attr('src','http://cdn.fnguide.com/SVO2/chartImg/03_01/A005930_'+$('#ReportGB').val()+'_Q_03_01.png');
        					$("#sonikChart2").attr('src','http://cdn.fnguide.com/SVO2/chartImg/03_02/A005930_'+$('#ReportGB').val()+'_Q_03_02.png');
        					$("#divSonikQ").show();

        				}else{
        					$('#btnSonik_Y').addClass("ac");
        					$('#btnSonikYQText').html("[연간]");
        					$("#sonikChart1").attr('src','http://cdn.fnguide.com/SVO2/chartImg/03_01/A005930_'+$('#ReportGB').val()+'_D_03_01.png');
        					$("#sonikChart2").attr('src','http://cdn.fnguide.com/SVO2/chartImg/03_02/A005930_'+$('#ReportGB').val()+'_D_03_02.png');
        					$("#divSonikY").show();
        				}

        				if ($('#daechaYQ').val() =="Q"){	
        					$('#btnDaecha_Q').addClass("ac")
        					$('#btnDaechaYQText').html("[분기]");
        					$("#deachaChart1").attr('src','http://cdn.fnguide.com/SVO2/chartImg/03_03/A005930_'+$('#ReportGB').val()+'_Q_03_03.png');
        					$("#deachaChart2").attr('src','http://cdn.fnguide.com/SVO2/chartImg/03_04/A005930_'+$('#ReportGB').val()+'_Q_03_04.png');
        					$("#divDaechaQ").show();
        				}else{
        					$('#btnDaecha_Y').addClass("ac")
        					$('#btnDaechaYQText').html("[연간]");
        					$("#deachaChart1").attr('src','http://cdn.fnguide.com/SVO2/chartImg/03_03/A005930_'+$('#ReportGB').val()+'_D_03_03.png');
        					$("#deachaChart2").attr('src','http://cdn.fnguide.com/SVO2/chartImg/03_04/A005930_'+$('#ReportGB').val()+'_D_03_04.png');
        					$("#divDaechaY").show();
        				}

        				if ($('#cashYQ').val() =="Q"){
        					$('#btnCash_Q').addClass("ac");
        					$('#btnCashYQText').html("[분기]");
        					$('#cashChart1').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_05/A005930_'+$('#ReportGB').val()+'_Q_03_05.png');
        					$('#cashChart2').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_06/A005930_'+$('#ReportGB').val()+'_Q_03_06.png');
        					$("#divCashQ").show();
        				}else{
        					$('#btnCash_Y').addClass("ac");
        					$('#btnCashYQText').html("[연간]");
        					$('#cashChart1').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_05/A005930_'+$('#ReportGB').val()+'_D_03_05.png');
        					$('#cashChart2').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_06/A005930_'+$('#ReportGB').val()+'_D_03_06.png');
        					$("#divCashY").show();
        				}

                        /*if ($('#ReportGB').val()=="B" || $('#ReportGB').val()=="A" ) {
                            $('div#divReportGb > a').eq(1).addClass('ac');  
                            $('div#divReportGb > a').eq(0).removeClass('ac');
                        }else{
                            $('div#divReportGb > a').eq(1).removeClass('ac');   
                            $('div#divReportGb > a').eq(0).addClass('ac');  
                        }*/

                        //손익계산서 연간/분기 클릭
        				var btnSonikYQ_Vals = ["Y","Q"];
                        $('#btnSonikYQ a').click(function(){
                            $(this).parent().find('a').removeClass('ac');
                            $(this).addClass('ac');
        					var rowNum = $('a',$('#btnSonikYQ')).index(this);

                            $('#sonkiYQ').val(btnSonikYQ_Vals[rowNum]);
        					$.cookie('finance_cook_sonkiYQ', $('#sonkiYQ').val());

                            if ($('#sonkiYQ').val() == 'Y'){
                                $('#divSonikY').show();
                                $('#divSonikQ').hide();
                                $('#sonikChart1').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_01/A005930_'+$('#ReportGB').val()+'_D_03_01.png');
                                $('#sonikChart2').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_02/A005930_'+$('#ReportGB').val()+'_D_03_02.png');
        						$('#btnSonikYQText').html("[연간]");
                            }else{
                                $('#divSonikY').hide();
                                $('#divSonikQ').show();
                                $('#sonikChart1').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_01/A005930_'+$('#ReportGB').val()+'_Q_03_01.png');
                                $('#sonikChart2').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_02/A005930_'+$('#ReportGB').val()+'_Q_03_02.png');
        						$('#btnSonikYQText').html("[분기]");
                            }
        					return false;
                        });
                        //재무상태표 연간/분기 클릭
        				var btnDaechaYQ_Vals = ["Y","Q"];
                        $('#btnDaechaYQ a').click(function(){
                            $(this).parent().find('a').removeClass('ac');
                            $(this).addClass('ac');
        					var rowNum = $('a',$('#btnDaechaYQ')).index(this);

                            $('#daechaYQ').val(btnDaechaYQ_Vals[rowNum]);
        					$.cookie('finance_cook_daechaYQ', $('#daechaYQ').val());
                            if ($('#daechaYQ').val() == 'Y'){
                                $('#divDaechaY').show();
                                $('#divDaechaQ').hide();
                                $('#deachaChart1').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_03/A005930_'+$('#ReportGB').val()+'_D_03_03.png');
                                $('#deachaChart2').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_04/A005930_'+$('#ReportGB').val()+'_D_03_04.png');
        						$('#btnDaechaYQText').html("[연간]");
                            }else{
                                $('#divDaechaY').hide();
                                $('#divDaechaQ').show();
                                $('#deachaChart1').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_03/A005930_'+$('#ReportGB').val()+'_Q_03_03.png');
                                $('#deachaChart2').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_04/A005930_'+$('#ReportGB').val()+'_Q_03_04.png');
        						$('#btnDaechaYQText').html("[분기]");
                            }
        					return false;
                        });

                        //현금흐름표 연간/분기 클릭
        				var btnCashYQ_Vals = ["Y","Q"];
                        $('#btnCashYQ a').click(function(){
                            $(this).parent().find('a').removeClass('ac');
                            $(this).addClass('ac');
        					var rowNum = $('a',$('#btnCashYQ')).index(this);

                            $('#cashYQ').val(btnCashYQ_Vals[rowNum]);
        					$.cookie('finance_cook_cashYQ', $('#cashYQ').val());
                            if ($('#cashYQ').val() == 'Y'){
                                $('#divCashY').show();
                                $('#divCashQ').hide();
                                $('#cashChart1').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_05/A005930_'+$('#ReportGB').val()+'_D_03_05.png');
                                $('#cashChart2').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_06/A005930_'+$('#ReportGB').val()+'_D_03_06.png');
        						$('#btnCashYQText').html("[연간]");
                            }else{
                                $('#divCashY').hide();
                                $('#divCashQ').show();
                                $('#cashChart1').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_05/A005930_'+$('#ReportGB').val()+'_Q_03_05.png');
                                $('#cashChart2').attr('src', 'http://cdn.fnguide.com/SVO2/chartImg/03_06/A005930_'+$('#ReportGB').val()+'_Q_03_06.png');
        						$('#btnCashYQText').html("[분기]");
                            }
        					return false;
                        });


        				//차트감추기 클릭
                        $('.chartHide').click(function(){
                            $('.chartHide').hide();
        					$('.chartShow').show();
        					$('.ul_col2_l').hide();
        					$('.ul_col2_r').hide();
        					return false;
                        });

        				//차트보이기 클릭
                        $('.chartShow').click(function(){
                            $('.chartHide').show();
        					$('.chartShow').hide();
        					$('.ul_col2_l').show();
        					$('.ul_col2_r').show();
        					return false;
                        });


                    }
                });
            </script>
        </head>
        <body class="">
        <div id="skip_navi">
         <a href="#compBody" onclick="document.getElementById('compBody').tabIndex=-1;document.getElementById('compBody').focus();return false;">본문 바로가기</a>
         <a href="#compGnb" onclick="document.getElementById('compGnb').tabIndex=-1;document.getElementById('compGnb').focus();return false;">메뉴 바로가기</a>
        </div>
            

        <div class="fng_body asp_body">
            <div class="fng_wrap">
                
                    <script language="javascript">
                        $(function() {

                        });
                    </script>
             <!-- GNB -->
                <div class="contwrap" id="compBody"><!-- 컨텐츠 페이지 구성은 섹션으로 구성됨  -->
                    <!-- 공통기업개요 -->
                    
                    <!-- 공통기업개요 -->
        			<input type="hidden" id="strMarket" value="KSE" />
        			<div class="section ul_corpinfo">
        				<div class="corp_group1">
        					<h1 id="giName">삼성전자</h1>

        					<h2>005930</h2>
        					<span class="etc1">&#124;</span>
        					<h2>12월 결산</h2>
        					<span class="etc1">&#124;</span>
        					<!--
        					<h3>SamsungElec</h3>
        					<span class="etc1">&#124;</span>
        					-->
        					<a href="javascript:goHompage('http://www.samsung.com')" id="btn_shome" class="btn_shome"><span>홈페이지</span></a>
        					<dl style="display:none;" id="dl_btn_shome"><dt>홈페이지</dt><dd id="dd_btn_shome">http://www.samsung.com</dd></dl>
        					<a href="javascript:void(0)" id="btn_sphone" class="btn_sphone"><span>전화번호</span></a>
        					<dl style="display:none;" id="dl_btn_sphone"><dt>전화번호</dt><dd id="dd_btn_sphone">031-200-1114 | IR 담당자 031-200-1114</dd></dl>
        					<a href="javascript:void(0)"  id="btn_spmap" class="btn_spmap"><span>주소</span></a>
        					
        					<dl style="display:none;" id="dl_btn_spmap"><dt>주소</dt><dd id="dd_btn_spmap">경기도 수원시 영통구 삼성로 129 (매탄동)</dd></dl>
        					<p class="stxt_group">
        						<span class="stxt stxt1">KSE&nbsp;&nbsp;코스피 전기,전자</span>
        						<span style="display:none" id="strMarketTxt">코스피 전기,전자</span>
        						<span class="etc2">&#124;</span>
        						<span class="stxt stxt2">FICS  반도체&nbsp;및&nbsp;관련장비</span>
        <!--						<span class="etc2">&#124;</span>-->
        <!--						<span class="stxt stxt3">12월 결산</span>-->
        						
        							<span class="etc2">&#124;</span>
        							<span class="stxt stxt3">K200</span>
        						

        					</p>
        				</div>
        				<div class="corp_group2" id="corp_group2">
        					<dl>
        						<dt>
        							<dl style="display:none;"><dt>PER(Price Earning Ratio)</dt><dd>전일자 보통주 수정주가 / 최근 결산 EPS(주당순이익) <br>* EPS = 당기순이익 / 수정평균발행주식수<br>* 최근결산은 2021/12 (연간) 기준임.</dd></dl>
        							<a href="javascript:void(0)" class="tip_in" id="h_per">PER</a>
        						</dt>
        						<dd>11.18</dd>
        					</dl>
        					<dl>
        						<dt>
        							<dl style="display:none;"><dt>12M PER</dt><dd>전일자 보통주 수정주가 / 12개월 Forward EPS</dd></dl>
        							<a href="javascript:void(0)" class="tip_in" id="h_12m">12M PER</a>
        						</dt>
        						<dd>23.30</dd>
        					</dl>
        					<dl>
        						<dt>
        							<dl style="display:none;"><dt>업종 PER</dt><dd>시장대표업종||SUM(구성종목 시가총액)/SUM(구성종목 당기순이익)<br>* 전일자 보통주 시가총액 기준<br>* 당기순이익은  최근결산 2021/12 (연간) 기준임.</dd></dl>
        							<a href="javascript:void(0)" class="tip_in" id="h_u_per"><span class="ko">업종</span> PER</a>
        						</dt>
        						<dd>12.46</dd>
        					</dl>
        					<dl>
        						<dt>
        							<dl style="display:none;"><dt>PBR(Price Book-value Ratio)</dt><dd>전일자 보통주 수정주가 / 최근 결산기 BPS(주당순자산) <br>* BPS=(지배주주지분-자기주식) / 무상조정기말주식수(우선주 및 자사주 포함) <br>* 최근결산은 2021/12 (연간) 기준임.</dd></dl>
        							<a href="javascript:void(0)" class="tip_in" id="h_pbr">PBR</a>
        						</dt>
        						<dd>1.48</dd>
        					</dl>
        					<dl>
        						<dt>
        							<dl style="display:none;"><dt>배당수익률</dt><dd>{최근 결산기 보통주 DPS(현금, 무상조정) / 전일자 보통주 수정주가} *100<br>* 최근결산은 2021/12 (연간) 기준임.</dd></dl>
        							<a href="javascript:void(0)" class="tip_in" id="h_rate"><span class="ko">배당수익률</span></a>
        						</dt>
        						
        							<dd>2.24%</dd>
        						
        					</dl>
        				</div>
        			</div>

                    <div class="section ul_de">
                        <!-- 데이터 없음 -->
                        
                            <div class="ul_wrap">
                                <div class="ul_tab_ty1">
                                    <div class="btline">
                                        <div class="um_gbtn tab_ty grd_ty1" id="divReportGb">
                                            <a class="reportGbTab gbtn_f r3 " href="?pGB=1&amp;gicode=A005930&amp;cID=&amp;MenuYn=N&amp;ReportGB=D&amp;NewMenuID=103&amp;stkGb=701">연결</a>
                                                <a class="reportGbTab gbtn_e r3 " href="?pGB=1&amp;gicode=A005930&amp;cID=&amp;MenuYn=N&amp;ReportGB=B&amp;NewMenuID=103&amp;stkGb=701">별도</a>
                                        </div>
                                    </div>
        							<div class="btns_rgt">
        								<a href="javascript:void(0);" class="us_btn chartHide"  style="display:block;"><span class="img"></span>차트 감추기</a>
        								<a href="javascript:void(0);" class="us_btn chartShow"  style="display:none;"><span class="img"></span>차트 보이기</a> 
        							</div>
                                </div>
                            </div>
                            <!-- 포괄손익계산서 -->
                            <div class="ul_col2wrap pd_t25">
                                <div class="um_topbar bmshadow">
                                    <div class="topbar_wrap">
                                        <div class="topbar_lft">
                                            <h2>포괄손익계산서</h2>
        									<span class="stxt" id="btnSonikYQText"></span>
                                        </div>
                                        <div class="topbar_rgt">
                                            <span class="txt1">단위 : 억원</span>
                                            <div id="btnSonikYQ" class="um_gbtn grd_ty2 fl">
                                                <a class="gbtn_f r3" id="btnSonik_Y" href="javascript:void(0);">연간</a>
                                                <a class="gbtn_e r3"  id="btnSonik_Q" href="javascript:void(0);">분기</a>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="ul_col2_l">
                                    <div class="ul_wrap ul_col_l">
                                        <div class="um_topbar botbd_n">
                                            <div class="topbar_wrap">
                                                <div class="topbar_lft">
                                                    <h2>주요 재무항목</h2>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="um_chart chart_ht1">
                                            <div class="chart_wrap">
                                                <a class="chartdatapop" href="javascript:openGridPopup('sonikChart1');" title="클릭:차트데이터 새창열림"><img id="sonikChart1" src="http://cdn.fnguide.com/SVO2/chartImg/03_01/A005930_D_D_03_01.png"  alt="손익계산서(주요재무항목): 자세한 내용은 클릭후 팝업창 참고" onerror="this.src='http://cdn.fnguide.com/SVO/Handbook_New/images/nodata.png'"/></a>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="ul_col2_r">
                                    <div class="ul_wrap ul_col_r">
                                        <div class="um_topbar botbd_n">
                                            <div class="topbar_wrap">
                                                <div class="topbar_lft">
                                                    <h2>성장성 지표</h2>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="um_chart chart_ht1">
                                            <div class="chart_wrap">
                                                <a class="chartdatapop" href="javascript:openGridPopup('sonikChart2');" title="클릭:차트데이터 새창열림"><img id="sonikChart2"  src="http://cdn.fnguide.com/SVO2/chartImg/03_02/A005930_D_D_03_02.png"  alt="손익계산서(성장성지표): 자세한 내용은 클릭후 팝업창 참고" onerror="this.src='http://cdn.fnguide.com/SVO/Handbook_New/images/nodata.png'"/></a>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="ul_co1_c pd_t1">
                                    <!--포괄손익계산서 연간 데이타 테이블 시작-->  
                                    <div class="um_table" id="divSonikY" style="display:none;">
                                        

                                        <table  class="us_table_ty1 h_fix zigbg_no" >
                                            <caption class="cphidden">포괄손익계산서</caption>
                                            <colgroup>
                                                <col style="width: 35%;">
                                                
                                                     <col >
                                                
                                                     <col >
                                                
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
                                                    
                                                    <th scope="col">전년동기</th>
                                                    <th scope="col" class="cle">전년동기(%)</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">매출액</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="2,304,008.81">2,304,009</td>
                                                                    
                                                                        <td class="r" title="2,368,069.88">2,368,070</td>
                                                                    
                                                                        <td class="r" title="2,796,047.99">2,796,048</td>
                                                                    
                                                                        <td class="r" title="2,317,667.85">2,317,668</td>
                                                                    
                                                                        <td class="r" title="2,030,392.75">2,030,393</td>
                                                                    
                                                                        <td class="r cle">14.1</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">매출원가</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="1,472,395.49">1,472,395</td>
                                                                    
                                                                        <td class="r" title="1,444,882.96">1,444,883</td>
                                                                    
                                                                        <td class="r" title="1,664,113.42">1,664,113</td>
                                                                    
                                                                        <td class="r" title="1,414,140.42">1,414,140</td>
                                                                    
                                                                        <td class="r" title="1,214,647.87">1,214,648</td>
                                                                    
                                                                        <td class="r cle">16.4</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">매출총이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="831,613.32">831,613</td>
                                                                    
                                                                        <td class="r" title="923,186.92">923,187</td>
                                                                    
                                                                        <td class="r" title="1,131,934.57">1,131,935</td>
                                                                    
                                                                        <td class="r" title="903,527.43">903,527</td>
                                                                    
                                                                        <td class="r" title="815,744.88">815,745</td>
                                                                    
                                                                        <td class="r cle">10.8</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1_4" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">판매비와관리비</span><a id="grid1_4" href="javascript:foldOpen('grid1_4');" class=" btn_acdopen"><span class="blind" id="span_grid1_4">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    

                                                                        <td class="r" title="553,928.23">553,928</td>
                                                                    

                                                                        <td class="r" title="563,248.16">563,248</td>
                                                                    

                                                                        <td class="r" title="615,596.01">615,596</td>
                                                                    

                                                                        <td class="r" title="512,822.44">512,822</td>
                                                                    

                                                                        <td class="r" title="438,073.44">438,073</td>
                                                                    
                                                                        <td class="r cle">17.1</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;인건비</th>
                                                                    
                                                                        <td class="r" title="64,225.90">64,226</td>
                                                                    
                                                                        <td class="r" title="70,428.54">70,429</td>
                                                                    
                                                                        <td class="r" title="75,568.04">75,568</td>
                                                                    
                                                                        <td class="r" title="60,862.64">60,863</td>
                                                                    
                                                                        <td class="r" title="54,633.88">54,634</td>
                                                                    
                                                                        <td class="r cle">11.4</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;유무형자산상각비</th>
                                                                    
                                                                        <td class="r" title="20,408.17">20,408</td>
                                                                    
                                                                        <td class="r" title="20,856.79">20,857</td>
                                                                    
                                                                        <td class="r" title="20,730.51">20,731</td>
                                                                    
                                                                        <td class="r" title="16,627.02">16,627</td>
                                                                    
                                                                        <td class="r" title="15,504.57">15,505</td>
                                                                    
                                                                        <td class="r cle">7.2</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;연구개발비</th>
                                                                    
                                                                        <td class="r" title="199,072.36">199,072</td>
                                                                    
                                                                        <td class="r" title="211,114.90">211,115</td>
                                                                    
                                                                        <td class="r" title="224,017.26">224,017</td>
                                                                    
                                                                        <td class="r" title="184,466.87">184,467</td>
                                                                    
                                                                        <td class="r" title="159,329.50">159,330</td>
                                                                    
                                                                        <td class="r cle">15.8</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;광고선전비</th>
                                                                    
                                                                        <td class="r" title="46,145.25">46,145</td>
                                                                    
                                                                        <td class="r" title="42,690.43">42,690</td>
                                                                    
                                                                        <td class="r" title="53,760.15">53,760</td>
                                                                    
                                                                        <td class="r" title="46,445.84">46,446</td>
                                                                    
                                                                        <td class="r" title="37,670.52">37,671</td>
                                                                    
                                                                        <td class="r cle">23.3</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;판매비</th>
                                                                    
                                                                        <td class="r" title="117,221.74">117,222</td>
                                                                    
                                                                        <td class="r" title="114,487.77">114,488</td>
                                                                    
                                                                        <td class="r" title="131,184.91">131,185</td>
                                                                    
                                                                        <td class="r" title="109,633.83">109,634</td>
                                                                    
                                                                        <td class="r" title="92,625.97">92,626</td>
                                                                    
                                                                        <td class="r cle">18.4</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;관리비</th>
                                                                    
                                                                        <td class="r" title="55,130.83">55,131</td>
                                                                    
                                                                        <td class="r" title="56,787.03">56,787</td>
                                                                    
                                                                        <td class="r" title="61,925.68">61,926</td>
                                                                    
                                                                        <td class="r" title="51,123.93">51,124</td>
                                                                    
                                                                        <td class="r" title="43,750.91">43,751</td>
                                                                    
                                                                        <td class="r cle">16.9</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타원가성비용</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타</th>
                                                                    
                                                                        <td class="r" title="51,723.98">51,724</td>
                                                                    
                                                                        <td class="r" title="46,882.70">46,883</td>
                                                                    
                                                                        <td class="r" title="48,409.46">48,409</td>
                                                                    
                                                                        <td class="r" title="43,662.31">43,662</td>
                                                                    
                                                                        <td class="r" title="34,558.09">34,558</td>
                                                                    
                                                                        <td class="r cle">26.3</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">영업이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="277,685.09">277,685</td>
                                                                    
                                                                        <td class="r" title="359,938.76">359,939</td>
                                                                    
                                                                        <td class="r" title="516,338.56">516,339</td>
                                                                    
                                                                        <td class="r" title="390,704.99">390,705</td>
                                                                    
                                                                        <td class="r" title="377,671.44">377,671</td>
                                                                    
                                                                        <td class="r cle">3.5</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">영업이익(발표기준)</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="277,685.09">277,685</td>
                                                                    
                                                                        <td class="r" title="359,938.76">359,939</td>
                                                                    
                                                                        <td class="r" title="516,338.56">516,339</td>
                                                                    
                                                                        <td class="r" title="390,704.99">390,705</td>
                                                                    
                                                                        <td class="r" title="377,671.44">377,671</td>
                                                                    
                                                                        <td class="r cle">3.5</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1_7" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">금융수익</span><a id="grid1_7" href="javascript:foldOpen('grid1_7');" class=" btn_acdopen"><span class="blind" id="span_grid1_7">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    

                                                                        <td class="r" title="101,616.32">101,616</td>
                                                                    

                                                                        <td class="r" title="122,676.00">122,676</td>
                                                                    

                                                                        <td class="r" title="85,431.87">85,432</td>
                                                                    

                                                                        <td class="r" title="156,330.46">156,330</td>
                                                                    

                                                                        <td class="r" title="66,866.61">66,867</td>
                                                                    
                                                                        <td class="r cle">133.8</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;이자수익</th>
                                                                    
                                                                        <td class="r" title="26,600.24">26,600</td>
                                                                    
                                                                        <td class="r" title="19,744.58">19,745</td>
                                                                    
                                                                        <td class="r" title="12,782.78">12,783</td>
                                                                    
                                                                        <td class="r" title="16,556.68">16,557</td>
                                                                    
                                                                        <td class="r" title="9,239.96">9,240</td>
                                                                    
                                                                        <td class="r cle">79.2</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;배당금수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;외환이익</th>
                                                                    
                                                                        <td class="r" title="67,690.00">67,690</td>
                                                                    
                                                                        <td class="r" title="92,700.39">92,700</td>
                                                                    
                                                                        <td class="r" title="65,256.76">65,257</td>
                                                                    
                                                                        <td class="r" title="126,563.05">126,563</td>
                                                                    
                                                                        <td class="r" title="52,577.09">52,577</td>
                                                                    
                                                                        <td class="r cle">140.7</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;대손충당금환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;매출채권처분이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산관련이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산처분이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산손상차손환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;파생상품이익</th>
                                                                    
                                                                        <td class="r" title="7,326.08">7,326</td>
                                                                    
                                                                        <td class="r" title="10,231.03">10,231</td>
                                                                    
                                                                        <td class="r" title="7,392.33">7,392</td>
                                                                    
                                                                        <td class="r" title="13,210.73">13,211</td>
                                                                    
                                                                        <td class="r" title="5,049.56">5,050</td>
                                                                    
                                                                        <td class="r cle">161.6</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타금융수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1_8" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">금융원가</span><a id="grid1_8" href="javascript:foldOpen('grid1_8');" class=" btn_acdopen"><span class="blind" id="span_grid1_8">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    

                                                                        <td class="r" title="82,748.71">82,749</td>
                                                                    

                                                                        <td class="r" title="113,180.55">113,181</td>
                                                                    

                                                                        <td class="r" title="77,045.54">77,046</td>
                                                                    

                                                                        <td class="r" title="142,658.52">142,659</td>
                                                                    

                                                                        <td class="r" title="60,072.45">60,072</td>
                                                                    
                                                                        <td class="r cle">137.5</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;이자비용</th>
                                                                    
                                                                        <td class="r" title="6,863.56">6,864</td>
                                                                    
                                                                        <td class="r" title="5,830.13">5,830</td>
                                                                    
                                                                        <td class="r" title="4,315.40">4,315</td>
                                                                    
                                                                        <td class="r" title="4,887.58">4,888</td>
                                                                    
                                                                        <td class="r" title="3,076.84">3,077</td>
                                                                    
                                                                        <td class="r cle">58.9</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;외환손실</th>
                                                                    
                                                                        <td class="r" title="68,524.09">68,524</td>
                                                                    
                                                                        <td class="r" title="98,685.91">98,686</td>
                                                                    
                                                                        <td class="r" title="64,860.93">64,861</td>
                                                                    
                                                                        <td class="r" title="126,106.03">126,106</td>
                                                                    
                                                                        <td class="r" title="51,054.51">51,055</td>
                                                                    
                                                                        <td class="r cle">147.0</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;대손상각비</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산관련손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;매출채권처분손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산처분손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산손상차손</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;파생상품손실</th>
                                                                    
                                                                        <td class="r" title="7,361.06">7,361</td>
                                                                    
                                                                        <td class="r" title="8,664.51">8,665</td>
                                                                    
                                                                        <td class="r" title="7,869.21">7,869</td>
                                                                    
                                                                        <td class="r" title="11,664.91">11,665</td>
                                                                    
                                                                        <td class="r" title="5,941.10">5,941</td>
                                                                    
                                                                        <td class="r cle">96.3</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타금융원가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1_9" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">기타수익</span><a id="grid1_9" href="javascript:foldOpen('grid1_9');" class=" btn_acdopen"><span class="blind" id="span_grid1_9">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    

                                                                        <td class="r" title="17,786.66">17,787</td>
                                                                    

                                                                        <td class="r" title="13,840.68">13,841</td>
                                                                    

                                                                        <td class="r" title="22,056.95">22,057</td>
                                                                    

                                                                        <td class="r" title="14,803.48">14,803</td>
                                                                    

                                                                        <td class="r" title="14,655.62">14,656</td>
                                                                    
                                                                        <td class="r cle">1.0</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;이자수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;배당금수익</th>
                                                                    
                                                                        <td class="r" title="1,546.79">1,547</td>
                                                                    
                                                                        <td class="r" title="1,524.40">1,524</td>
                                                                    
                                                                        <td class="r" title="1,358.40">1,358</td>
                                                                    
                                                                        <td class="r" title="3,735.28">3,735</td>
                                                                    
                                                                        <td class="r" title="987.38">987</td>
                                                                    
                                                                        <td class="r cle">278.3</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;외환이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;재고자산감모손실환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;재고자산폐기(처분)이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산처분(폐기)이익</th>
                                                                    
                                                                        <td class="r" title="3,524.36">3,524</td>
                                                                    
                                                                        <td class="r" title="1,542.49">1,542</td>
                                                                    
                                                                        <td class="r" title="3,404.00">3,404</td>
                                                                    
                                                                        <td class="r" title="1,359.40">1,359</td>
                                                                    
                                                                        <td class="r" title="3,196.20">3,196</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-57.5</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산손상차손환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;파생상품이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;임대료수익</th>
                                                                    
                                                                        <td class="r" title="1,533.57">1,534</td>
                                                                    
                                                                        <td class="r" title="1,471.04">1,471</td>
                                                                    
                                                                        <td class="r" title="1,328.01">1,328</td>
                                                                    
                                                                        <td class="r" title="1,050.09">1,050</td>
                                                                    
                                                                        <td class="r" title="994.02">994</td>
                                                                    
                                                                        <td class="r cle">5.6</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;로열티수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;수수료수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;대손충당금환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;충당부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타</th>
                                                                    
                                                                        <td class="r" title="11,181.94">11,182</td>
                                                                    
                                                                        <td class="r" title="9,302.75">9,303</td>
                                                                    
                                                                        <td class="r" title="15,966.54">15,967</td>
                                                                    
                                                                        <td class="r" title="8,658.71">8,659</td>
                                                                    
                                                                        <td class="r" title="9,478.02">9,478</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-8.6</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1_10" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">기타비용</span><a id="grid1_10" href="javascript:foldOpen('grid1_10');" class=" btn_acdopen"><span class="blind" id="span_grid1_10">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    

                                                                        <td class="r" title="14,147.07">14,147</td>
                                                                    

                                                                        <td class="r" title="24,889.02">24,889</td>
                                                                    

                                                                        <td class="r" title="20,559.71">20,560</td>
                                                                    

                                                                        <td class="r" title="13,370.37">13,370</td>
                                                                    

                                                                        <td class="r" title="15,239.30">15,239</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-12.3</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;이자비용</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;외환손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;재고자산감모손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;재고자산폐기(처분)손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산처분(폐기)손실</th>
                                                                    
                                                                        <td class="r" title="1,445.47">1,445</td>
                                                                    
                                                                        <td class="r" title="876.73">877</td>
                                                                    
                                                                        <td class="r" title="755.86">756</td>
                                                                    
                                                                        <td class="r" title="347.98">348</td>
                                                                    
                                                                        <td class="r" title="695.96">696</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-50.0</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산손상차손</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;파생상품손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타대손상각비</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;충당부채전입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타</th>
                                                                    
                                                                        <td class="r" title="12,701.60">12,702</td>
                                                                    
                                                                        <td class="r" title="24,012.29">24,012</td>
                                                                    
                                                                        <td class="r" title="19,803.85">19,804</td>
                                                                    
                                                                        <td class="r" title="13,022.39">13,022</td>
                                                                    
                                                                        <td class="r" title="14,543.34">14,543</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-10.5</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1_11" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">종속기업,공동지배기업및관계기업관련손익</span><a id="grid1_11" href="javascript:foldOpen('grid1_11');" class=" btn_acdopen"><span class="blind" id="span_grid1_11">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    

                                                                        <td class="r" title="4,129.60">4,130</td>
                                                                    

                                                                        <td class="r" title="5,065.30">5,065</td>
                                                                    

                                                                        <td class="r" title="7,296.14">7,296</td>
                                                                    

                                                                        <td class="r" title="8,045.52">8,046</td>
                                                                    

                                                                        <td class="r" title="6,007.52">6,008</td>
                                                                    
                                                                        <td class="r cle">33.9</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;지분법손익</th>
                                                                    
                                                                        <td class="r" title="4,129.60">4,130</td>
                                                                    
                                                                        <td class="r" title="5,065.30">5,065</td>
                                                                    
                                                                        <td class="r" title="7,296.14">7,296</td>
                                                                    
                                                                        <td class="r" title="8,045.52">8,046</td>
                                                                    
                                                                        <td class="r" title="6,007.52">6,008</td>
                                                                    
                                                                        <td class="r cle">33.9</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;종속기업,공동지배기업및관계기업투자주식처분손익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;종속기업,공동지배기업및관계기업투자주식손상관련손익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid1_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">세전계속사업이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="304,321.89">304,322</td>
                                                                    
                                                                        <td class="r" title="363,451.17">363,451</td>
                                                                    
                                                                        <td class="r" title="533,518.27">533,518</td>
                                                                    
                                                                        <td class="r" title="413,855.56">413,856</td>
                                                                    
                                                                        <td class="r" title="389,889.44">389,889</td>
                                                                    
                                                                        <td class="r cle">6.1</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">법인세비용</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="86,933.24">86,933</td>
                                                                    
                                                                        <td class="r" title="99,372.85">99,373</td>
                                                                    
                                                                        <td class="r" title="134,443.77">134,444</td>
                                                                    
                                                                        <td class="r" title="95,729.25">95,729</td>
                                                                    
                                                                        <td class="r" title="99,194.42">99,194</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-3.5</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">계속영업이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="217,388.65">217,389</td>
                                                                    
                                                                        <td class="r" title="264,078.32">264,078</td>
                                                                    
                                                                        <td class="r" title="399,074.50">399,075</td>
                                                                    
                                                                        <td class="r" title="318,126.31">318,126</td>
                                                                    
                                                                        <td class="r" title="290,695.02">290,695</td>
                                                                    
                                                                        <td class="r cle">9.4</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">중단영업이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">당기순이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="217,388.65">217,389</td>
                                                                    
                                                                        <td class="r" title="264,078.32">264,078</td>
                                                                    
                                                                        <td class="r" title="399,074.50">399,075</td>
                                                                    
                                                                        <td class="r" title="318,126.31">318,126</td>
                                                                    
                                                                        <td class="r" title="290,695.02">290,695</td>
                                                                    
                                                                        <td class="r cle">9.4</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;지배주주순이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="215,050.54">215,051</td>
                                                                    
                                                                        <td class="r" title="260,908.46">260,908</td>
                                                                    
                                                                        <td class="r" title="392,437.91">392,438</td>
                                                                    
                                                                        <td class="r" title="312,275.09">312,275</td>
                                                                    
                                                                        <td class="r" title="286,006.69">286,007</td>
                                                                    
                                                                        <td class="r cle">9.2</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;비지배주주순이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="2,338.11">2,338</td>
                                                                    
                                                                        <td class="r" title="3,169.86">3,170</td>
                                                                    
                                                                        <td class="r" title="6,636.59">6,637</td>
                                                                    
                                                                        <td class="r" title="5,851.22">5,851</td>
                                                                    
                                                                        <td class="r" title="4,688.33">4,688</td>
                                                                    
                                                                        <td class="r cle">24.8</td>
                                                                    
                                                            </tr>
                                                

                                            </tbody>
                                        </table>
                                    </div>
                                    <!--포괄손익계산서 분기 데이타 테이블 시작-->  
                                    <div class="um_table" id="divSonikQ" style="display:none;">
                                        

                                        <table  class="us_table_ty1 h_fix zigbg_no" >
                                            <caption class="cphidden">포괄손익계산서</caption>
                                            <colgroup>
                                                <col style="width: 35%;">
                                                
                                                     <col >
                                                
                                                     <col >
                                                
                                                     <col >
                                                
                                                     <col >
                                                
                                                     <col >
                                                
                                                     <col >
                                                
                                            </colgroup>
                                            <thead>
                                                <tr>
                                                    <th scope="col" class="clf tbold">IFRS(연결)</th>
                                                    
                                                        <th scope="col">2021/12</th>
                                                    
                                                        <th scope="col">2022/03</th>
                                                    
                                                        <th scope="col">2022/06</th>
                                                    
                                                        <th scope="col">2022/09</th>
                                                    
                                                    <th scope="col">전년동기</th>
                                                    <th scope="col" class="cle">전년동기(%)</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">매출액</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="765,655.24">765,655</td>
                                                                    
                                                                        <td class="r" title="777,814.98">777,815</td>
                                                                    
                                                                        <td class="r" title="772,036.07">772,036</td>
                                                                    
                                                                        <td class="r" title="767,816.80">767,817</td>
                                                                    
                                                                        <td class="r" title="739,791.87">739,792</td>
                                                                    
                                                                        <td class="r cle">3.8</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">매출원가</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="449,465.55">449,466</td>
                                                                    
                                                                        <td class="r" title="470,720.57">470,721</td>
                                                                    
                                                                        <td class="r" title="462,697.48">462,697</td>
                                                                    
                                                                        <td class="r" title="480,722.37">480,722</td>
                                                                    
                                                                        <td class="r" title="428,988.71">428,989</td>
                                                                    
                                                                        <td class="r cle">12.1</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">매출총이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="316,189.69">316,190</td>
                                                                    
                                                                        <td class="r" title="307,094.41">307,094</td>
                                                                    
                                                                        <td class="r" title="309,338.59">309,339</td>
                                                                    
                                                                        <td class="r" title="287,094.43">287,094</td>
                                                                    
                                                                        <td class="r" title="310,803.16">310,803</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-7.6</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1Q_4" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">판매비와관리비</span><a id="grid1Q_4" href="javascript:foldOpen('grid1Q_4');" class=" btn_acdopen"><span class="blind" id="span_grid1Q_4">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="177,522.57">177,523</td>
                                                                    
                                                                        <td class="r" title="165,880.32">165,880</td>
                                                                    
                                                                        <td class="r" title="168,368.14">168,368</td>
                                                                    
                                                                        <td class="r" title="178,573.98">178,574</td>
                                                                    
                                                                        <td class="r" title="152,627.85">152,628</td>
                                                                    
                                                                        <td class="r cle">17.0</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;인건비</th>
                                                                    
                                                                        <td class="r" title="20,934.16">20,934</td>
                                                                    
                                                                        <td class="r" title="20,726.47">20,726</td>
                                                                    
                                                                        <td class="r" title="19,571.71">19,572</td>
                                                                    
                                                                        <td class="r" title="20,564.46">20,564</td>
                                                                    
                                                                        <td class="r" title="18,634.32">18,634</td>
                                                                    
                                                                        <td class="r cle">10.4</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;유무형자산상각비</th>
                                                                    
                                                                        <td class="r" title="5,225.94">5,226</td>
                                                                    
                                                                        <td class="r" title="5,472.35">5,472</td>
                                                                    
                                                                        <td class="r" title="5,459.05">5,459</td>
                                                                    
                                                                        <td class="r" title="5,695.62">5,696</td>
                                                                    
                                                                        <td class="r" title="5,193.75">5,194</td>
                                                                    
                                                                        <td class="r cle">9.7</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;연구개발비</th>
                                                                    
                                                                        <td class="r" title="64,687.76">64,688</td>
                                                                    
                                                                        <td class="r" title="59,222.32">59,222</td>
                                                                    
                                                                        <td class="r" title="62,548.74">62,549</td>
                                                                    
                                                                        <td class="r" title="62,695.81">62,696</td>
                                                                    
                                                                        <td class="r" title="51,098.90">51,099</td>
                                                                    
                                                                        <td class="r cle">22.7</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;광고선전비</th>
                                                                    
                                                                        <td class="r" title="16,089.63">16,090</td>
                                                                    
                                                                        <td class="r" title="14,677.01">14,677</td>
                                                                    
                                                                        <td class="r" title="13,974.80">13,975</td>
                                                                    
                                                                        <td class="r" title="17,794.03">17,794</td>
                                                                    
                                                                        <td class="r" title="15,376.92">15,377</td>
                                                                    
                                                                        <td class="r cle">15.7</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;판매비</th>
                                                                    
                                                                        <td class="r" title="38,558.94">38,559</td>
                                                                    
                                                                        <td class="r" title="35,745.75">35,746</td>
                                                                    
                                                                        <td class="r" title="33,580.59">33,581</td>
                                                                    
                                                                        <td class="r" title="40,307.49">40,307</td>
                                                                    
                                                                        <td class="r" title="35,628.16">35,628</td>
                                                                    
                                                                        <td class="r cle">13.1</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;관리비</th>
                                                                    
                                                                        <td class="r" title="18,174.77">18,175</td>
                                                                    
                                                                        <td class="r" title="17,303.91">17,304</td>
                                                                    
                                                                        <td class="r" title="18,069.07">18,069</td>
                                                                    
                                                                        <td class="r" title="15,750.95">15,751</td>
                                                                    
                                                                        <td class="r" title="14,454.61">14,455</td>
                                                                    
                                                                        <td class="r cle">9.0</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타원가성비용</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타</th>
                                                                    
                                                                        <td class="r" title="13,851.37">13,851</td>
                                                                    
                                                                        <td class="r" title="12,732.51">12,733</td>
                                                                    
                                                                        <td class="r" title="15,164.18">15,164</td>
                                                                    
                                                                        <td class="r" title="15,765.62">15,766</td>
                                                                    
                                                                        <td class="r" title="12,241.19">12,241</td>
                                                                    
                                                                        <td class="r cle">28.8</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">영업이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="138,667.12">138,667</td>
                                                                    
                                                                        <td class="r" title="141,214.09">141,214</td>
                                                                    
                                                                        <td class="r" title="140,970.45">140,970</td>
                                                                    
                                                                        <td class="r" title="108,520.45">108,520</td>
                                                                    
                                                                        <td class="r" title="158,175.31">158,175</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-31.4</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">영업이익(발표기준)</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="138,667.12">138,667</td>
                                                                    
                                                                        <td class="r" title="141,214.09">141,214</td>
                                                                    
                                                                        <td class="r" title="140,970.45">140,970</td>
                                                                    
                                                                        <td class="r" title="108,520.45">108,520</td>
                                                                    
                                                                        <td class="r" title="158,175.31">158,175</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-31.4</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1Q_7" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">금융수익</span><a id="grid1Q_7" href="javascript:foldOpen('grid1Q_7');" class=" btn_acdopen"><span class="blind" id="span_grid1Q_7">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="18,565.26">18,565</td>
                                                                    
                                                                        <td class="r" title="35,021.89">35,022</td>
                                                                    
                                                                        <td class="r" title="53,718.85">53,719</td>
                                                                    
                                                                        <td class="r" title="67,589.72">67,590</td>
                                                                    
                                                                        <td class="r" title="26,608.80">26,609</td>
                                                                    
                                                                        <td class="r cle">154.0</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;이자수익</th>
                                                                    
                                                                        <td class="r" title="3,542.82">3,543</td>
                                                                    
                                                                        <td class="r" title="3,690.32">3,690</td>
                                                                    
                                                                        <td class="r" title="4,871.94">4,872</td>
                                                                    
                                                                        <td class="r" title="7,994.42">7,994</td>
                                                                    
                                                                        <td class="r" title="3,373.15">3,373</td>
                                                                    
                                                                        <td class="r cle">137.0</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;배당금수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;외환이익</th>
                                                                    
                                                                        <td class="r" title="12,679.67">12,680</td>
                                                                    
                                                                        <td class="r" title="26,086.03">26,086</td>
                                                                    
                                                                        <td class="r" title="44,908.04">44,908</td>
                                                                    
                                                                        <td class="r" title="55,568.98">55,569</td>
                                                                    
                                                                        <td class="r" title="21,406.70">21,407</td>
                                                                    
                                                                        <td class="r cle">159.6</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;대손충당금환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;매출채권처분이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산관련이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산처분이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산손상차손환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;파생상품이익</th>
                                                                    
                                                                        <td class="r" title="2,342.77">2,343</td>
                                                                    
                                                                        <td class="r" title="5,245.54">5,246</td>
                                                                    
                                                                        <td class="r" title="3,938.87">3,939</td>
                                                                    
                                                                        <td class="r" title="4,026.32">4,026</td>
                                                                    
                                                                        <td class="r" title="1,828.95">1,829</td>
                                                                    
                                                                        <td class="r cle">120.1</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타금융수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1Q_8" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">금융원가</span><a id="grid1Q_8" href="javascript:foldOpen('grid1Q_8');" class=" btn_acdopen"><span class="blind" id="span_grid1Q_8">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="16,973.09">16,973</td>
                                                                    
                                                                        <td class="r" title="30,333.18">30,333</td>
                                                                    
                                                                        <td class="r" title="51,309.79">51,310</td>
                                                                    
                                                                        <td class="r" title="61,015.55">61,016</td>
                                                                    
                                                                        <td class="r" title="24,906.46">24,906</td>
                                                                    
                                                                        <td class="r cle">145.0</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;이자비용</th>
                                                                    
                                                                        <td class="r" title="1,238.56">1,239</td>
                                                                    
                                                                        <td class="r" title="1,492.31">1,492</td>
                                                                    
                                                                        <td class="r" title="1,230.64">1,231</td>
                                                                    
                                                                        <td class="r" title="2,164.63">2,165</td>
                                                                    
                                                                        <td class="r" title="1,370.10">1,370</td>
                                                                    
                                                                        <td class="r cle">58.0</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;외환손실</th>
                                                                    
                                                                        <td class="r" title="13,806.42">13,806</td>
                                                                    
                                                                        <td class="r" title="24,606.82">24,607</td>
                                                                    
                                                                        <td class="r" title="46,755.75">46,756</td>
                                                                    
                                                                        <td class="r" title="54,743.46">54,743</td>
                                                                    
                                                                        <td class="r" title="21,843.73">21,844</td>
                                                                    
                                                                        <td class="r cle">150.6</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;대손상각비</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산관련손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;매출채권처분손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산처분손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;금융자산손상차손</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;파생상품손실</th>
                                                                    
                                                                        <td class="r" title="1,928.11">1,928</td>
                                                                    
                                                                        <td class="r" title="4,234.05">4,234</td>
                                                                    
                                                                        <td class="r" title="3,323.40">3,323</td>
                                                                    
                                                                        <td class="r" title="4,107.46">4,107</td>
                                                                    
                                                                        <td class="r" title="1,692.63">1,693</td>
                                                                    
                                                                        <td class="r cle">142.7</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타금융원가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1Q_9" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">기타수익</span><a id="grid1Q_9" href="javascript:foldOpen('grid1Q_9');" class=" btn_acdopen"><span class="blind" id="span_grid1Q_9">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="7,401.33">7,401</td>
                                                                    
                                                                        <td class="r" title="7,001.93">7,002</td>
                                                                    
                                                                        <td class="r" title="4,639.71">4,640</td>
                                                                    
                                                                        <td class="r" title="3,161.84">3,162</td>
                                                                    
                                                                        <td class="r" title="4,251.52">4,252</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-25.6</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;이자수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;배당금수익</th>
                                                                    
                                                                        <td class="r" title="371.02">371</td>
                                                                    
                                                                        <td class="r" title="2,685.09">2,685</td>
                                                                    
                                                                        <td class="r" title="624.00">624</td>
                                                                    
                                                                        <td class="r" title="426.19">426</td>
                                                                    
                                                                        <td class="r" title="216.69">217</td>
                                                                    
                                                                        <td class="r cle">96.7</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;외환이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;재고자산감모손실환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;재고자산폐기(처분)이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산처분(폐기)이익</th>
                                                                    
                                                                        <td class="r" title="207.80">208</td>
                                                                    
                                                                        <td class="r" title="504.20">504</td>
                                                                    
                                                                        <td class="r" title="637.80">638</td>
                                                                    
                                                                        <td class="r" title="217.40">217</td>
                                                                    
                                                                        <td class="r" title="1,135.99">1,136</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-80.9</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산손상차손환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;파생상품이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;임대료수익</th>
                                                                    
                                                                        <td class="r" title="333.99">334</td>
                                                                    
                                                                        <td class="r" title="345.84">346</td>
                                                                    
                                                                        <td class="r" title="349.14">349</td>
                                                                    
                                                                        <td class="r" title="355.11">355</td>
                                                                    
                                                                        <td class="r" title="326.17">326</td>
                                                                    
                                                                        <td class="r cle">8.9</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;로열티수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;수수료수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;대손충당금환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;충당부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_9 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타</th>
                                                                    
                                                                        <td class="r" title="6,488.52">6,489</td>
                                                                    
                                                                        <td class="r" title="3,466.80">3,467</td>
                                                                    
                                                                        <td class="r" title="3,028.77">3,029</td>
                                                                    
                                                                        <td class="r" title="2,163.14">2,163</td>
                                                                    
                                                                        <td class="r" title="2,572.67">2,573</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-15.9</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1Q_10" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">기타비용</span><a id="grid1Q_10" href="javascript:foldOpen('grid1Q_10');" class=" btn_acdopen"><span class="blind" id="span_grid1Q_10">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="5,320.41">5,320</td>
                                                                    
                                                                        <td class="r" title="4,531.10">4,531</td>
                                                                    
                                                                        <td class="r" title="6,003.93">6,004</td>
                                                                    
                                                                        <td class="r" title="2,835.34">2,835</td>
                                                                    
                                                                        <td class="r" title="3,227.94">3,228</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-12.2</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;이자비용</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;외환손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;재고자산감모손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;재고자산폐기(처분)손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산처분(폐기)손실</th>
                                                                    
                                                                        <td class="r" title="59.90">60</td>
                                                                    
                                                                        <td class="r" title="41.96">42</td>
                                                                    
                                                                        <td class="r" title="199.11">199</td>
                                                                    
                                                                        <td class="r" title="106.91">107</td>
                                                                    
                                                                        <td class="r" title="191.27">191</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-44.1</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;자산손상차손</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;파생상품손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타대손상각비</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;충당부채전입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타</th>
                                                                    
                                                                        <td class="r" title="5,260.51">5,261</td>
                                                                    
                                                                        <td class="r" title="4,489.14">4,489</td>
                                                                    
                                                                        <td class="r" title="5,804.82">5,805</td>
                                                                    
                                                                        <td class="r" title="2,728.43">2,728</td>
                                                                    
                                                                        <td class="r" title="3,036.67">3,037</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-10.2</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid1Q_11" class=" rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">종속기업,공동지배기업및관계기업관련손익</span><a id="grid1Q_11" href="javascript:foldOpen('grid1Q_11');" class=" btn_acdopen"><span class="blind" id="span_grid1Q_11">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="1,288.62">1,289</td>
                                                                    
                                                                        <td class="r" title="2,324.77">2,325</td>
                                                                    
                                                                        <td class="r" title="2,592.29">2,592</td>
                                                                    
                                                                        <td class="r" title="3,128.46">3,128</td>
                                                                    
                                                                        <td class="r" title="2,657.46">2,657</td>
                                                                    
                                                                        <td class="r cle">17.7</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;지분법손익</th>
                                                                    
                                                                        <td class="r" title="1,288.62">1,289</td>
                                                                    
                                                                        <td class="r" title="2,324.77">2,325</td>
                                                                    
                                                                        <td class="r" title="2,592.29">2,592</td>
                                                                    
                                                                        <td class="r" title="3,128.46">3,128</td>
                                                                    
                                                                        <td class="r" title="2,657.46">2,657</td>
                                                                    
                                                                        <td class="r cle">17.7</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;종속기업,공동지배기업및관계기업투자주식처분손익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;종속기업,공동지배기업및관계기업투자주식손상관련손익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="c_grid1Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;기타</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">세전계속사업이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="143,628.83">143,629</td>
                                                                    
                                                                        <td class="r" title="150,698.40">150,698</td>
                                                                    
                                                                        <td class="r" title="144,607.58">144,608</td>
                                                                    
                                                                        <td class="r" title="118,549.58">118,550</td>
                                                                    
                                                                        <td class="r" title="163,558.69">163,559</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-27.5</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">법인세비용</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="35,249.35">35,249</td>
                                                                    
                                                                        <td class="r" title="37,452.12">37,452</td>
                                                                    
                                                                        <td class="r" title="33,619.53">33,620</td>
                                                                    
                                                                        <td class="r" title="24,657.60">24,658</td>
                                                                    
                                                                        <td class="r" title="40,625.38">40,625</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-39.3</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">계속영업이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="108,379.48">108,379</td>
                                                                    
                                                                        <td class="r" title="113,246.28">113,246</td>
                                                                    
                                                                        <td class="r" title="110,988.05">110,988</td>
                                                                    
                                                                        <td class="r" title="93,891.98">93,892</td>
                                                                    
                                                                        <td class="r" title="122,933.31">122,933</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-23.6</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">중단영업이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">당기순이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="108,379.48">108,379</td>
                                                                    
                                                                        <td class="r" title="113,246.28">113,246</td>
                                                                    
                                                                        <td class="r" title="110,988.05">110,988</td>
                                                                    
                                                                        <td class="r" title="93,891.98">93,892</td>
                                                                    
                                                                        <td class="r" title="122,933.31">122,933</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-23.6</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;지배주주순이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="106,431.22">106,431</td>
                                                                    
                                                                        <td class="r" title="111,290.94">111,291</td>
                                                                    
                                                                        <td class="r" title="109,545.15">109,545</td>
                                                                    
                                                                        <td class="r" title="91,439.00">91,439</td>
                                                                    
                                                                        <td class="r" title="120,572.07">120,572</td>
                                                                    
                                                                        <td class="r cle"><span class='tcr'>-24.2</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;비지배주주순이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="1,948.26">1,948</td>
                                                                    
                                                                        <td class="r" title="1,955.34">1,955</td>
                                                                    
                                                                        <td class="r" title="1,442.90">1,443</td>
                                                                    
                                                                        <td class="r" title="2,452.98">2,453</td>
                                                                    
                                                                        <td class="r" title="2,361.24">2,361</td>
                                                                    
                                                                        <td class="r cle">3.9</td>
                                                                    
                                                            </tr>
                                                

                                            </tbody>
                                        </table>
                                    </div>
                                </div>
                            </div>
                            
                            <!-- 재무상태표 -->
                            <div class="ul_col2wrap pd_t25">
                                <div class="um_topbar bmshadow">
                                    <div class="topbar_wrap">
                                        <div class="topbar_lft">
                                            <h2>재무상태표</h2>
        									<span class="stxt" id="btnDaechaYQText"></span>
                                        </div>
                                        <div class="topbar_rgt">
                                            <span class="txt1">단위 : 억원</span>
                                            <div id="btnDaechaYQ" class="um_gbtn grd_ty2 fl">
                                                <a class="gbtn_f r3" id="btnDaecha_Y" href="javascript:void(0);">연간</a>
                                                <a class="gbtn_e r3" id="btnDaecha_Q" href="javascript:void(0);">분기</a>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="ul_col2_l">
                                    <div class="ul_wrap ul_col_l">
                                        <div class="um_topbar botbd_n">
                                            <div class="topbar_wrap">
                                                <div class="topbar_lft">
                                                    <h2>주요 재무항목</h2>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="um_chart chart_ht1">
                                            <div class="chart_wrap">
                                                <a class="chartdatapop" href="javascript:openGridPopup('deachaChart1');" title="클릭:차트데이터 새창열림"><img id="deachaChart1"  alt="재무상태표(주요재무항목): 자세한 내용은 클릭후 팝업창 참고" onerror="this.src='http://cdn.fnguide.com/SVO/Handbook_New/images/nodata.png'"/></a>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="ul_col2_r">
                                    <div class="ul_wrap ul_col_r">
                                        <div class="um_topbar botbd_n">
                                            <div class="topbar_wrap">
                                                <div class="topbar_lft">
                                                    <h2>안정성 지표</h2>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="um_chart chart_ht1">
                                            <div class="chart_wrap">
                                                <a class="chartdatapop" href="javascript:openGridPopup('deachaChart2');" title="클릭:차트데이터 새창열림"><img id="deachaChart2"  alt ="재무상태표(안정성지표): 자세한 내용은 클릭후 팝업창 참고" onerror="this.src='http://cdn.fnguide.com/SVO/Handbook_New/images/nodata.png'" /></a>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="ul_co1_c pd_t1">
                                    <!--재무상태표 연간 데이타 테이블 시작--> 
                                    <div class="um_table" id="divDaechaY" style="display:none;">
                                        
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
                                    </div>
                                    <!--재무상태표 분기간 데이타 테이블 시작--> 
                                    <div class="um_table" id="divDaechaQ" style="display:none;">
                                        <!--
                                        daecha_bungi
                                        -->
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
                                                    
                                                        <th scope="col">2021/12</th>
                                                    
                                                        <th scope="col">2022/03</th>
                                                    
                                                        <th scope="col">2022/06</th>
                                                    
                                                        <th scope="col">2022/09</th>
                                                    
                                                    
                                                </tr>
                                            </thead>
                                            <tbody>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">자산</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="4,266,211.58">4,266,212</td>
                                                                    
                                                                        <td class="r" title="4,393,269.59">4,393,270</td>
                                                                    
                                                                        <td class="r" title="4,480,406.50">4,480,407</td>
                                                                    
                                                                        <td class="r cle" title="4,702,784.09">4,702,784</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid2Q_2" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;유동자산</span><a id="grid2Q_2" href="javascript:foldOpen('grid2Q_2');" class=" btn_acdopen"><span class="blind" id="span_grid2Q_2">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="2,181,631.85">2,181,632</td>
                                                                    
                                                                        <td class="r" title="2,323,690.82">2,323,691</td>
                                                                    
                                                                        <td class="r" title="2,362,874.91">2,362,875</td>
                                                                    
                                                                        <td class="r cle" title="2,508,806.37">2,508,806</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_2 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;재고자산</th>
                                                                    
                                                                        <td class="r" title="413,844.04">413,844</td>
                                                                    
                                                                        <td class="r" title="475,907.31">475,907</td>
                                                                    
                                                                        <td class="r" title="520,922.41">520,922</td>
                                                                    
                                                                        <td class="r cle" title="573,198.48">573,198</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_2 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동생물자산</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_2 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동금융자산</th>
                                                                    
                                                                        <td class="r" title="851,187.77">851,188</td>
                                                                    
                                                                        <td class="r" title="769,297.07">769,297</td>
                                                                    
                                                                        <td class="r" title="857,375.23">857,375</td>
                                                                    
                                                                        <td class="r cle" title="843,006.78">843,007</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_2 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매출채권및기타유동채권</th>
                                                                    
                                                                        <td class="r" title="452,106.72">452,107</td>
                                                                    
                                                                        <td class="r" title="501,174.03">501,174</td>
                                                                    
                                                                        <td class="r" title="497,167.11">497,167</td>
                                                                    
                                                                        <td class="r cle" title="533,932.43">533,932</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_2 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;당기법인세자산</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_2 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약자산</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_2 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)자산</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_2 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출권</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_2 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타유동자산</th>
                                                                    
                                                                        <td class="r" title="74,179.17">74,179</td>
                                                                    
                                                                        <td class="r" title="87,862.59">87,863</td>
                                                                    
                                                                        <td class="r" title="91,578.75">91,579</td>
                                                                    
                                                                        <td class="r cle" title="113,514.47">113,514</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_2 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;현금및현금성자산</th>
                                                                    
                                                                        <td class="r" title="390,314.15">390,314</td>
                                                                    
                                                                        <td class="r" title="489,449.82">489,450</td>
                                                                    
                                                                        <td class="r" title="395,831.41">395,831</td>
                                                                    
                                                                        <td class="r cle" title="445,154.21">445,154</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_2 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매각예정비유동자산및처분자산집단</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid2Q_3" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;비유동자산</span><a id="grid2Q_3" href="javascript:foldOpen('grid2Q_3');" class=" btn_acdopen"><span class="blind" id="span_grid2Q_3">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="2,084,579.73">2,084,580</td>
                                                                    
                                                                        <td class="r" title="2,069,578.77">2,069,579</td>
                                                                    
                                                                        <td class="r" title="2,117,531.59">2,117,532</td>
                                                                    
                                                                        <td class="r cle" title="2,193,977.72">2,193,978</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유형자산</th>
                                                                    
                                                                        <td class="r" title="1,499,285.39">1,499,285</td>
                                                                    
                                                                        <td class="r" title="1,496,180.19">1,496,180</td>
                                                                    
                                                                        <td class="r" title="1,542,545.76">1,542,546</td>
                                                                    
                                                                        <td class="r cle" title="1,603,435.68">1,603,436</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;무형자산</th>
                                                                    
                                                                        <td class="r" title="202,362.44">202,362</td>
                                                                    
                                                                        <td class="r" title="200,106.94">200,107</td>
                                                                    
                                                                        <td class="r" title="200,969.26">200,969</td>
                                                                    
                                                                        <td class="r cle" title="214,848.57">214,849</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;비유동생물자산</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;투자부동산</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기금융자산</th>
                                                                    
                                                                        <td class="r" title="154,911.83">154,912</td>
                                                                    
                                                                        <td class="r" title="146,976.80">146,977</td>
                                                                    
                                                                        <td class="r" title="129,927.85">129,928</td>
                                                                    
                                                                        <td class="r cle" title="127,871.00">127,871</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;관계기업등지분관련투자자산</th>
                                                                    
                                                                        <td class="r" title="89,322.51">89,323</td>
                                                                    
                                                                        <td class="r" title="88,416.05">88,416</td>
                                                                    
                                                                        <td class="r" title="103,829.38">103,829</td>
                                                                    
                                                                        <td class="r cle" title="108,527.74">108,528</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기매출채권및기타비유동채권</th>
                                                                    
                                                                        <td class="r" title="12,278.53">12,279</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이연법인세자산</th>
                                                                    
                                                                        <td class="r" title="42,612.14">42,612</td>
                                                                    
                                                                        <td class="r" title="41,789.84">41,790</td>
                                                                    
                                                                        <td class="r" title="51,143.32">51,143</td>
                                                                    
                                                                        <td class="r cle" title="54,881.63">54,882</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기당기법인세자산</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약자산</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)자산</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출권</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_3 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타비유동자산</th>
                                                                    
                                                                        <td class="r" title="83,806.89">83,807</td>
                                                                    
                                                                        <td class="r" title="96,108.95">96,109</td>
                                                                    
                                                                        <td class="r" title="89,116.02">89,116</td>
                                                                    
                                                                        <td class="r cle" title="84,413.10">84,413</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;기타금융업자산</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">부채</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="1,217,212.27">1,217,212</td>
                                                                    
                                                                        <td class="r" title="1,240,360.40">1,240,360</td>
                                                                    
                                                                        <td class="r" title="1,201,339.86">1,201,340</td>
                                                                    
                                                                        <td class="r cle" title="1,253,715.20">1,253,715</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid2Q_6" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;유동부채</span><a id="grid2Q_6" href="javascript:foldOpen('grid2Q_6');" class=" btn_acdopen"><span class="blind" id="span_grid2Q_6">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="881,171.33">881,171</td>
                                                                    
                                                                        <td class="r" title="904,637.01">904,637</td>
                                                                    
                                                                        <td class="r" title="833,622.68">833,623</td>
                                                                    
                                                                        <td class="r cle" title="852,856.69">852,857</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;단기사채</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;단기차입금</th>
                                                                    
                                                                        <td class="r" title="136,877.93">136,878</td>
                                                                    
                                                                        <td class="r" title="138,858.89">138,859</td>
                                                                    
                                                                        <td class="r" title="129,891.88">129,892</td>
                                                                    
                                                                        <td class="r cle" title="76,161.43">76,161</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동성장기부채</th>
                                                                    
                                                                        <td class="r" title="13,299.68">13,300</td>
                                                                    
                                                                        <td class="r" title="8,649.05">8,649</td>
                                                                    
                                                                        <td class="r" title="9,528.11">9,528</td>
                                                                    
                                                                        <td class="r cle" title="10,557.74">10,558</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동금융부채</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매입채무및기타유동채무</th>
                                                                    
                                                                        <td class="r" title="582,603.00">582,603</td>
                                                                    
                                                                        <td class="r" title="594,700.15">594,700</td>
                                                                    
                                                                        <td class="r" title="542,258.59">542,259</td>
                                                                    
                                                                        <td class="r cle" title="628,845.26">628,845</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동종업원급여충당부채</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타단기충당부채</th>
                                                                    
                                                                        <td class="r" title="53,728.72">53,729</td>
                                                                    
                                                                        <td class="r" title="59,108.08">59,108</td>
                                                                    
                                                                        <td class="r" title="59,957.90">59,958</td>
                                                                    
                                                                        <td class="r cle" title="59,659.63">59,660</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;당기법인세부채</th>
                                                                    
                                                                        <td class="r" title="67,491.49">67,491</td>
                                                                    
                                                                        <td class="r" title="71,937.54">71,938</td>
                                                                    
                                                                        <td class="r" title="60,676.53">60,677</td>
                                                                    
                                                                        <td class="r cle" title="42,806.32">42,806</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약부채</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)부채</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출부채</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타유동부채</th>
                                                                    
                                                                        <td class="r" title="27,170.51">27,171</td>
                                                                    
                                                                        <td class="r" title="31,383.30">31,383</td>
                                                                    
                                                                        <td class="r" title="31,309.67">31,310</td>
                                                                    
                                                                        <td class="r cle" title="34,826.31">34,826</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매각예정으로분류된처분자산집단에포함된부채</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid2Q_7" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;비유동부채</span><a id="grid2Q_7" href="javascript:foldOpen('grid2Q_7');" class=" btn_acdopen"><span class="blind" id="span_grid2Q_7">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="336,040.94">336,041</td>
                                                                    
                                                                        <td class="r" title="335,723.39">335,723</td>
                                                                    
                                                                        <td class="r" title="367,717.18">367,717</td>
                                                                    
                                                                        <td class="r cle" title="400,858.51">400,859</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;사채</th>
                                                                    
                                                                        <td class="r" title="5,082.32">5,082</td>
                                                                    
                                                                        <td class="r" title="5,188.56">5,189</td>
                                                                    
                                                                        <td class="r" title="5,537.83">5,538</td>
                                                                    
                                                                        <td class="r cle" title="6,141.40">6,141</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기차입금</th>
                                                                    
                                                                        <td class="r" title="15.00">15</td>
                                                                    
                                                                        <td class="r" title="25.00">25</td>
                                                                    
                                                                        <td class="r" title="85.00">85</td>
                                                                    
                                                                        <td class="r cle" title="85.00">85</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;비유동금융부채</th>
                                                                    
                                                                        <td class="r" title="28,646.56">28,647</td>
                                                                    
                                                                        <td class="r" title="27,782.07">27,782</td>
                                                                    
                                                                        <td class="r" title="29,347.08">29,347</td>
                                                                    
                                                                        <td class="r cle" title="31,674.48">31,674</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기매입채무및기타비유동채무</th>
                                                                    
                                                                        <td class="r" title="29,914.40">29,914</td>
                                                                    
                                                                        <td class="r" title="25,665.73">25,666</td>
                                                                    
                                                                        <td class="r" title="28,719.92">28,720</td>
                                                                    
                                                                        <td class="r cle" title="31,287.81">31,288</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;비유동종업원급여충당부채</th>
                                                                    
                                                                        <td class="r" title="4,658.84">4,659</td>
                                                                    
                                                                        <td class="r" title="4,769.03">4,769</td>
                                                                    
                                                                        <td class="r" title="5,171.43">5,171</td>
                                                                    
                                                                        <td class="r cle" title="5,757.45">5,757</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타장기충당부채</th>
                                                                    
                                                                        <td class="r" title="23,069.94">23,070</td>
                                                                    
                                                                        <td class="r" title="22,496.19">22,496</td>
                                                                    
                                                                        <td class="r" title="22,782.31">22,782</td>
                                                                    
                                                                        <td class="r cle" title="23,671.73">23,672</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이연법인세부채</th>
                                                                    
                                                                        <td class="r" title="231,982.05">231,982</td>
                                                                    
                                                                        <td class="r" title="237,053.74">237,054</td>
                                                                    
                                                                        <td class="r" title="263,413.82">263,414</td>
                                                                    
                                                                        <td class="r cle" title="288,962.79">288,963</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기당기법인세부채</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약부채</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)부채</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출부채</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_7 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타비유동부채</th>
                                                                    
                                                                        <td class="r" title="12,671.83">12,672</td>
                                                                    
                                                                        <td class="r" title="12,743.07">12,743</td>
                                                                    
                                                                        <td class="r" title="12,659.79">12,660</td>
                                                                    
                                                                        <td class="r cle" title="13,277.85">13,278</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;기타금융업부채</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">자본</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="3,048,999.31">3,048,999</td>
                                                                    
                                                                        <td class="r" title="3,152,909.19">3,152,909</td>
                                                                    
                                                                        <td class="r" title="3,279,066.64">3,279,067</td>
                                                                    
                                                                        <td class="r cle" title="3,449,068.89">3,449,069</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid2Q_10" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;지배기업주주지분</span><a id="grid2Q_10" href="javascript:foldOpen('grid2Q_10');" class=" btn_acdopen"><span class="blind" id="span_grid2Q_10">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="2,962,376.97">2,962,377</td>
                                                                    
                                                                        <td class="r" title="3,063,918.70">3,063,919</td>
                                                                    
                                                                        <td class="r" title="3,188,306.12">3,188,306</td>
                                                                    
                                                                        <td class="r cle" title="3,354,701.76">3,354,702</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자본금</th>
                                                                    
                                                                        <td class="r" title="8,975.14">8,975</td>
                                                                    
                                                                        <td class="r" title="8,975.14">8,975</td>
                                                                    
                                                                        <td class="r" title="8,975.14">8,975</td>
                                                                    
                                                                        <td class="r cle" title="8,975.14">8,975</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;신종자본증권</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자본잉여금</th>
                                                                    
                                                                        <td class="r" title="44,038.93">44,039</td>
                                                                    
                                                                        <td class="r" title="44,038.93">44,039</td>
                                                                    
                                                                        <td class="r" title="44,038.93">44,039</td>
                                                                    
                                                                        <td class="r cle" title="44,038.93">44,039</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타자본</th>
                                                                    
                                                                        <td class="r" title="867.63">868</td>
                                                                    
                                                                        <td class="r" title="931.05">931</td>
                                                                    
                                                                        <td class="r" title="946.72">947</td>
                                                                    
                                                                        <td class="r cle" title="876.49">876</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타포괄손익누계액</th>
                                                                    
                                                                        <td class="r" title="-22,152.36"><span class='tcr'>-22,152</span></td>
                                                                    
                                                                        <td class="r" title="-7,457.94"><span class='tcr'>-7,458</span></td>
                                                                    
                                                                        <td class="r" title="32,177.48">32,177</td>
                                                                    
                                                                        <td class="r cle" title="131,908.48">131,908</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid2Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이익잉여금(결손금)</th>
                                                                    
                                                                        <td class="r" title="2,930,647.63">2,930,648</td>
                                                                    
                                                                        <td class="r" title="3,017,431.52">3,017,432</td>
                                                                    
                                                                        <td class="r" title="3,102,167.85">3,102,168</td>
                                                                    
                                                                        <td class="r cle" title="3,168,902.72">3,168,903</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;비지배주주지분</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="86,622.34">86,622</td>
                                                                    
                                                                        <td class="r" title="88,990.49">88,990</td>
                                                                    
                                                                        <td class="r" title="90,760.52">90,761</td>
                                                                    
                                                                        <td class="r cle" title="94,367.13">94,367</td>
                                                                    
                                                            </tr>
                                                

                                            </tbody>
                                        </table>
                                    </div>
                                </div>
                            </div>
                            <!-- 현금흐름표 -->
                            <div class="ul_col2wrap pd_t25">
                                <div class="um_topbar bmshadow">
                                    <div class="topbar_wrap">
                                        <div class="topbar_lft">
                                            <h2>현금흐름표</h2>
        									<span class="stxt" id="btnCashYQText"></span>
                                        </div>
                                        <div class="topbar_rgt">
                                            <span class="txt1">단위 : 억원</span>
                                            <div class="um_gbtn grd_ty2 fl" id="btnCashYQ">
                                                <a class="gbtn_f r3" id="btnCash_Y" href="javascript:void(0);">연간</a>
                                                <a class="gbtn_e r3" id="btnCash_Q" href="javascript:void(0);">분기</a>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="ul_col2_l">
                                    <div class="ul_wrap ul_col_l">
                                        <div class="um_topbar botbd_n">
                                            <div class="topbar_wrap">
                                                <div class="topbar_lft">
                                                    <h2>Invested Capital</h2>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="um_chart chart_ht1">
                                            <div class="chart_wrap">
                                                <a class="chartdatapop" href="javascript:openGridPopup('cashChart1');" title="클릭:차트데이터 새창열림"><img id ="cashChart1"  alt="현금흐름표(Invested Capital): 자세한 내용은 클릭후 팝업창 참고" onerror="this.src='http://cdn.fnguide.com/SVO/Handbook_New/images/nodata.png'"/></a>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="ul_col2_r">
                                    <div class="ul_wrap ul_col_r">
                                        <div class="um_topbar botbd_n">
                                            <div class="topbar_wrap">
                                                <div class="topbar_lft">
                                                    <h2>Free Cash Flow</h2>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="um_chart chart_ht1">
                                            <div class="chart_wrap">
                                                <a class="chartdatapop" href="javascript:openGridPopup('cashChart2');" title="클릭:차트데이터 새창열림"><img id ="cashChart2"  alt="현금흐름표(Free Cash Flow): 자세한 내용은 클릭후 팝업창 참고" onerror="this.src='http://cdn.fnguide.com/SVO/Handbook_New/images/nodata.png'"/></a>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="ul_co1_c pd_t1">
                                    <!--현금흐름표 연간 데이타 테이블 시작-->                    
                                    <div class="um_table" id="divCashY" style="display:none;">
                                        
                                        <table  class="us_table_ty1 h_fix zigbg_no" >
                                            <caption class="cphidden">현금흐름표</caption>
                                            <colgroup>
                                                <col style="width: 40%;">
                                                    
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
                                                                          <div class="th_b">영업활동으로인한현금흐름</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="453,829.15">453,829</td>
                                                                    
                                                                        <td class="r" title="652,870.09">652,870</td>
                                                                    
                                                                        <td class="r" title="651,054.48">651,054</td>
                                                                    
                                                                        <td class="r cle" title="435,684.46">435,684</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;당기순손익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="217,388.65">217,389</td>
                                                                    
                                                                        <td class="r" title="264,078.32">264,078</td>
                                                                    
                                                                        <td class="r" title="399,074.50">399,075</td>
                                                                    
                                                                        <td class="r cle" title="318,126.31">318,126</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;법인세비용차감전계속사업이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3_4" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;현금유출이없는비용등가산</span><a id="grid3_4" href="javascript:foldOpen('grid3_4');" class=" btn_acdopen"><span class="blind" id="span_grid3_4">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="424,268.48">424,268</td>
                                                                    
                                                                        <td class="r" title="461,506.14">461,506</td>
                                                                    
                                                                        <td class="r" title="527,991.04">527,991</td>
                                                                    
                                                                        <td class="r cle" title="486,690.43">486,690</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;퇴직급여</th>
                                                                    
                                                                        <td class="r" title="11,716.06">11,716</td>
                                                                    
                                                                        <td class="r" title="12,901.79">12,902</td>
                                                                    
                                                                        <td class="r" title="13,603.44">13,603</td>
                                                                    
                                                                        <td class="r cle" title="10,402.41">10,402</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;종업원급여</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;주식보상비</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;대손상각비</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="400.06">400</td>
                                                                    
                                                                        <td class="r" title="179.90">180</td>
                                                                    
                                                                        <td class="r cle" title="397.55">398</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;감가상각비</th>
                                                                    
                                                                        <td class="r" title="265,738.16">265,738</td>
                                                                    
                                                                        <td class="r" title="271,157.35">271,157</td>
                                                                    
                                                                        <td class="r" title="312,852.09">312,852</td>
                                                                    
                                                                        <td class="r cle" title="271,089.61">271,090</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;무형자산상각비</th>
                                                                    
                                                                        <td class="r" title="30,238.22">30,238</td>
                                                                    
                                                                        <td class="r" title="32,198.81">32,199</td>
                                                                    
                                                                        <td class="r" title="29,621.52">29,622</td>
                                                                    
                                                                        <td class="r cle" title="23,496.40">23,496</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약부채전입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)부채전입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출부채전입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;충당부채전입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;외환손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타의대손상각비</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융원가</th>
                                                                    
                                                                        <td class="r" title="20,345.18">20,345</td>
                                                                    
                                                                        <td class="r" title="23,067.70">23,068</td>
                                                                    
                                                                        <td class="r" title="19,177.05">19,177</td>
                                                                    
                                                                        <td class="r cle" title="50,115.92">50,116</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자비용</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금지급</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매출채권처분손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산관련손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산처분손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산손상차손</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;파생상품손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융부채관련손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;사채상환손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;재고자산감모손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;재고자산폐기(처분)손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산처분(폐기)손실</th>
                                                                    
                                                                        <td class="r" title="1,445.47">1,445</td>
                                                                    
                                                                        <td class="r" title="876.73">877</td>
                                                                    
                                                                        <td class="r" title="755.86">756</td>
                                                                    
                                                                        <td class="r cle" title="347.98">348</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;투자자산평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산재평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산손상차손</th>
                                                                    
                                                                        <td class="r" title="7,852.15">7,852</td>
                                                                    
                                                                        <td class="r" title="10,007.63">10,008</td>
                                                                    
                                                                        <td class="r" title="17,357.41">17,357</td>
                                                                    
                                                                        <td class="r cle" title="33,361.20">33,361</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;지분법관련손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;종속기업관련손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;관계기업관련손익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세비용</th>
                                                                    
                                                                        <td class="r" title="86,933.24">86,933</td>
                                                                    
                                                                        <td class="r" title="99,372.85">99,373</td>
                                                                    
                                                                        <td class="r" title="134,443.77">134,444</td>
                                                                    
                                                                        <td class="r cle" title="95,729.25">95,729</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타비용</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="11,523.22">11,523</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="1,750.11">1,750</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3_5" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;(현금유입이없는수익등차감)</span><a id="grid3_5" href="javascript:foldOpen('grid3_5');" class=" btn_acdopen"><span class="blind" id="span_grid3_5">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="49,841.66">49,842</td>
                                                                    
                                                                        <td class="r" title="45,320.60">45,321</td>
                                                                    
                                                                        <td class="r" title="37,434.71">37,435</td>
                                                                    
                                                                        <td class="r cle" title="70,340.88">70,341</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;외환이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;대손충당금환입액</th>
                                                                    
                                                                        <td class="r" title="1,833.80">1,834</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융수익</th>
                                                                    
                                                                        <td class="r" title="38,314.28">38,314</td>
                                                                    
                                                                        <td class="r" title="37,188.41">37,188</td>
                                                                    
                                                                        <td class="r" title="24,856.79">24,857</td>
                                                                    
                                                                        <td class="r cle" title="57,200.68">57,201</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금수익</th>
                                                                    
                                                                        <td class="r" title="1,546.79">1,547</td>
                                                                    
                                                                        <td class="r" title="1,524.40">1,524</td>
                                                                    
                                                                        <td class="r" title="1,358.40">1,358</td>
                                                                    
                                                                        <td class="r cle" title="3,735.28">3,735</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매출채권처분이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산관련이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산처분이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산손상차손환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;파생상품이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융부채관련이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;사채상환이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;퇴직급여충당부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;충당부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;주식보상비환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;재고자산감모손실환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;재고자산폐기(처분)이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산처분(폐기)이익</th>
                                                                    
                                                                        <td class="r" title="3,524.36">3,524</td>
                                                                    
                                                                        <td class="r" title="1,542.49">1,542</td>
                                                                    
                                                                        <td class="r" title="3,404.00">3,404</td>
                                                                    
                                                                        <td class="r cle" title="1,359.40">1,359</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;투자자산평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산재평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산손상차손환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;지분법관련이익</th>
                                                                    
                                                                        <td class="r" title="4,129.60">4,130</td>
                                                                    
                                                                        <td class="r" title="5,065.30">5,065</td>
                                                                    
                                                                        <td class="r" title="7,296.14">7,296</td>
                                                                    
                                                                        <td class="r cle" title="8,045.52">8,046</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;종속회사관련이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;관계기업관련손익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타수익</th>
                                                                    
                                                                        <td class="r" title="492.83">493</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="519.38">519</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3_6" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;영업활동으로인한자산부채변동(운전자본변동)</span><a id="grid3_6" href="javascript:foldOpen('grid3_6');" class=" btn_acdopen"><span class="blind" id="span_grid3_6">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-25,457.56"><span class='tcr'>-25,458</span></td>
                                                                    
                                                                        <td class="r" title="1,224.24">1,224</td>
                                                                    
                                                                        <td class="r" title="-162,868.84"><span class='tcr'>-162,869</span></td>
                                                                    
                                                                        <td class="r cle" title="-218,390.75"><span class='tcr'>-218,391</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산의감소(증가)</th>
                                                                    
                                                                        <td class="r" title="38,981.37">38,981</td>
                                                                    
                                                                        <td class="r" title="-58,160.69"><span class='tcr'>-58,161</span></td>
                                                                    
                                                                        <td class="r" title="-174,243.02"><span class='tcr'>-174,243</span></td>
                                                                    
                                                                        <td class="r cle" title="-139,856.25"><span class='tcr'>-139,856</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;부채의증가(감소)</th>
                                                                    
                                                                        <td class="r" title="-57,967.40"><span class='tcr'>-57,967</span></td>
                                                                    
                                                                        <td class="r" title="76,163.60">76,164</td>
                                                                    
                                                                        <td class="r" title="45,247.80">45,248</td>
                                                                    
                                                                        <td class="r cle" title="-66,470.49"><span class='tcr'>-66,470</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;정부보조금등의변동</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타운전자본의변동</th>
                                                                    
                                                                        <td class="r" title="-6,471.53"><span class='tcr'>-6,472</span></td>
                                                                    
                                                                        <td class="r" title="-16,778.67"><span class='tcr'>-16,779</span></td>
                                                                    
                                                                        <td class="r" title="-33,873.62"><span class='tcr'>-33,874</span></td>
                                                                    
                                                                        <td class="r cle" title="-12,064.01"><span class='tcr'>-12,064</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;*영업에서창출된현금흐름</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="566,357.91">566,358</td>
                                                                    
                                                                        <td class="r" title="681,488.10">681,488</td>
                                                                    
                                                                        <td class="r" title="726,761.99">726,762</td>
                                                                    
                                                                        <td class="r cle" title="516,085.11">516,085</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3_8" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;기타영업활동으로인한현금흐름</span><a id="grid3_8" href="javascript:foldOpen('grid3_8');" class=" btn_acdopen"><span class="blind" id="span_grid3_8">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-112,528.76"><span class='tcr'>-112,529</span></td>
                                                                    
                                                                        <td class="r" title="-28,618.01"><span class='tcr'>-28,618</span></td>
                                                                    
                                                                        <td class="r" title="-75,707.51"><span class='tcr'>-75,708</span></td>
                                                                    
                                                                        <td class="r cle" title="-80,400.65"><span class='tcr'>-80,401</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자수입</th>
                                                                    
                                                                        <td class="r" title="23,064.01">23,064</td>
                                                                    
                                                                        <td class="r" title="22,202.09">22,202</td>
                                                                    
                                                                        <td class="r" title="14,067.06">14,067</td>
                                                                    
                                                                        <td class="r cle" title="12,295.43">12,295</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자지급(-)</th>
                                                                    
                                                                        <td class="r" title="-5,799.79"><span class='tcr'>-5,800</span></td>
                                                                    
                                                                        <td class="r" title="-5,553.21"><span class='tcr'>-5,553</span></td>
                                                                    
                                                                        <td class="r" title="-4,344.41"><span class='tcr'>-4,344</span></td>
                                                                    
                                                                        <td class="r cle" title="-4,810.29"><span class='tcr'>-4,810</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금수입</th>
                                                                    
                                                                        <td class="r" title="2,418.01">2,418</td>
                                                                    
                                                                        <td class="r" title="2,436.66">2,437</td>
                                                                    
                                                                        <td class="r" title="2,990.33">2,990</td>
                                                                    
                                                                        <td class="r cle" title="4,899.67">4,900</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금지급(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세납부(-)</th>
                                                                    
                                                                        <td class="r" title="-132,210.99"><span class='tcr'>-132,211</span></td>
                                                                    
                                                                        <td class="r" title="-47,703.55"><span class='tcr'>-47,704</span></td>
                                                                    
                                                                        <td class="r" title="-88,420.49"><span class='tcr'>-88,420</span></td>
                                                                    
                                                                        <td class="r cle" title="-92,785.46"><span class='tcr'>-92,785</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;중단영업관련현금흐름</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">투자활동으로인한현금흐름</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-399,481.71"><span class='tcr'>-399,482</span></td>
                                                                    
                                                                        <td class="r" title="-536,285.91"><span class='tcr'>-536,286</span></td>
                                                                    
                                                                        <td class="r" title="-330,477.63"><span class='tcr'>-330,478</span></td>
                                                                    
                                                                        <td class="r cle" title="-279,528.27"><span class='tcr'>-279,528</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3_10" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;투자활동으로인한현금유입액</span><a id="grid3_10" href="javascript:foldOpen('grid3_10');" class=" btn_acdopen"><span class="blind" id="span_grid3_10">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="63,007.75">63,008</td>
                                                                    
                                                                        <td class="r" title="155,516.79">155,517</td>
                                                                    
                                                                        <td class="r" title="254,825.06">254,825</td>
                                                                    
                                                                        <td class="r cle" title="134,745.10">134,745</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동금융자산의감소</th>
                                                                    
                                                                        <td class="r" title="11,354.62">11,355</td>
                                                                    
                                                                        <td class="r" title="29,836.07">29,836</td>
                                                                    
                                                                        <td class="r" title="142,179.22">142,179</td>
                                                                    
                                                                        <td class="r cle" title="63,508.80">63,509</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기금융상품의감소</th>
                                                                    
                                                                        <td class="r" title="45,866.10">45,866</td>
                                                                    
                                                                        <td class="r" title="121,843.01">121,843</td>
                                                                    
                                                                        <td class="r" title="102,160.82">102,161</td>
                                                                    
                                                                        <td class="r cle" title="68,964.13">68,964</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매도가능금융자산의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;만기보유금융자산의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기대여금의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;파생상품의변동</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타비유동금융자산의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;관계기업등지분관련투자자산의감소</th>
                                                                    
                                                                        <td class="r" title="121.49">121</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="191.69">192</td>
                                                                    
                                                                        <td class="r cle" title="132.33">132</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유형자산의감소</th>
                                                                    
                                                                        <td class="r" title="5,132.65">5,133</td>
                                                                    
                                                                        <td class="r" title="3,767.44">3,767</td>
                                                                    
                                                                        <td class="r" title="3,582.84">3,583</td>
                                                                    
                                                                        <td class="r cle" title="1,905.22">1,905</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;무형자산의감소</th>
                                                                    
                                                                        <td class="r" title="72.41">72</td>
                                                                    
                                                                        <td class="r" title="70.27">70</td>
                                                                    
                                                                        <td class="r" title="17.52">18</td>
                                                                    
                                                                        <td class="r cle" title="234.62">235</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;생물자산의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;투자부동산의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타투자활동으로인한현금유입액</th>
                                                                    
                                                                        <td class="r" title="460.48">460</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="6,692.97">6,693</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3_11" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;(투자활동으로인한현금유출액)</span><a id="grid3_11" href="javascript:foldOpen('grid3_11');" class=" btn_acdopen"><span class="blind" id="span_grid3_11">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="462,489.46">462,489</td>
                                                                    
                                                                        <td class="r" title="691,802.70">691,803</td>
                                                                    
                                                                        <td class="r" title="585,302.69">585,303</td>
                                                                    
                                                                        <td class="r cle" title="414,273.37">414,273</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동금융자산의증가</th>
                                                                    
                                                                        <td class="r" title="38,736.28">38,736</td>
                                                                    
                                                                        <td class="r" title="206,992.97">206,993</td>
                                                                    
                                                                        <td class="r" title="16,664.22">16,664</td>
                                                                    
                                                                        <td class="r cle" title="1,556.85">1,557</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기금융상품의증가</th>
                                                                    
                                                                        <td class="r" title="127,254.65">127,255</td>
                                                                    
                                                                        <td class="r" title="80,192.63">80,193</td>
                                                                    
                                                                        <td class="r" title="69,818.10">69,818</td>
                                                                    
                                                                        <td class="r cle" title="43,940.20">43,940</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매도가능금융자산의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;만기보유금융자산의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기대여금의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;파생상품의변동</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타비유동금융자산의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;관계기업등지분관련투자자산의증가</th>
                                                                    
                                                                        <td class="r" title="127.78">128</td>
                                                                    
                                                                        <td class="r" title="832.80">833</td>
                                                                    
                                                                        <td class="r" title="470.90">471</td>
                                                                    
                                                                        <td class="r cle" title="9,037.58">9,038</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유형자산의증가</th>
                                                                    
                                                                        <td class="r" title="253,677.56">253,678</td>
                                                                    
                                                                        <td class="r" title="375,920.34">375,920</td>
                                                                    
                                                                        <td class="r" title="471,221.06">471,221</td>
                                                                    
                                                                        <td class="r cle" title="324,713.28">324,713</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;무형자산의증가</th>
                                                                    
                                                                        <td class="r" title="32,499.14">32,499</td>
                                                                    
                                                                        <td class="r" title="26,797.79">26,798</td>
                                                                    
                                                                        <td class="r" title="27,069.15">27,069</td>
                                                                    
                                                                        <td class="r cle" title="31,894.59">31,895</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;생물자산의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;투자부동산의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타투자활동으로인한현금유출액</th>
                                                                    
                                                                        <td class="r" title="10,194.05">10,194</td>
                                                                    
                                                                        <td class="r" title="1,066.17">1,066</td>
                                                                    
                                                                        <td class="r" title="59.26">59</td>
                                                                    
                                                                        <td class="r cle" title="3,130.87">3,131</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3_12" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;기타투자활동으로인한현금흐름</span><a id="grid3_12" href="javascript:foldOpen('grid3_12');" class=" btn_acdopen"><span class="blind" id="span_grid3_12">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자수입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자지급(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금수입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금지급(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세납부(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;중단영업관련현금흐름</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">재무활동으로인한현금흐름</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-94,845.10"><span class='tcr'>-94,845</span></td>
                                                                    
                                                                        <td class="r" title="-83,278.39"><span class='tcr'>-83,278</span></td>
                                                                    
                                                                        <td class="r" title="-239,910.33"><span class='tcr'>-239,910</span></td>
                                                                    
                                                                        <td class="r cle" title="-146,847.55"><span class='tcr'>-146,848</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3_14" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;재무활동으로인한현금유입액</span><a id="grid3_14" href="javascript:foldOpen('grid3_14');" class=" btn_acdopen"><span class="blind" id="span_grid3_14">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="8,657.92">8,658</td>
                                                                    
                                                                        <td class="r" title="22,138.68">22,139</td>
                                                                    
                                                                        <td class="r" title="582.79">583</td>
                                                                    
                                                                        <td class="r cle" title="1,434.71">1,435</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;사채의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;차입금의증가</th>
                                                                    
                                                                        <td class="r" title="8,657.92">8,658</td>
                                                                    
                                                                        <td class="r" title="22,056.81">22,057</td>
                                                                    
                                                                        <td class="r" title="582.79">583</td>
                                                                    
                                                                        <td class="r cle" title="1,434.71">1,435</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;미지급금의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동성장기부채의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타금융부채의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타부채의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유상증자</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자기주식의처분</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;주식매입선택권의행사</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자본구성항목의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="81.87">82</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타재무활동으로인한현금유입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3_15" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;(재무활동으로인한현금유출액)</span><a id="grid3_15" href="javascript:foldOpen('grid3_15');" class=" btn_acdopen"><span class="blind" id="span_grid3_15">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="7,111.00">7,111</td>
                                                                    
                                                                        <td class="r" title="8,649.47">8,649</td>
                                                                    
                                                                        <td class="r" title="35,389.62">35,390</td>
                                                                    
                                                                        <td class="r cle" title="74,689.22">74,689</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;사채의감소</th>
                                                                    
                                                                        <td class="r" title="7,094.00">7,094</td>
                                                                    
                                                                        <td class="r" title="8,649.47">8,649</td>
                                                                    
                                                                        <td class="r" title="8,947.49">8,947</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;차입금의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="26,169.43">26,169</td>
                                                                    
                                                                        <td class="r cle" title="74,681.19">74,681</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;미지급금의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동성장기부채의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타금융부채의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타부채의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유상감자</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자기주식의취득</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자본구성항목의감소</th>
                                                                    
                                                                        <td class="r" title="17.00">17</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="272.70">273</td>
                                                                    
                                                                        <td class="r cle" title="8.03">8</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타재무활동으로인한현금유출액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3_16" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;기타재무활동으로인한현금흐름</span><a id="grid3_16" href="javascript:foldOpen('grid3_16');" class=" btn_acdopen"><span class="blind" id="span_grid3_16">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-96,392.02"><span class='tcr'>-96,392</span></td>
                                                                    
                                                                        <td class="r" title="-96,767.60"><span class='tcr'>-96,768</span></td>
                                                                    
                                                                        <td class="r" title="-205,103.50"><span class='tcr'>-205,104</span></td>
                                                                    
                                                                        <td class="r cle" title="-73,593.04"><span class='tcr'>-73,593</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자수입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자지급(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금수입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금지급(-)</th>
                                                                    
                                                                        <td class="r" title="-96,392.02"><span class='tcr'>-96,392</span></td>
                                                                    
                                                                        <td class="r" title="-96,767.60"><span class='tcr'>-96,768</span></td>
                                                                    
                                                                        <td class="r" title="-205,103.50"><span class='tcr'>-205,104</span></td>
                                                                    
                                                                        <td class="r cle" title="-73,593.04"><span class='tcr'>-73,593</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세납부(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;중단영업관련현금흐름</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">영업투자재무활동기타현금흐름</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="-1.39"><span class='tcr'>-1</span></td>
                                                                    
                                                                        <td class="r" title="1.39">1</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">연결범위변동으로인한현금의증가</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">환율변동효과</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="5,952.60">5,953</td>
                                                                    
                                                                        <td class="r" title="-8,338.61"><span class='tcr'>-8,339</span></td>
                                                                    
                                                                        <td class="r" title="15,820.46">15,820</td>
                                                                    
                                                                        <td class="r cle" title="45,531.42">45,531</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">현금및현금성자산의증가</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-34,545.06"><span class='tcr'>-34,545</span></td>
                                                                    
                                                                        <td class="r" title="24,965.79">24,966</td>
                                                                    
                                                                        <td class="r" title="96,488.37">96,488</td>
                                                                    
                                                                        <td class="r cle" title="54,840.06">54,840</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">기초현금및현금성자산</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="303,405.05">303,405</td>
                                                                    
                                                                        <td class="r" title="268,859.99">268,860</td>
                                                                    
                                                                        <td class="r" title="293,825.78">293,826</td>
                                                                    
                                                                        <td class="r cle" title="390,314.15">390,314</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">기말현금및현금성자산</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="268,859.99">268,860</td>
                                                                    
                                                                        <td class="r" title="293,825.78">293,826</td>
                                                                    
                                                                        <td class="r" title="390,314.15">390,314</td>
                                                                    
                                                                        <td class="r cle" title="445,154.21">445,154</td>
                                                                    
                                                            </tr>
                                                

                                            </tbody>
                                        </table>
                                    </div>
                                    <!--현금흐름표 분기 데이타 테이블 시작-->                    
                                    <div class="um_table" id="divCashQ" style="display:none;">
                                        
                                        <table  class="us_table_ty1 h_fix zigbg_no" >
                                            <caption class="cphidden">현금흐름표</caption>
                                            <colgroup>
                                                <col style="width: 40%;">
                                                    
                                                        <col >
                                                    
                                                        <col >
                                                    
                                                        <col >
                                                    
                                                        <col >
                                                    
                                            </colgroup>
                                            <thead>
                                                <tr>
                                                    <th scope="col" class="clf tbold">IFRS(연결)</th>
                                                    
                                                        <th scope="col">2021/12</th>
                                                    
                                                        <th scope="col">2022/03</th>
                                                    
                                                        <th scope="col">2022/06</th>
                                                    
                                                        <th scope="col">2022/09</th>
                                                    
                                                </tr>
                                            </thead>
                                            <tbody>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">영업활동으로인한현금흐름</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="206,344.61">206,345</td>
                                                                    
                                                                        <td class="r" title="104,530.69">104,531</td>
                                                                    
                                                                        <td class="r" title="141,360.66">141,361</td>
                                                                    
                                                                        <td class="r cle" title="189,793.11">189,793</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;당기순손익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="108,379.48">108,379</td>
                                                                    
                                                                        <td class="r" title="113,246.28">113,246</td>
                                                                    
                                                                        <td class="r" title="110,988.05">110,988</td>
                                                                    
                                                                        <td class="r cle" title="93,891.98">93,892</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;법인세비용차감전계속사업이익</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3Q_4" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;현금유출이없는비용등가산</span><a id="grid3Q_4" href="javascript:foldOpen('grid3Q_4');" class=" btn_acdopen"><span class="blind" id="span_grid3Q_4">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="140,617.98">140,618</td>
                                                                    
                                                                        <td class="r" title="162,029.70">162,030</td>
                                                                    
                                                                        <td class="r" title="158,247.48">158,247</td>
                                                                    
                                                                        <td class="r cle" title="166,413.25">166,413</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;퇴직급여</th>
                                                                    
                                                                        <td class="r" title="3,512.33">3,512</td>
                                                                    
                                                                        <td class="r" title="3,508.96">3,509</td>
                                                                    
                                                                        <td class="r" title="3,415.28">3,415</td>
                                                                    
                                                                        <td class="r cle" title="3,478.17">3,478</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;종업원급여</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;주식보상비</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;대손상각비</th>
                                                                    
                                                                        <td class="r" title="131.26">131</td>
                                                                    
                                                                        <td class="r" title="114.25">114</td>
                                                                    
                                                                        <td class="r" title="129.96">130</td>
                                                                    
                                                                        <td class="r cle" title="153.34">153</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;감가상각비</th>
                                                                    
                                                                        <td class="r" title="88,645.70">88,646</td>
                                                                    
                                                                        <td class="r" title="90,090.31">90,090</td>
                                                                    
                                                                        <td class="r" title="90,688.14">90,688</td>
                                                                    
                                                                        <td class="r cle" title="90,311.16">90,311</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;무형자산상각비</th>
                                                                    
                                                                        <td class="r" title="7,372.62">7,373</td>
                                                                    
                                                                        <td class="r" title="7,704.77">7,705</td>
                                                                    
                                                                        <td class="r" title="7,725.52">7,726</td>
                                                                    
                                                                        <td class="r cle" title="8,066.11">8,066</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약부채전입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)부채전입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출부채전입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;충당부채전입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;외환손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타의대손상각비</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융원가</th>
                                                                    
                                                                        <td class="r" title="932.00">932</td>
                                                                    
                                                                        <td class="r" title="14,628.63">14,629</td>
                                                                    
                                                                        <td class="r" title="10,532.70">10,533</td>
                                                                    
                                                                        <td class="r cle" title="24,954.59">24,955</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자비용</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금지급</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매출채권처분손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산관련손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산처분손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산손상차손</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;파생상품손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융부채관련손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;사채상환손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;재고자산감모손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;재고자산폐기(처분)손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산처분(폐기)손실</th>
                                                                    
                                                                        <td class="r" title="59.90">60</td>
                                                                    
                                                                        <td class="r" title="41.96">42</td>
                                                                    
                                                                        <td class="r" title="199.11">199</td>
                                                                    
                                                                        <td class="r cle" title="106.91">107</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;투자자산평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산재평가손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산손상차손</th>
                                                                    
                                                                        <td class="r" title="4,714.82">4,715</td>
                                                                    
                                                                        <td class="r" title="7,720.44">7,720</td>
                                                                    
                                                                        <td class="r" title="11,624.86">11,625</td>
                                                                    
                                                                        <td class="r cle" title="14,015.90">14,016</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;지분법관련손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;종속기업관련손실</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;관계기업관련손익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세비용</th>
                                                                    
                                                                        <td class="r" title="35,249.35">35,249</td>
                                                                    
                                                                        <td class="r" title="37,452.12">37,452</td>
                                                                    
                                                                        <td class="r" title="33,619.53">33,620</td>
                                                                    
                                                                        <td class="r cle" title="24,657.60">24,658</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_4 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타비용</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="768.26">768</td>
                                                                    
                                                                        <td class="r" title="312.38">312</td>
                                                                    
                                                                        <td class="r cle" title="669.47">669</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3Q_5" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;(현금유입이없는수익등차감)</span><a id="grid3Q_5" href="javascript:foldOpen('grid3Q_5');" class=" btn_acdopen"><span class="blind" id="span_grid3Q_5">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="3,106.97">3,107</td>
                                                                    
                                                                        <td class="r" title="22,972.40">22,972</td>
                                                                    
                                                                        <td class="r" title="16,726.80">16,727</td>
                                                                    
                                                                        <td class="r cle" title="30,641.68">30,642</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;외환이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;대손충당금환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융수익</th>
                                                                    
                                                                        <td class="r" title="2,770.19">2,770</td>
                                                                    
                                                                        <td class="r" title="17,458.34">17,458</td>
                                                                    
                                                                        <td class="r" title="12,872.71">12,873</td>
                                                                    
                                                                        <td class="r cle" title="26,869.63">26,870</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금수익</th>
                                                                    
                                                                        <td class="r" title="371.02">371</td>
                                                                    
                                                                        <td class="r" title="2,685.09">2,685</td>
                                                                    
                                                                        <td class="r" title="624.00">624</td>
                                                                    
                                                                        <td class="r cle" title="426.19">426</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매출채권처분이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;당기손익-공정가치측정&nbsp;금융자산관련이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산처분이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융자산손상차손환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;파생상품이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;금융부채관련이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;사채상환이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;퇴직급여충당부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;계약부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;반품(환불)부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배출부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;충당부채환입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;주식보상비환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;재고자산감모손실환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;재고자산폐기(처분)이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산처분(폐기)이익</th>
                                                                    
                                                                        <td class="r" title="207.80">208</td>
                                                                    
                                                                        <td class="r" title="504.20">504</td>
                                                                    
                                                                        <td class="r" title="637.80">638</td>
                                                                    
                                                                        <td class="r cle" title="217.40">217</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;투자자산평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산재평가이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산손상차손환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;지분법관련이익</th>
                                                                    
                                                                        <td class="r" title="1,288.62">1,289</td>
                                                                    
                                                                        <td class="r" title="2,324.77">2,325</td>
                                                                    
                                                                        <td class="r" title="2,592.29">2,592</td>
                                                                    
                                                                        <td class="r cle" title="3,128.46">3,128</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;종속회사관련이익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;관계기업관련손익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세수익</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_5 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타수익</th>
                                                                    
                                                                        <td class="r" title="-1,530.66"><span class='tcr'>-1,531</span></td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3Q_6" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;영업활동으로인한자산부채변동(운전자본변동)</span><a id="grid3Q_6" href="javascript:foldOpen('grid3Q_6');" class=" btn_acdopen"><span class="blind" id="span_grid3Q_6">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-25,766.81"><span class='tcr'>-25,767</span></td>
                                                                    
                                                                        <td class="r" title="-142,333.15"><span class='tcr'>-142,333</span></td>
                                                                    
                                                                        <td class="r" title="-51,938.85"><span class='tcr'>-51,939</span></td>
                                                                    
                                                                        <td class="r cle" title="-24,118.75"><span class='tcr'>-24,119</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자산의감소(증가)</th>
                                                                    
                                                                        <td class="r" title="-22,378.88"><span class='tcr'>-22,379</span></td>
                                                                    
                                                                        <td class="r" title="-98,647.69"><span class='tcr'>-98,648</span></td>
                                                                    
                                                                        <td class="r" title="-12,055.92"><span class='tcr'>-12,056</span></td>
                                                                    
                                                                        <td class="r cle" title="-29,152.64"><span class='tcr'>-29,153</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;부채의증가(감소)</th>
                                                                    
                                                                        <td class="r" title="15,897.00">15,897</td>
                                                                    
                                                                        <td class="r" title="-35,659.08"><span class='tcr'>-35,659</span></td>
                                                                    
                                                                        <td class="r" title="-42,149.28"><span class='tcr'>-42,149</span></td>
                                                                    
                                                                        <td class="r cle" title="11,337.87">11,338</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;정부보조금등의변동</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_6 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타운전자본의변동</th>
                                                                    
                                                                        <td class="r" title="-19,284.93"><span class='tcr'>-19,285</span></td>
                                                                    
                                                                        <td class="r" title="-8,026.38"><span class='tcr'>-8,026</span></td>
                                                                    
                                                                        <td class="r" title="2,266.35">2,266</td>
                                                                    
                                                                        <td class="r cle" title="-6,303.98"><span class='tcr'>-6,304</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">&nbsp;&nbsp;&nbsp;*영업에서창출된현금흐름</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="220,123.68">220,124</td>
                                                                    
                                                                        <td class="r" title="109,970.43">109,970</td>
                                                                    
                                                                        <td class="r" title="200,569.88">200,570</td>
                                                                    
                                                                        <td class="r cle" title="205,544.80">205,545</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3Q_8" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;기타영업활동으로인한현금흐름</span><a id="grid3Q_8" href="javascript:foldOpen('grid3Q_8');" class=" btn_acdopen"><span class="blind" id="span_grid3Q_8">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-13,779.07"><span class='tcr'>-13,779</span></td>
                                                                    
                                                                        <td class="r" title="-5,439.74"><span class='tcr'>-5,440</span></td>
                                                                    
                                                                        <td class="r" title="-59,209.22"><span class='tcr'>-59,209</span></td>
                                                                    
                                                                        <td class="r cle" title="-15,751.69"><span class='tcr'>-15,752</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자수입</th>
                                                                    
                                                                        <td class="r" title="3,675.90">3,676</td>
                                                                    
                                                                        <td class="r" title="3,340.26">3,340</td>
                                                                    
                                                                        <td class="r" title="3,727.29">3,727</td>
                                                                    
                                                                        <td class="r cle" title="5,227.88">5,228</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자지급(-)</th>
                                                                    
                                                                        <td class="r" title="-1,529.08"><span class='tcr'>-1,529</span></td>
                                                                    
                                                                        <td class="r" title="-1,590.77"><span class='tcr'>-1,591</span></td>
                                                                    
                                                                        <td class="r" title="-1,057.57"><span class='tcr'>-1,058</span></td>
                                                                    
                                                                        <td class="r cle" title="-2,161.95"><span class='tcr'>-2,162</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금수입</th>
                                                                    
                                                                        <td class="r" title="381.97">382</td>
                                                                    
                                                                        <td class="r" title="500.84">501</td>
                                                                    
                                                                        <td class="r" title="1,797.22">1,797</td>
                                                                    
                                                                        <td class="r cle" title="2,601.61">2,602</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금지급(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세납부(-)</th>
                                                                    
                                                                        <td class="r" title="-16,307.86"><span class='tcr'>-16,308</span></td>
                                                                    
                                                                        <td class="r" title="-7,690.07"><span class='tcr'>-7,690</span></td>
                                                                    
                                                                        <td class="r" title="-63,676.16"><span class='tcr'>-63,676</span></td>
                                                                    
                                                                        <td class="r cle" title="-21,419.23"><span class='tcr'>-21,419</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_8 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;중단영업관련현금흐름</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">투자활동으로인한현금흐름</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-103,513.21"><span class='tcr'>-103,513</span></td>
                                                                    
                                                                        <td class="r" title="-6,365.11"><span class='tcr'>-6,365</span></td>
                                                                    
                                                                        <td class="r" title="-192,927.56"><span class='tcr'>-192,928</span></td>
                                                                    
                                                                        <td class="r cle" title="-80,235.60"><span class='tcr'>-80,236</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3Q_10" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;투자활동으로인한현금유입액</span><a id="grid3Q_10" href="javascript:foldOpen('grid3Q_10');" class=" btn_acdopen"><span class="blind" id="span_grid3Q_10">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="42,353.14">42,353</td>
                                                                    
                                                                        <td class="r" title="119,965.14">119,965</td>
                                                                    
                                                                        <td class="r" title="-36,805.67"><span class='tcr'>-36,806</span></td>
                                                                    
                                                                        <td class="r cle" title="51,585.63">51,586</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동금융자산의감소</th>
                                                                    
                                                                        <td class="r" title="21,365.21">21,365</td>
                                                                    
                                                                        <td class="r" title="78,865.06">78,865</td>
                                                                    
                                                                        <td class="r" title="-50,462.88"><span class='tcr'>-50,463</span></td>
                                                                    
                                                                        <td class="r cle" title="35,106.62">35,107</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기금융상품의감소</th>
                                                                    
                                                                        <td class="r" title="20,729.03">20,729</td>
                                                                    
                                                                        <td class="r" title="40,358.52">40,359</td>
                                                                    
                                                                        <td class="r" title="13,080.69">13,081</td>
                                                                    
                                                                        <td class="r cle" title="15,524.92">15,525</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매도가능금융자산의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;만기보유금융자산의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기대여금의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;파생상품의변동</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타비유동금융자산의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;관계기업등지분관련투자자산의감소</th>
                                                                    
                                                                        <td class="r" title="182.53">183</td>
                                                                    
                                                                        <td class="r" title="53.00">53</td>
                                                                    
                                                                        <td class="r" title="16.33">16</td>
                                                                    
                                                                        <td class="r cle" title="63.00">63</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유형자산의감소</th>
                                                                    
                                                                        <td class="r" title="-4.92"><span class='tcr'>-5</span></td>
                                                                    
                                                                        <td class="r" title="687.30">687</td>
                                                                    
                                                                        <td class="r" title="371.09">371</td>
                                                                    
                                                                        <td class="r cle" title="846.83">847</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;무형자산의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="1.26">1</td>
                                                                    
                                                                        <td class="r" title="189.10">189</td>
                                                                    
                                                                        <td class="r cle" title="44.26">44</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;생물자산의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;투자부동산의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_10 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타투자활동으로인한현금유입액</th>
                                                                    
                                                                        <td class="r" title="81.29">81</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3Q_11" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;(투자활동으로인한현금유출액)</span><a id="grid3Q_11" href="javascript:foldOpen('grid3Q_11');" class=" btn_acdopen"><span class="blind" id="span_grid3Q_11">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="145,866.35">145,866</td>
                                                                    
                                                                        <td class="r" title="126,330.25">126,330</td>
                                                                    
                                                                        <td class="r" title="156,121.89">156,122</td>
                                                                    
                                                                        <td class="r cle" title="131,821.23">131,821</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동금융자산의증가</th>
                                                                    
                                                                        <td class="r" title="6,320.18">6,320</td>
                                                                    
                                                                        <td class="r" title="728.24">728</td>
                                                                    
                                                                        <td class="r" title="9,409.77">9,410</td>
                                                                    
                                                                        <td class="r cle" title="-8,581.16"><span class='tcr'>-8,581</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기금융상품의증가</th>
                                                                    
                                                                        <td class="r" title="11,039.64">11,040</td>
                                                                    
                                                                        <td class="r" title="28,433.88">28,434</td>
                                                                    
                                                                        <td class="r" title="14,835.00">14,835</td>
                                                                    
                                                                        <td class="r cle" title="671.32">671</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;매도가능금융자산의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;만기보유금융자산의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;장기대여금의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;파생상품의변동</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타비유동금융자산의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;관계기업등지분관련투자자산의증가</th>
                                                                    
                                                                        <td class="r" title="135.67">136</td>
                                                                    
                                                                        <td class="r" title="61.00">61</td>
                                                                    
                                                                        <td class="r" title="8,976.58">8,977</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유형자산의증가</th>
                                                                    
                                                                        <td class="r" title="123,726.40">123,726</td>
                                                                    
                                                                        <td class="r" title="87,068.39">87,068</td>
                                                                    
                                                                        <td class="r" title="114,509.36">114,509</td>
                                                                    
                                                                        <td class="r cle" title="123,135.53">123,136</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;무형자산의증가</th>
                                                                    
                                                                        <td class="r" title="4,928.67">4,929</td>
                                                                    
                                                                        <td class="r" title="8,534.43">8,534</td>
                                                                    
                                                                        <td class="r" title="7,229.18">7,229</td>
                                                                    
                                                                        <td class="r cle" title="16,130.98">16,131</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;생물자산의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;투자부동산의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_11 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타투자활동으로인한현금유출액</th>
                                                                    
                                                                        <td class="r" title="-284.21"><span class='tcr'>-284</span></td>
                                                                    
                                                                        <td class="r" title="1,504.31">1,504</td>
                                                                    
                                                                        <td class="r" title="1,162.00">1,162</td>
                                                                    
                                                                        <td class="r cle" title="464.56">465</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3Q_12" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;기타투자활동으로인한현금흐름</span><a id="grid3Q_12" href="javascript:foldOpen('grid3Q_12');" class=" btn_acdopen"><span class="blind" id="span_grid3Q_12">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자수입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자지급(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금수입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금지급(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세납부(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_12 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;중단영업관련현금흐름</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">재무활동으로인한현금흐름</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-38,003.31"><span class='tcr'>-38,003</span></td>
                                                                    
                                                                        <td class="r" title="-4,953.46"><span class='tcr'>-4,953</span></td>
                                                                    
                                                                        <td class="r" title="-59,622.97"><span class='tcr'>-59,623</span></td>
                                                                    
                                                                        <td class="r cle" title="-82,271.12"><span class='tcr'>-82,271</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3Q_14" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;재무활동으로인한현금유입액</span><a id="grid3Q_14" href="javascript:foldOpen('grid3Q_14');" class=" btn_acdopen"><span class="blind" id="span_grid3Q_14">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="239.55">240</td>
                                                                    
                                                                        <td class="r" title="2,140.34">2,140</td>
                                                                    
                                                                        <td class="r" title="-1,408.10"><span class='tcr'>-1,408</span></td>
                                                                    
                                                                        <td class="r cle" title="702.47">702</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;사채의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;차입금의증가</th>
                                                                    
                                                                        <td class="r" title="239.55">240</td>
                                                                    
                                                                        <td class="r" title="2,140.34">2,140</td>
                                                                    
                                                                        <td class="r" title="-1,408.10"><span class='tcr'>-1,408</span></td>
                                                                    
                                                                        <td class="r cle" title="702.47">702</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;미지급금의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동성장기부채의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타금융부채의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타부채의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유상증자</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자기주식의처분</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;주식매입선택권의행사</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자본구성항목의증가</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_14 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타재무활동으로인한현금유입액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3Q_15" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;(재무활동으로인한현금유출액)</span><a id="grid3Q_15" href="javascript:foldOpen('grid3Q_15');" class=" btn_acdopen"><span class="blind" id="span_grid3Q_15">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="13,670.06">13,670</td>
                                                                    
                                                                        <td class="r" title="7,089.19">7,089</td>
                                                                    
                                                                        <td class="r" title="9,154.30">9,154</td>
                                                                    
                                                                        <td class="r cle" title="58,445.73">58,446</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;사채의감소</th>
                                                                    
                                                                        <td class="r" title="8,947.49">8,947</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;차입금의감소</th>
                                                                    
                                                                        <td class="r" title="4,606.09">4,606</td>
                                                                    
                                                                        <td class="r" title="7,087.17">7,087</td>
                                                                    
                                                                        <td class="r" title="9,153.09">9,153</td>
                                                                    
                                                                        <td class="r cle" title="58,440.93">58,441</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;미지급금의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유동성장기부채의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타금융부채의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타부채의감소</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;유상감자</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자기주식의취득</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;자본구성항목의감소</th>
                                                                    
                                                                        <td class="r" title="116.48">116</td>
                                                                    
                                                                        <td class="r" title="2.02">2</td>
                                                                    
                                                                        <td class="r" title="1.21">1</td>
                                                                    
                                                                        <td class="r cle" title="4.80">5</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_15 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;기타재무활동으로인한현금유출액</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr id="p_grid3Q_16" class="rwf acd_dep_start_close ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class=""><span class="txt_acd">&nbsp;&nbsp;&nbsp;기타재무활동으로인한현금흐름</span><a id="grid3Q_16" href="javascript:foldOpen('grid3Q_16');" class=" btn_acdopen"><span class="blind" id="span_grid3Q_16">계산에 참여한 계정 펼치기</span></a></div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-24,572.80"><span class='tcr'>-24,573</span></td>
                                                                    
                                                                        <td class="r" title="-4.61"><span class='tcr'>-5</span></td>
                                                                    
                                                                        <td class="r" title="-49,060.57"><span class='tcr'>-49,061</span></td>
                                                                    
                                                                        <td class="r cle" title="-24,527.86"><span class='tcr'>-24,528</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자수입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;이자지급(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금수입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;배당금지급(-)</th>
                                                                    
                                                                        <td class="r" title="-24,572.80"><span class='tcr'>-24,573</span></td>
                                                                    
                                                                        <td class="r" title="-4.61"><span class='tcr'>-5</span></td>
                                                                    
                                                                        <td class="r" title="-49,060.57"><span class='tcr'>-49,061</span></td>
                                                                    
                                                                        <td class="r cle" title="-24,527.86"><span class='tcr'>-24,528</span></td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세환입</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;법인세납부(-)</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr  class="c_grid3Q_16 rwf acd_dep2_sub" style="display:none;">
                                                                
                                                                        <th scope="row" class="l clf">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;중단영업관련현금흐름</th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">영업투자재무활동기타현금흐름</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">연결범위변동으로인한현금의증가</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r" title="">&nbsp;</td>
                                                                    
                                                                        <td class="r cle" title="">&nbsp;</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">환율변동효과</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="-1,264.34"><span class='tcr'>-1,264</span></td>
                                                                    
                                                                        <td class="r" title="5,923.55">5,924</td>
                                                                    
                                                                        <td class="r" title="17,571.46">17,571</td>
                                                                    
                                                                        <td class="r cle" title="22,036.41">22,036</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf rowBold">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="th_b">현금및현금성자산의증가</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="63,563.75">63,564</td>
                                                                    
                                                                        <td class="r" title="99,135.67">99,136</td>
                                                                    
                                                                        <td class="r" title="-93,618.41"><span class='tcr'>-93,618</span></td>
                                                                    
                                                                        <td class="r cle" title="49,322.80">49,323</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">기초현금및현금성자산</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="326,750.40">326,750</td>
                                                                    
                                                                        <td class="r" title="390,314.15">390,314</td>
                                                                    
                                                                        <td class="r" title="489,449.82">489,450</td>
                                                                    
                                                                        <td class="r cle" title="395,831.41">395,831</td>
                                                                    
                                                            </tr>
                                                
                                                            <tr class="rwf ">
                                                                
                                                                        <th scope="row" class="l clf">
                                                                          <div class="">기말현금및현금성자산</div>
                                                                        </th>
                                                                    
                                                                        <td class="r" title="390,314.15">390,314</td>
                                                                    
                                                                        <td class="r" title="489,449.82">489,450</td>
                                                                    
                                                                        <td class="r" title="395,831.41">395,831</td>
                                                                    
                                                                        <td class="r cle" title="445,154.21">445,154</td>
                                                                    
                                                            </tr>
                                                

                                            </tbody>
                                        </table>
                                    </div>
                                    
                                </div>
                            </div>
                        
        				<div class="um_comment">
        					<a href="javascript:void(0)" onclick="cmView(this)" class="comment_wrap btn_cm cmhidden">
        						<ul>
        							<li>* 재무제표는 요약정보를 수록하고 있습니다.</li>
        							<li>* 자산총액 2조원 미만의 기업은 2013년까지 분기 및 반기 연결재무제표 의무 공시가 유예되므로, 해당 기업의 연결 기준 Net Quarter의 항목은 비어있는 것이 정상입니다. 
        (단, 재무상태표의 경우 4분기 Net Quarter는 공시되는 Annual과 동일함)</li>
        						</ul>
        						<span class="cmtext"></span>
        					</a>
        				</div>
                    </div>
                </div>
                <div class="bootomwrap">
                    <div class="ul_bottom">
                        <div class="ul_bottomwrap">
                            <a class="link_txt" href="http://www.fnguide.com" target="_blank"><span>fnguide.com</span></a>
                            <div class="txtbox">
                                <p class="txt">FnGuide에서 제공하는 정보는 신뢰할 만한 자료 및 정보로부터 얻어진 것이나 그 정확성이나 완전성을 보장 할 수 없으며, 시간이 경과함에 따라 변경될 수 있습니다. 따라서 정보의 오류, 누락에 대하여 FnGuide 또는FnGuide에 자료를 제공하는 회사에서는 그 결과에 대해 법적인 책임을 지지 않습니다. 모든 콘텐츠에 대한 저작권은 FnGuide에 있으며 사전 허가없이 이를 무단으로 사용하거나, 데이터 베이스화 할 경우 민형사상 책임을 물을 수 있습니다.</p>
                            </div>
                            <p class="waimg"><span>웹 접근성 우수사이트 인증마크(WA인증마크)</span></p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <input type="hidden" id="ddd" name="ddd"   value="005930"/>
        <input type="hidden" id="ReportGB" name="ReportGB"   value="D"/>
        <input type="hidden" name="stkGb"   value="701"/>
        <input type="hidden" id="sonkiYQ" name="sonkiYQ"    value=""/>
        <input type="hidden" id="daechaYQ" name="daechaYQ"   value=""/>
        <input type="hidden" id="cashYQ" name="cashYQ"   value=""/>
        </body>
        </html>
        "###);
    }
}
