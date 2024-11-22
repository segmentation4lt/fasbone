//-----------------------------------------------------------------------------------------------------------------------------------------
// actix_webは取り込むモジュールが異なるので各個呼び出し ※要検証
//-----------------------------------------------------------------------------------------------------------------------------------------
use actix_web::{HttpResponse,ResponseError};


//--------------------------------------------------------------------------
// usr dependencies
//--------------------------------------------------------------------------
use log::info;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
//use std::collections::HashMap;

//--------------------------------------------------------------------------
// ResponseError のラッパー宣言。独自のエラー処理に使用
//--------------------------------------------------------------------------
#[derive(Error, Debug)]
pub enum MyError {}
impl ResponseError for MyError {}

//-----------------------------------------------------------------------------------------------------------------------------------------
// COMMON モジュール(SEG4)
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::seg4_common;

//テスト
use std::fs;

//--------------------------------------------------------------------------
// 画面遷移別個別対応
//--------------------------------------------------------------------------
/**
 *  構造体の命名規則：<ハンドラ名>Param ※ハンドラ名はsnake case
 *  ！！ 型指定は画面遷移を避ける目的で、Stringで統一。
 *
 */
#[derive(Serialize, Deserialize)]
pub struct UsernameParam {
    username: String,
}
#[derive(Serialize, Deserialize)]
pub struct PasswdParam {
    passwd: String,
}

// MyError は actix_web::ResponseError を実装しているので、
// index の戻り値に MyError を使うことが出来ます。
pub async fn execute(
    actix_web::web::Path(_user_id): actix_web::web::Path<String>,
) -> Result<HttpResponse, MyError> {
    info!("user_id is {}", _user_id);
    let response_body = "{\"lists\":[\"りhんご\",\"ごりら\", \"ラッパ\",\"パンツ\", \"つみき\"]}";
//cgi-bin開発動作確認
//ret_user_agent.contains("Mobi");
//response_body
/*
cache-control:
public, max-age=10800
cache-control:
no-cache,no-store
content-encoding:
gzip
content-type:
text/html; charset=UTF-8

*/



//-----------------------------------------------------------------------------------------------------------------------------------------
// cgiテンプレートファイル設置フォルダ※パッケージ基幹パスと連結
//-----------------------------------------------------------------------------------------------------------------------------------------
let ret_reqest_uri: String = "/cgi-bin/req_test/666".to_string();

//-----------------------------------------------------------------------------------------------------------------------------------------
// アクセス対象ファイルのパス ファイルは body  head  read_module
//-----------------------------------------------------------------------------------------------------------------------------------------
let template_path = String::from(
    format!("{}/{}/",seg4_common::define::PACKAGE_PATH,seg4_common::define::CGI_TEMPLATE_DIR)
);




//アクション名はF.A.C.Sにて動的に記載
//let cgi_head = fs::read_to_string(format!("{}/req_test/head",template_path)).expect("FileLoading is Failed.");
//let cgi_body = fs::read_to_string(format!("{}/req_test/body",template_path)).expect("FileLoading is Failed.");
//let cgi_read_module = fs::read_to_string(format!("{}/req_test/read_module",template_path)).expect("FileLoading is Failed.");
//head読み込み前 TEMPLATE_READ_1
//head読み込み後～body読み込み前 TEMPLATE_READ_2
//body読み込み後～bodyCompiledまでTEMPLATE_READ_3
//※bodyCompiled内の引数はapiのbody出力～→);};</script></body></html>
//</script>の前にread_moduleを読む


//format!("{}{}{}{}\);};</script>{}</body></html>",seg4_common::define::TEMPLATE_READ_1,cgi_head,seg4_common::define::TEMPLATE_READ_2,cgi_body,seg4_common::define::TEMPLATE_READ_3,response_body,cgi_read_module);




if ret_reqest_uri.contains("cgi-bin") == true {
    //CGIでの出力
    //Ok(HttpResponse::Ok()
    //.header("Content-Type", seg4_common::HTTP_CONTENT_TYPE_HTML)
    //.header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
    //.header("Set-Cookie", server_info.cookie_line)
    //.body("CGI-BIN"))
    Ok(HttpResponse::Ok().body(format!("{}<script type=\"application/json\" id=\"laravel\">{}</script>{}{}{}{}{}</body></html>",
    seg4_common::define::TEMPLATE_READ_1,
    response_body,   
    fs::read_to_string(format!("{}/req_test/head",template_path)).expect("FileLoading is Failed."),
    seg4_common::define::TEMPLATE_READ_2,
    fs::read_to_string(format!("{}/req_test/body",template_path)).expect("FileLoading is Failed."),
    seg4_common::define::TEMPLATE_READ_3,
    fs::read_to_string(format!("{}/req_test/read_module",template_path)).expect("FileLoading is Failed."))))
} else {
    // Ok(HttpResponse::InternalServerError().finish()) //能動的なエラー返却 500
    //Ok(HttpResponse::Unauthorized().finish())//能動的なエラー返却 401
    Ok(HttpResponse::Ok().body(response_body))
}


}
