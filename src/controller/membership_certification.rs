/*
### PARAM ###←パラメータ名
### METHOD ###←メソッド(Query or Form)
※GETの場合    <ハンドラ名>_struct:web::Query<<ハンドラ名>Param>
※POSTの場合    <ハンドラ名>_struct:web::Form<<ハンドラ名>Param>

手順
①touch ./src/controller/membership_certification.rs
②vi ./src/controller/mod.rs
[pub mod membership_certification; //<コメント>]
③main.rs内を編集
[.route("/json/api/member/cert/",web::post().to(controller::membership_certification::execute))]
※GETの場合    get()
※POSTの場合   post()

curl -XPOST -d '_token=00000000-0000-0000-0000-000000000000' -d 'userid=test@localhost.localdomain' -d 'passwd=abcdefghijk' -b 'laravel_session=OE5wN2wqVEV3aUIrR1UwcYvTR5OQ7W55gtJkwbDvT6i1iyGd06m/LJoOAfSLx7mv+T8pZ78XW5WDUkrkIehvpg==' http://127.0.0.1/json/api/member/cert/


*/

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
//  action_baseの読み込み
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::action_base;

//-----------------------------------------------------------------------------------------------------------------------------------------
//  business_logicの読み込み
//-----------------------------------------------------------------------------------------------------------------------------------------
pub use crate::business_logic::membership_certification;

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

//-----------------------------------------------------------------------------------------------------------------------------------------
// 画面遷移別個別対応
//-----------------------------------------------------------------------------------------------------------------------------------------
#[derive(seg4_common::Serialize, seg4_common::Deserialize)]
pub struct PostParam {
    _token: String,
    userid: String,
    passwd: String,
} 

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
    postForm: actix_web::web::Form<PostParam>,
    req: HttpRequest,
) -> Result<HttpResponse, MyError> {
    //-------------------------------------------------------------------------------------------------------------------------------------
    // ログイン必須ページかどうかをアクション別に設定。 true→誰でも見れる false→ログイン専用
    //-------------------------------------------------------------------------------------------------------------------------------------
    const GUEST_ACCESS_ALLOW: bool = true;

    //-------------------------------------------------------------------------------------------------------------------------------------
    // 多重登録禁止チェックかどうかをアクション別に設定。 true→必要 false→不要
    //-------------------------------------------------------------------------------------------------------------------------------------
    const UPDATE_EXISTS_ALLOW: bool = false;
 
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
    // ハンドラをチェック関数を使って挿入する(userid)
    //-------------------------------------------------------------------------------------------------------------------------------------
    let userid = &postForm.userid;
    input_params.insert(String::from(r"userid"),action_base::InputParametars::set_input_parametars(
        true,userid.to_string(),true,"所定の書式にて入力して下さい".to_string(),2,-1,
        "^[A-Za-z0-9]{1}[A-Za-z0-9_.-]*@{1}[A-Za-z0-9_.-]+.[A-Za-z0-9]+$".to_string(),),
    );

    //-------------------------------------------------------------------------------------------------------------------------------------
    // ハンドラをチェック関数を使って挿入する(passwd)
    //-------------------------------------------------------------------------------------------------------------------------------------
    let passwd = &postForm.passwd;
    input_params.insert(String::from(r"passwd"),action_base::InputParametars::set_input_parametars(
        true,passwd.to_string(),true,"英数字及び一部の記号のみ使用可能".to_string(),7,-1,"^[A-Za-z0-9--_/*+.,!#$%&()~|]*$".to_string(),),
    );

    //-------------------------------------------------------------------------------------------------------------------------------------
    // ハンドラをチェック関数を使って挿入する(_token) ※POSTでは必須
    //-------------------------------------------------------------------------------------------------------------------------------------
    let _token = &postForm._token;
    if &server_info.post_token_id != _token && server_info.reqest_method == "POST" {
        valiback_detail.insert("_token", "トークンが一致しません。");
        input_result.insert(String::from("Result"), 5);
    };

    /*   非共通部ココマデ   */

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
        let business_logic = membership_certification::BusinessLogic::execute(&server_info,&input_params,&mut pg_client);

        //---------------------------------------------------------------------------------------------------------------------------------
        // ビジネスロジック処理結果のチェック。5は権限付属、9はシステムエラー、それ以外は正常終了。
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
            seg4_common::info!("● BusinessLogic Error. {} {}",serde_json::to_string(&server_info).unwrap(),serde_json::to_string(&input_params).unwrap());
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
    //  正常終了時のJSON出力
    //-------------------------------------------------------------------------------------------------------------------------------------
    Ok(HttpResponse::Ok()
    .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
    .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
    .header("Set-Cookie", server_info.cookie_line)
    .body(json))
} //execute 終端

