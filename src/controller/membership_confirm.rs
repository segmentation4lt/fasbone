#![allow(non_snake_case)]
//-----------------------------------------------------------------------------------------------------------------------------------------
// actix_webは取り込むモジュールが異なるので各個呼び出し ※要検証
//-----------------------------------------------------------------------------------------------------------------------------------------
use actix_web::{HttpRequest, HttpResponse,ResponseError};
use thiserror::Error;

//-----------------------------------------------------------------------------------------------------------------------------------------
// COMMON モジュール(SEG4)
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::seg4_common;

//-----------------------------------------------------------------------------------------------------------------------------------------
// DB モジュール
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::db_base;

//-----------------------------------------------------------------------------------------------------------------------------------------
// JSON出力
//-----------------------------------------------------------------------------------------------------------------------------------------
use std::io::prelude::*;
use std::fs::{File,create_dir_all};


//-----------------------------------------------------------------------------------------------------------------------------------------
//  action_baseの読み込み
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::action_base;

//-----------------------------------------------------------------------------------------------------------------------------------------
//  business_logicの読み込み
//-----------------------------------------------------------------------------------------------------------------------------------------
pub use crate::business_logic::membership_confirm;

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
* //※GETの場合    <ハンドラ名>_struct:web::get<<ハンドラ名>Param>
* //※POSTの場合    <ハンドラ名>_struct:web::get<<ハンドラ名>Param>
*  最後はHashMapが作成されてそれを使用するので命名規則がカオスでもよい。
* 手動で書くのは大変なので、ツールで生成するようにする
*/
//-----------------------------------------------------------------------------------------------------------------------------------------
// execute 処理開始
//-----------------------------------------------------------------------------------------------------------------------------------------
pub async fn execute(
    actix_web::web::Path(confirm_uuid): actix_web::web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, MyError> {
    //-------------------------------------------------------------------------------------------------------------------------------------
    // ログイン必須ページかどうかをアクション別に設定。 true→誰でも見れる false→ログイン専用
    //-------------------------------------------------------------------------------------------------------------------------------------
    const GUEST_ACCESS_ALLOW: bool = true;

    //-------------------------------------------------------------------------------------------------------------------------------------
    // 多重登録禁止チェックかどうかをアクション別に設定。 true→必要 false→不要
    //-------------------------------------------------------------------------------------------------------------------------------------
    const UPDATE_EXISTS_ALLOW: bool = true;

    //-------------------------------------------------------------------------------------------------------------------------------------
    // コントローラの先頭でDBインスタンスを確立
    //-------------------------------------------------------------------------------------------------------------------------------------
    let mut pg_client = db_base::db_connect();

    //-------------------------------------------------------------------------------------------------------------------------------------
    // サーバ関連の初期化。
    //-------------------------------------------------------------------------------------------------------------------------------------
    let server_info = action_base::ServerInfomation::set_server_infomation(req,&mut pg_client);

    //-------------------------------------------------------------------------------------------------------------------------------------
    // 不正アクセスのチェック。post_token_idが空の場合は NotFoundを返却
    //-------------------------------------------------------------------------------------------------------------------------------------
    if server_info.post_token_id == "" {
        //エラーログ
        seg4_common::info!("● Post UUID is Noting. {}",serde_json::to_string(&server_info).unwrap());
        return Ok(HttpResponse::NotFound()
        .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
        .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
        .header("Set-Cookie", server_info.cookie_line)
        .body(seg4_common::NOTFOUND_ERROR))
    };

    //---------------------------------------------------------------------------------------------------------------------------------
    // ログイン中かどうかの判定(sessionはaction、ログインはbusinesslogicで管理)
    //---------------------------------------------------------------------------------------------------------------------------------
    if server_info.business_login_id < 1 && GUEST_ACCESS_ALLOW == false {
        if server_info.business_login_id == -1 {
            return Ok(HttpResponse::Unauthorized()
            .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
            .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
            .header("Set-Cookie", server_info.cookie_line)
            .body(seg4_common::NO_LOGIN_ERROR))
        } else {
            //エラーログ
            seg4_common::info!("● Login Check Failed. {}",serde_json::to_string(&server_info).unwrap());
            return Ok(HttpResponse::InternalServerError()
            .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
            .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
            .header("Set-Cookie", server_info.cookie_line)
            .body(seg4_common::FOTAL_ERROR))  
        };
    };

    //-------------------------------------------------------------------------------------------------------------------------------------
    //  フォームハンドラー関連の初期化
    //-------------------------------------------------------------------------------------------------------------------------------------
    //フォームハンドラーのデータクランプ
    let mut input_params = seg4_common::HashMap::new();
    //ヴァリテーションチェックの結果保持 (0:正常 5:バリテーションバック 9:ライズエクセプション)
    let mut input_result = seg4_common::HashMap::new();
    //ヴァリテーションバック詳細
    let mut valiback_detail = seg4_common::HashMap::new();

    //-------------------------------------------------------------------------------------------------------------------------------------
    // ステータスチェック 初期値 0→正常 5→バリバック 9→サーバエラー扱い
    //-------------------------------------------------------------------------------------------------------------------------------------
    input_result.insert(String::from("Result"), 0);

    /*   ここから非共通部   */
    //-------------------------------------------------------------------------------------------------------------------------------------
    // ハンドラをチェック関数を使って挿入する(_token) ※POSTでは必須
    //-------------------------------------------------------------------------------------------------------------------------------------
    input_params.insert(String::from(r"confirm_uuid"),action_base::InputParametars::set_input_parametars(
        true,confirm_uuid,true,"リンクの値が不正です。".to_string(),35,37,r"^[a-z0-9-]*$".to_string(),),
    );

    /*   非共通部 ココマデ  */

    //-------------------------------------------------------------------------------------------------------------------------------------
    // 入力チェック結果を集計
    //-------------------------------------------------------------------------------------------------------------------------------------
    for (key, value) in &input_params {
        if value.result == false {
            //詳細を追加
            valiback_detail.insert(key, &value.result_msg);
            //全体の戻り値を更新
            input_result.insert(String::from("Result"), 5);
        }
    }
    
    //-------------------------------------------------------------------------------------------------------------------------------------
    // ヴァリテーションがある場合は input_resultを組み立てて返却
    //-------------------------------------------------------------------------------------------------------------------------------------
    let &check_result = input_result.get(&String::from("Result")).unwrap();
    // ヴァリテーションの判定
    let json = 
    if check_result == 5 {
        //ヴァリテーションバックで終了
        format!("{{ \"result\": \"205 validation back\",\"valiback_detail\": {} }}", serde_json::to_string(&valiback_detail).unwrap())
    } else if server_info.query_string == "validation_only" {
        // QUERY_STRING指定でビジネスロジックを呼ばずに終了させる。
        String::from(seg4_common::VALIDATION_ALLOK)
    } else if UPDATE_EXISTS_ALLOW == true && server_info.is_exists_check== true {
        // 多重登録禁止チェック
        String::from(seg4_common::SAME_REQEST)    
    }else {
        //---------------------------------------------------------------------------------------------------------------------------------
        //ビジネスロジックの呼び出し
        //---------------------------------------------------------------------------------------------------------------------------------
        let business_logic = membership_confirm::BusinessLogic::execute(&server_info,&input_params,&mut pg_client);

        //---------------------------------------------------------------------------------------------------------------------------------
        // ビジネスロジック処理結果のチェック。NO LOGIN ERRORとFOTAL ERRORの2種類ある
        //---------------------------------------------------------------------------------------------------------------------------------
        if business_logic.result == 5 {
            //エラーログ
            //seg4_common::info!("● BusinessLogic Error. {}",serde_json::to_string(&business_logic).unwrap());
            return Ok(HttpResponse::Unauthorized()
            .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
            .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
            .header("Set-Cookie", server_info.cookie_line)
            .body(business_logic.data))
        } else if business_logic.result == 9 {
            //エラーログ
            seg4_common::info!(
                "● BusinessLogic Error. {} {}",serde_json::to_string(&server_info).unwrap(),serde_json::to_string(&input_params).unwrap()
            );
            return Ok(HttpResponse::InternalServerError()
            .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
            .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
            .header("Set-Cookie", server_info.cookie_line)
            .body(business_logic.data))  
        }; 

        //最終出力 ※JSONは一気に変換せず、struct毎にformatで連結する。
        business_logic.data
    };

    //-------------------------------------------------------------------------------------------------------------------------------------
    //  正常終了時の出力 パスがcgi-binだと画面出力 APIはJSON出力。リクエストはnginxにて制御済 
    //-------------------------------------------------------------------------------------------------------------------------------------
    // PATHの定義。アクセス対象ファイルのパス ファイルは body  head  read_module
    // Action名をF.A.C.Sにて動的に記載↓membership_confirm
    let template_path = String::from(
        format!("{}/{}/membership_confirm",seg4_common::define::PACKAGE_PATH,seg4_common::define::CGI_TEMPLATE_DIR)
    );
    if server_info.reqest_uri.contains("/json/api") == false && json.contains("result\":\"200") == true && server_info.reqest_method == "GET" {
        //使いまわすので変数化
        let cgi_output = format!("{}<script type=\"application/json\" id=\"laravel\">{}</script>{}{}{}{}{}{}{}</body></html>",
        seg4_common::define::TEMPLATE_READ_1,
        json,
        seg4_common::fs::read_to_string(format!("{}/head",template_path)).expect("FileLoading is Failed."),
        seg4_common::define::TEMPLATE_READ_2,
        seg4_common::fs::read_to_string(format!("{}/body",template_path)).expect("FileLoading is Failed."),
        seg4_common::define::TEMPLATE_READ_3,
"",// ### DYNAMIC HEAD ###
        seg4_common::define::TEMPLATE_READ_4,
        seg4_common::fs::read_to_string(format!("{}/read_module",template_path)).expect("FileLoading is Failed."));

        //-------------------------------------------------------------------------------------------------------------------------------------
        // SSGの出力
        //-------------------------------------------------------------------------------------------------------------------------------------
        if server_info.is_debug == true && server_info.user_agent.contains("curl") == true && server_info.query_string.contains("permanent") == true {
            let permanent_dir = &format!("{}/{}{}",seg4_common::define::PACKAGE_PATH,seg4_common::define::CGI_PERMANENT_DIR,server_info.reqest_uri);
            create_dir_all(permanent_dir).expect("mkdir[CGI_PERMANENT_DIR]  is Failed");
            let mut file = File::create(format!("{}/index.html",&permanent_dir)).expect("Index HTML File Create is Failed");
            file.write_all(cgi_output.as_bytes()).expect("CreatedCgiFile Output is Failed");
        }

        Ok(HttpResponse::Ok()
        .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE_HTML)
        .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
        .header("Set-Cookie", server_info.cookie_line)
        .body(cgi_output))
    } else {
        //-------------------------------------------------------------------------------------------------------------------------------------
        // 永続JSONの出力
        //-------------------------------------------------------------------------------------------------------------------------------------
        //user_agent 
        if server_info.is_debug == true && server_info.user_agent.contains("curl") == true && server_info.query_string.contains("permanent") == true {
            let permanent_dir = &format!("{}/{}{}",seg4_common::define::PACKAGE_PATH,seg4_common::define::CGI_PERMANENT_DIR,server_info.reqest_uri.replace("api","static"));
            create_dir_all(permanent_dir).expect("mkdir[JSON_PERMANENT_DIR]  is Failed");
            let mut file = File::create(format!("{}/index.json",&permanent_dir)).expect("Static JsonFile Create is Failed");
            file.write_all(json.as_bytes()).expect("CreatedJsonFile Output is Failed");
        }

        Ok(HttpResponse::Ok()
        .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
        .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
        .header("Set-Cookie", server_info.cookie_line)
        .body(json))
    }
} //execute 終端

