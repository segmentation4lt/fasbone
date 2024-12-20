/*
認証が通ったファイルを出力。

メンバーシップWEBVIEWの基幹パスはdefineに記載
呼び出し方は file_uploadを参照。


*/

#![allow(non_snake_case)] 
//-----------------------------------------------------------------------------------------------------------------------------------------
// actix_webは取り込むモジュールが異なるので各個呼び出し ※要検証
//-----------------------------------------------------------------------------------------------------------------------------------------
use actix_web::{HttpRequest, HttpResponse,ResponseError};
use thiserror::Error;

//-----------------------------------------------------------------------------------------------------------------------------------------
//  PAYLOAD
//-----------------------------------------------------------------------------------------------------------------------------------------
use std::io::prelude::*;
use std::fs::{File};

//-----------------------------------------------------------------------------------------------------------------------------------------
// COMMON モジュール(SEG4)
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::seg4_common;

//-----------------------------------------------------------------------------------------------------------------------------------------
// DB モジュール
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::db_base;

//-----------------------------------------------------------------------------------------------------------------------------------------
//  action_baseの読み込み
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::action_base;

//-----------------------------------------------------------------------------------------------------------------------------------------
// ResponseError のラッパー宣言。独自のエラー処理に使用
//-----------------------------------------------------------------------------------------------------------------------------------------
#[derive(Error, Debug)]
pub enum MyError {}
impl ResponseError for MyError {}

/**
*  引数での構造体宣言
* 空はString固定。GET、POSTごとにハンドラー名を分解する
*/
 /*   ここから非共通部   */
//※GETの場合 

/*   非共通部 ココマデ  */


/**
*  引数での構造体宣言
* //※GETの場合    <ハンドラ名>_struct:web::### METHOD ###<<ハンドラ名>Param>
* //※POSTの場合    <ハンドラ名>_struct:web::### METHOD ###<<ハンドラ名>Param>
*  最後はHashMapが作成されてそれを使用するので命名規則がカオスでもよい。
* 手動で書くのは大変なので、ツールで生成するようにする
*/
//-----------------------------------------------------------------------------------------------------------------------------------------
// execute 処理開始
//-----------------------------------------------------------------------------------------------------------------------------------------
pub async fn execute(
    path: actix_web::web::Path<(String, i64, String)>,
    req: HttpRequest,
) -> Result<HttpResponse, MyError> {

    //-------------------------------------------------------------------------------------------------------------------------------------
    // コントローラの先頭でDBインスタンスを確立
    //-------------------------------------------------------------------------------------------------------------------------------------
    let mut pg_client = db_base::db_connect();

    //-------------------------------------------------------------------------------------------------------------------------------------
    // パスを変数化する
    //-------------------------------------------------------------------------------------------------------------------------------------
    /*   ここから非共通部   */
    let (dir_name, count, file_name) = path.into_inner();
    /*   非共通部 ココマデ  */

    //-------------------------------------------------------------------------------------------------------------------------------------
    // サーバ関連の初期化。
    //-------------------------------------------------------------------------------------------------------------------------------------
    let server_info = action_base::ServerInfomation::set_server_infomation(req,&mut pg_client);
    
    //-------------------------------------------------------------------------------------------------------------------------------------
    // アクセス対象ファイルのフルパス
    //-------------------------------------------------------------------------------------------------------------------------------------
    let access_file = String::from(format!("{}/{}/{}/{}",seg4_common::define::MENBERSHIP_WEBVIEW_TBACKBONEDIR,dir_name, count, file_name));

    //-------------------------------------------------------------------------------------------------------------------------------------
    // ファイル拡張子を算出
    //-------------------------------------------------------------------------------------------------------------------------------------    
    let file_extens = match file_name.split(".").collect::<Vec<_>>()[1].parse::<String>() {
        Ok(value) => value,
        Err(_error) => {
            //エラーログ
            seg4_common::info!("●No File Extents. {}",serde_json::to_string(&server_info).unwrap());
            // ファイルを開くのが失敗
            return Ok(HttpResponse::Forbidden()
                .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
                .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
                .header("Set-Cookie", server_info.cookie_line)
                .body(seg4_common::PARAM_ERROR)
            );  
        },     
    };

    //-------------------------------------------------------------------------------------------------------------------------------------
    // 不正アクセスのチェック。未ログイン場合は表示しない。
    //-------------------------------------------------------------------------------------------------------------------------------------
    if server_info.post_token_id == "" || server_info.business_login_id < 1 {
        return Ok(HttpResponse::Forbidden()
        .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
        .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
        .header("Set-Cookie", server_info.cookie_line)
        .body(seg4_common::FORBIDDEN_ERROR))
    };

    //-------------------------------------------------------------------------------------------------------------------------------------
    // ファイル処理。未存在ファイルの場合は処理を続行してPARAM_ERROR
    //-------------------------------------------------------------------------------------------------------------------------------------
    let f = File::open(access_file);
    let mut open_file = match f {
        Ok(file) => file,
        Err(_error) => {
            //エラーログ
            seg4_common::info!("●File Open Failed. {}",serde_json::to_string(&server_info).unwrap());
            // ファイルを開くのが失敗
            return Ok(HttpResponse::Forbidden()
                .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
                .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
                .header("Set-Cookie", server_info.cookie_line)
                .body(seg4_common::PARAM_ERROR)
            );
        },
    };
    let mut buf = Vec::new();
    let _ = open_file.read_to_end(&mut buf).expect("FileLoading is Failed.");

    //-------------------------------------------------------------------------------------------------------------------------------------
    //  正常終了時のJSON出力
    //-------------------------------------------------------------------------------------------------------------------------------------
    Ok(HttpResponse::Ok()
    .header("Content-Type", seg4_common::extnsis_to_contenttype(&file_extens))
    .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
    .header("Set-Cookie", server_info.cookie_line)
    .body(buf))
} //execute 終端
