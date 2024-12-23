//-----------------------------------------------------------------------------------------------------------------------------------------
// 定数関連
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::resorce_module::define;
use actix_web::http::header::{
    REFERER,
    USER_AGENT,
    WWW_AUTHENTICATE,
    CONTENT_LENGTH,
    CONTENT_TYPE,
};

//-----------------------------------------------------------------------------------------------------------------------------------------
// Regix関連
//-----------------------------------------------------------------------------------------------------------------------------------------
extern crate regex;
use regex::Regex;
use std::collections::HashMap;

//-----------------------------------------------------------------------------------------------------------------------------------------
// COMMON モジュール(SEG4)
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::seg4_common;

//-----------------------------------------------------------------------------------------------------------------------------------------
// DB モジュール
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::db_base;

//-----------------------------------------------------------------------------------------------------------------------------------------
// 構造体:ServerInfomation
//-----------------------------------------------------------------------------------------------------------------------------------------
#[derive(seg4_common::Serialize, seg4_common::Deserialize)]
pub struct ServerInfomation{
    pub reqest_method:String,//メソッド(GETでセッション発行 POSTでセッションチェック)
    pub user_agent:String,  // ユーザーエージェント
    pub http_referer:String, //リファラー
    pub realip_remote_addr:String, //ホスト
    pub http_content_length:String, //ファイル長(標準入力)
    pub http_content_type:String, //タイプ(multipartかどうが判別する材料)
    pub reqest_uri:String, //URI
    pub query_string:String, //QUERY_STRING
    pub is_mobile: bool, //ユーザーエージェントがmobileならtrue
    pub is_exists_check: bool, //前回リクエストが同一且連続的アクセスの場合はtrue
    pub http_authenticate:String, //BASIN認証のやつ
    pub http_x_remote_addr:String, //アクセス元IP
    pub http_x_forwarded_for:String, //アクセス元IP
    pub post_token_id:String, //払い出しで返す _tokenのuuid 普段はDB内で管理
    pub is_debug:bool,//trueならデバッグモード falseなら本番モード
    pub cookie_line:String,//クッキーに書き込むライン
    pub last_access:String,//最終アクセス時刻
    pub timestamp:i64,//last_accessのタイムスタンプ
    pub business_login_id:i32,//BusinessLogicの共通処理の戻り値
}

//-----------------------------------------------------------------------------------------------------------------------------------------
// 構造体:InputParametars
//-----------------------------------------------------------------------------------------------------------------------------------------
#[derive(seg4_common::Serialize, seg4_common::Deserialize)]
pub struct InputParametars {
    pub string_type: bool,  // 文字列で扱うなら true それ以外なら false
    pub str_value: String,  // 文字列でのパンドラ値
    pub str_length: i64,    // 文字列長
    pub int_value: i64,     //整数でのパンドラ値
    pub float_value: f64,   // floatでのパンドラ値
    pub result: bool,       // 値チェックの結果 OK なら true NG なら false
    pub result_msg: String, // ValidationBack時のメッセージ
}

//-----------------------------------------------------------------------------------------------------------------------------------------
// トレイト (構造体:ServerInfomation)
//-----------------------------------------------------------------------------------------------------------------------------------------
impl ServerInfomation {
    //-------------------------------------------------------------------------------------------------------------------------------------
    // 投入SQLは定数で設定
    //-------------------------------------------------------------------------------------------------------------------------------------
    // 多重登録禁止チェック用のSQL
    const UPDATE_EXISTS_CHECK: &str = "select exists (select plimary from seg4planet_session_managements 
        where uuid=$1 and reqest_uri=$2 and last_update > cast(extract(epoch from now()) as integer) - $3);
    ";
    //session新規追加
    const INSERT_SESSION_MANAGEMENTS: &str = "insert into seg4planet_session_managements
        (uuid, http_referer, user_agent, realip_remote_addr, reqest_uri, last_update) values 
        ($1, $2, $3, $4, $5, $6);
    ";
    //UUID取得&最終更新時刻更新
    const SELECT_BY_UUID: &str = "update seg4planet_session_managements set last_update=$5, reqest_uri=$6 where 
        uuid= $1 and 
        user_agent= $2 and 
        realip_remote_addr=$3 and 
        http_referer like $4 returning auth_id;
    ";
    //時間が経過したsessionを削除
    const TIME_OVER_SESSION_DELETE: &str = "delete from seg4planet_session_managements where  uuid <> '00000000-0000-0000-0000-000000000000' and last_update < $1";

    //-------------------------------------------------------------------------------------------------------------------------------------
    // * トレイト内関数:set_server_infomation
    // * 構造体ServerInfomation として値を代入する。
    //-------------------------------------------------------------------------------------------------------------------------------------
    pub fn set_server_infomation(
        req: actix_web::HttpRequest,
        pg_client: &mut db_base::postgres::Client,
        //----- 戻り値 -----//
    ) -> ServerInfomation {
        //---------------------------------------------------------------------------------------------------------------------------------
        // REFERER
        //---------------------------------------------------------------------------------------------------------------------------------
        let str_http_referer: &str= match req.headers().get(&REFERER) {
            Some(value) => value.to_str().unwrap(),
            None => ""
        };
        let ret_http_referer = str_http_referer.to_string();

        //---------------------------------------------------------------------------------------------------------------------------------
        // USER_AGENT
        //---------------------------------------------------------------------------------------------------------------------------------
        let str_user_agent: &str= match req.headers().get(&USER_AGENT) {
            Some(value) => value.to_str().unwrap(),
            None => ""
        };
        let ret_user_agent = str_user_agent.to_string(); 

        //---------------------------------------------------------------------------------------------------------------------------------
        // REALIP_REMOTE_ADDR
        //---------------------------------------------------------------------------------------------------------------------------------
        let ret_realip_remote_addr: String= match req.connection_info().realip_remote_addr() {
            Some(value) => value.to_string(),
            None => "".to_string()
        };
        //let ret_realip_remote_addr = str_realip_remote_addr.to_string();

        //---------------------------------------------------------------------------------------------------------------------------------
        // CONTENT_LENGTH
        //---------------------------------------------------------------------------------------------------------------------------------
        let str_http_content_length: &str= match req.headers().get(&CONTENT_LENGTH) {
            Some(value) => value.to_str().unwrap(),
            None => ""
        };
        let ret_http_content_length = str_http_content_length.to_string();

        //---------------------------------------------------------------------------------------------------------------------------------
        // CONTENT_TYPE
        //---------------------------------------------------------------------------------------------------------------------------------
        let str_http_content_type: &str= match req.headers().get(&CONTENT_TYPE) {
            Some(value) => value.to_str().unwrap(),
            None => ""
        };
        let ret_http_content_type = str_http_content_type.to_string();
    
        //---------------------------------------------------------------------------------------------------------------------------------
        // URIの取得 1行で処理できない。
        //---------------------------------------------------------------------------------------------------------------------------------
        let binding = req.uri().to_string();
        let uri_array: Vec<&str> = binding.split('?').collect();
        let mut ret_reqest_uri: String  = uri_array[0].to_string();
        //末尾がスラッシュ(/)なら除外
        if ret_reqest_uri.chars().last().expect("REASON").to_string() == String::from("/") {
            ret_reqest_uri.pop();
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        // query_string
        //---------------------------------------------------------------------------------------------------------------------------------
        let ret_query_string = req.query_string().to_string();

        //---------------------------------------------------------------------------------------------------------------------------------
        // メソッド
        //---------------------------------------------------------------------------------------------------------------------------------
        let ret_reqest_method = req.method().to_string();

        //---------------------------------------------------------------------------------------------------------------------------------
        // モバイルかどうかを判定
        //---------------------------------------------------------------------------------------------------------------------------------
        let ret_is_mobile = ret_user_agent.contains("Mobi");

        //---------------------------------------------------------------------------------------------------------------------------------
        // WWW_AUTHENTICATE
        //---------------------------------------------------------------------------------------------------------------------------------
        let str_http_authenticate: &str= match req.headers().get(&WWW_AUTHENTICATE) {
            Some(value) => value.to_str().unwrap(),
            None => ""
        };
        let ret_http_authenticate = str_http_authenticate.to_string();

        //---------------------------------------------------------------------------------------------------------------------------------
        //足りないものだけ取得
        //---------------------------------------------------------------------------------------------------------------------------------
        let mut ret_header_result = HashMap::new();
        for header in req.headers().into_iter() {
            ret_header_result.insert(header.0.to_string(), header.1.to_str().unwrap());
        }

        //---------------------------------------------------------------------------------------------------------------------------------
        // x-remote-addr
        //---------------------------------------------------------------------------------------------------------------------------------
        let str_http_x_remote_addr = match ret_header_result.get(&String::from("x-remote-addr")) {
            Some(value) => value.to_string(),
            None => String::from("")
        };
        let ret_http_x_remote_addr = str_http_x_remote_addr.to_string();

        //---------------------------------------------------------------------------------------------------------------------------------
        // x-forwarded-for (プロクシチェック。串付きなら処理中断)
        //---------------------------------------------------------------------------------------------------------------------------------
        let str_http_x_forwarded_for = match ret_header_result.get(&String::from("x-forwarded-for")) {
            Some(value) => value.to_string(),
            None => String::from("")
        };
        let ret_http_x_forwarded_for = str_http_x_forwarded_for.to_string();

        //---------------------------------------------------------------------------------------------------------------------------------
        // ログの定義がdebugならデバックモード
        //---------------------------------------------------------------------------------------------------------------------------------
        let ret_is_debug:bool = match seg4_common::define::RUST_LOG {
            "Debug" => true,
            "Trace" => true,
            _ => false,
        };
       
        //---------------------------------------------------------------------------------------------------------------------------------
        // 最終アクセス時刻
        //---------------------------------------------------------------------------------------------------------------------------------
        let dt:  seg4_common::DateTime< seg4_common::Local> = seg4_common::Local::now();
        let ret_timestamp: i64 = dt.timestamp();
        let ret_last_access = seg4_common::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        //---------------------------------------------------------------------------------------------------------------------------------
        // _tolem(post_token_id)の取得 フォームハンドラーから取得した値と比較して整合性を確認
        //---------------------------------------------------------------------------------------------------------------------------------
        let str_getcookie: &str= match req.headers().get("cookie") {
            Some(value) => value.to_str().unwrap(),
            None => ""
        };
   
        //---------------------------------------------------------------------------------------------------------------------------------
        // Cookieからlaravel_sessionを取り出す ※Cookieはブラウザの挙動にてGET用,POST用の順で横並びに展開されているのをGET用に統一する
        // 2024.10.19 cokkkieの値がブラウザによって汚染されているのでサイニタイズ処置を追加
        //---------------------------------------------------------------------------------------------------------------------------------
        //本来の受信cookie※汚染済
        let getcoolie_split_ary_zero: Vec<&str> = str_getcookie.split(";").collect();
        //サニタイズ用配列を生成
        let mut getcoolie_upd: Vec<&str> =[].to_vec();
        //laravel_session該当項目を配列に抜き出す
        for name1 in &getcoolie_split_ary_zero {
            if name1.contains("laravel_session") == true {
                getcoolie_upd.push(name1);
            }
        }
        //残りの並びを要確認※ブラウザ依存
        let getcoolie_vrs_0 : String = if getcoolie_upd.len() > 0 {
            getcoolie_upd[0].replacen("laravel_session=", "laravel_session:", 1).replace(" ", "")
        } else{
            String::from("laravel_session:")
        };      
        let getcoolie_vrs_1 : String = if getcoolie_upd.len() > 1 {
            getcoolie_upd[1].replacen("laravel_session=", "laravel_session:", 1).replace(" ", "")
        } else{
            String::from("laravel_session:")
        };
        let getcoolie_split_ary_1: Vec<&str> = getcoolie_vrs_1.split(":").collect();
        let getcoolie_split_ary: Vec<&str> = if getcoolie_vrs_0 == "laravel_session:" && getcoolie_split_ary_1.len() > 1 {
            getcoolie_vrs_1.split(":").collect()
        } else{
            getcoolie_vrs_0.split(":").collect()
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        // 時間経過したsessionを削除
        //---------------------------------------------------------------------------------------------------------------------------------
        let until_time : i64 = ret_timestamp - define::MAX_AGE;
        pg_client.execute(
            Self::TIME_OVER_SESSION_DELETE,
            &[&until_time],
        ).expect("TIME_OVER_SESSION_DELETE is Failed");

        //---------------------------------------------------------------------------------------------------------------------------------
        // 多重登録禁止チェック
        //---------------------------------------------------------------------------------------------------------------------------------
        let mut ret_is_exists_check : bool = false;

        //---------------------------------------------------------------------------------------------------------------------------------
        // 整合性のチェック
        //---------------------------------------------------------------------------------------------------------------------------------
        //前回のlaravel_sessionを継続するかどうかのフラグ
        let mut laravel_continue :bool = false;
        //戻り値のlogin_id
        let mut ret_login_id :i32 =-1;
        //取り出したsessionを解析
        let ret_post_token_id: String = if getcoolie_split_ary.len() > 1 {
            //-----------------------------------------------------------------------------------------------------------------------------
            // 本番モードにてリファラなし、又は プロクシを通していればセッション拒否
            //-----------------------------------------------------------------------------------------------------------------------------
            if (ret_is_debug == false && ret_http_referer == "") || ( str_http_x_forwarded_for != str_http_x_remote_addr ) {
                String::from("")
            } else {
                //-------------------------------------------------------------------------------------------------------------------------
                // cookieからlocal_uuidを取得
                //-------------------------------------------------------------------------------------------------------------------------
                let local_uuid = if getcoolie_split_ary[1] == "" {String::from("")}else{seg4_common::decrypt(&getcoolie_split_ary[1])};

                //-------------------------------------------------------------------------------------------------------------------------
                // 多重登録のチェック(汎用)
                //-------------------------------------------------------------------------------------------------------------------------
                let exists_check_stmt = pg_client.prepare_typed(&Self::UPDATE_EXISTS_CHECK, &[
                    db_base::Type::TEXT,db_base::Type::TEXT,db_base::Type::INT8
                ]).unwrap();
                let exists_check_query = pg_client.query_one(
                    &exists_check_stmt,&[&local_uuid,&ret_reqest_uri,&seg4_common::define::UPDATE_ASEC]
                );
                ret_is_exists_check = if exists_check_query.is_err() ==true{
                    false
                } else{
                    match exists_check_query.expect("UPDATE Exists Chekking Error.").get(0) {
                        Some(value) => value,
                        None => false  
                    }
                };

                //-------------------------------------------------------------------------------------------------------------------------
                // 整合性のチェック
                //-------------------------------------------------------------------------------------------------------------------------
                let referer_check_ary: Vec<&str> = ret_http_referer.split("/").collect();//refererはドメイン部分だけ
                let check_referer = if referer_check_ary.len() > 2 {
                    String::from(format!("{}/{}/{}%",referer_check_ary[0],referer_check_ary[1],referer_check_ary[2]))
                }else{
                    String::from("")
                };
                let prep_stmt = pg_client.prepare_typed(&Self::SELECT_BY_UUID, &[
                    db_base::Type::TEXT,db_base::Type::TEXT,db_base::Type::TEXT,
                    db_base::Type::TEXT,db_base::Type::INT8,db_base::Type::TEXT
                ]).unwrap();
                let count_ret = pg_client.query(&prep_stmt, &[
                    &local_uuid,&ret_user_agent,&ret_realip_remote_addr,&check_referer,&ret_timestamp,&ret_reqest_uri
                ]).unwrap();
                let mut count_score: i64 = 0;
                for count_args in count_ret {
                    ret_login_id = count_args.get(0);
                    count_score+=1;
                };
                //正常に最終更新時刻が更新出来た場合は現状維持。
                let ret2_uuid = if count_score == 1 {
                    laravel_continue = true;
                    local_uuid
                } else {
                    //---------------------------------------------------------------------------------------------------------------------
                    //メソッドがGETなら新しいuuidを付与
                    //---------------------------------------------------------------------------------------------------------------------
                    Self::new_session_record (
                        &ret_reqest_method,&ret_http_referer,&ret_user_agent,
                        &ret_realip_remote_addr,&ret_reqest_uri,&ret_timestamp,pg_client, 
                    )
                };
                ret2_uuid
            }
        }else{
            //-----------------------------------------------------------------------------------------------------------------------------
            //メソッドがGETなら新しいuuidを付与
            //-----------------------------------------------------------------------------------------------------------------------------
            Self::new_session_record (
                &ret_reqest_method,&ret_http_referer,&ret_user_agent,
                &ret_realip_remote_addr,&ret_reqest_uri,&ret_timestamp,pg_client, 
            )
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        // 最終的なcookieに投入する値。uuidは新規発行又は既存のsessionテーブルから抽出
        //---------------------------------------------------------------------------------------------------------------------------------
        let ret_cookie_line :String = if ret_post_token_id == "" {
            format!("laravel_session=;Domain={};Path=/;{}",ret_realip_remote_addr,seg4_common::define::SAMESITE_SECURE)
        }else {
            format!("laravel_session={};Domain={};Path=/;{}",if laravel_continue == true {
                getcoolie_split_ary[1].to_string()
            }else{
                seg4_common::encrypt(&ret_post_token_id)},ret_realip_remote_addr,seg4_common::define::SAMESITE_SECURE)
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        // 戻り値として返却
        //---------------------------------------------------------------------------------------------------------------------------------
        ServerInfomation {
            reqest_method:ret_reqest_method,
            user_agent:ret_user_agent, 
            http_referer:ret_http_referer,
            realip_remote_addr:ret_realip_remote_addr,
            http_content_length:ret_http_content_length,
            http_content_type:ret_http_content_type,
            reqest_uri:ret_reqest_uri,
            query_string:ret_query_string,
            is_mobile:ret_is_mobile,
            is_exists_check:ret_is_exists_check,
            http_authenticate:ret_http_authenticate,
            http_x_remote_addr:ret_http_x_remote_addr,
            http_x_forwarded_for:ret_http_x_forwarded_for,
            post_token_id:ret_post_token_id,
            is_debug:ret_is_debug,
            cookie_line:ret_cookie_line,
            last_access:ret_last_access,
            timestamp:ret_timestamp,
            business_login_id:ret_login_id,
        }
    } //メンバ関数:set_server_infomation ブロック

    //-------------------------------------------------------------------------------------------------------------------------------------
    // * トレイト内関数:new_session_record
    // * 新しいsessionレコードを追加する
    //-------------------------------------------------------------------------------------------------------------------------------------
    fn new_session_record (
        ret_reqest_method:&String,
        ret_http_referer:&String,
        ret_user_agent:&String,
        ret_realip_remote_addr:&String,
        ret_reqest_uri:&String,
        ret_timestamp:&i64,
        pg_client: &mut db_base::postgres::Client, 
    )->String {
        //---------------------------------------------------------------------------------------------------------------------------------
        //メソッドがGETなら新しいuuidを付与
        //---------------------------------------------------------------------------------------------------------------------------------
        if ret_reqest_method == "GET" {
            //-----------------------------------------------------------------------------------------------------------------------------
            // (新規session発行時のみ使用):新しいuuid ※要改修:既存の場合はsessionから取り出し。
            //-----------------------------------------------------------------------------------------------------------------------------
            let local_uuid: String = seg4_common::Uuid::new_v4().to_string();
            //sessionを新規登録
            pg_client.execute(
                Self::INSERT_SESSION_MANAGEMENTS,
                &[&local_uuid,&ret_http_referer,&ret_user_agent,&ret_realip_remote_addr,&ret_reqest_uri,&ret_timestamp],
            ).unwrap();
            local_uuid
        } else {
            String::from("")
        }
    } //メンバ関数:new_session_record ブロック


}//トレイト:ServerInfomation ブロック

//-----------------------------------------------------------------------------------------------------------------------------------------
// トレイト (構造体:InputParametars)
//-----------------------------------------------------------------------------------------------------------------------------------------
impl InputParametars {
    //-------------------------------------------------------------------------------------------------------------------------------------
    // * トレイト内関数:set_input_parametars
    // * 構造体InputParametars として値を代入する。引数項目以外は演算して代入を実施 
    // * 目的:フォームハンドラーの値を精査し、サーバエラーで無くヴァリテーションバックとして返却
    // * 境界値チェックや不正アクセスの精査を体系的に実施
    //-------------------------------------------------------------------------------------------------------------------------------------
    pub fn set_input_parametars(
        //----- 引数一覧 -----//
        required : bool ,   // 必須項目 trueなら必須
        args: String,           //ハンドラ文字列(ハンドラ名はハッシュの添え字で判別)
        string_type_in: bool,   //文字列で扱うなら true それ以外なら false
        mut message_in: String, //ヴァリテーションバック時のメッセージ文字列
        min_in: i64,            //最小値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
        max_in: i64,            //最大値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
        check_regix_in: String,
        //----- 戻り値 -----//
    ) -> InputParametars {
        //戻り値の型は構造体InputParametars

        //---------------------------------------------------------------------------------------------------------------------------------
        //  正規表現トレイト
        //---------------------------------------------------------------------------------------------------------------------------------
        //実数
        let regix_jissuu = Regex::new(r"\d+(?:\.\d+)?").unwrap();
        //整数
        let regix_seisuu = Regex::new(r"[+-]?\d+").unwrap();

        //---------------------------------------------------------------------------------------------------------------------------------
        //  整数の処理 文字列を整数に変換
        //---------------------------------------------------------------------------------------------------------------------------------
        let ret_intvalue: i64 = if regix_jissuu.is_match(&args) == false {
            0
        } else {
            let _data = regix_seisuu.captures(&args).unwrap().at(0).unwrap();
            let sandata: i64 = _data.parse().expect("変換できない文字列でした");
            sandata
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        //  文字列長 ※強引にカウント
        //---------------------------------------------------------------------------------------------------------------------------------
        let ret_str_length: i64 = args.chars().count() as i64;

        //---------------------------------------------------------------------------------------------------------------------------------
        //  数値を扱う場合、フロートも算出
        //---------------------------------------------------------------------------------------------------------------------------------
        let ret_float_value: f64 = if string_type_in == true {
            0.0
        } else if regix_jissuu.is_match(&args) == false {
            0.0
        } else {
            let _data = regix_jissuu.captures(&args).unwrap().at(0).unwrap();
            let cst_data: f64 = _data.parse().expect("変換できない文字列でした");
            cst_data
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        //  整合性チェックの初期値
        //---------------------------------------------------------------------------------------------------------------------------------
        let mut ret_result: bool = true; //文字列チェック結果の初期値

        //---------------------------------------------------------------------------------------------------------------------------------
        //  必須項目
        //---------------------------------------------------------------------------------------------------------------------------------
        if required == true && ret_str_length < 1 {
            message_in = "必須項目です".to_string();
            ret_result = false;      
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        //  文字列長、値のチェック ※存在のみチェックする場合、 0,-1を指定
        //---------------------------------------------------------------------------------------------------------------------------------
        if string_type_in == true {
            if ret_str_length <= min_in && min_in > -1 {
                message_in = format!("{}文字より多く入力して下さい({}文字)", min_in,ret_str_length);
                ret_result = false;
            } else if ret_str_length > max_in && max_in > -1 {
                message_in = format!("{}文字以下で入力して下さい({}文字)", max_in,ret_str_length);
                ret_result = false;
            }
        } else {
        //---------------------------------------------------------------------------------------------------------------------------------
        //  フロート、値のチェック ※-1はマジックナンバー
        //---------------------------------------------------------------------------------------------------------------------------------
            if ret_float_value <= min_in as f64 && min_in != -1 {
                message_in = format!("{}を超える値を入力して下さい", min_in);
                ret_result = false;
            } else if ret_float_value > max_in as f64 && max_in != -1 {
                message_in = format!("{}未満の値を入力して下さい", max_in);
                ret_result = false;
            }
        //---------------------------------------------------------------------------------------------------------------------------------
        //  整数、値のチェック ※-1はマジックナンバー
        //---------------------------------------------------------------------------------------------------------------------------------
            if ret_intvalue <= min_in && min_in != -1 {
                message_in = format!("{}より多い値を入力して下さい", min_in);
                ret_result = false;
            } else if ret_intvalue > max_in && max_in != -1 {
                message_in = format!("{}未満の値を入力して下さい", max_in);
                ret_result = false;
            }
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        //  正規表現(境界値精査もこれで実施すること！)
        //---------------------------------------------------------------------------------------------------------------------------------
        //  値チェックが有効時のみ、結果を反映
        if ret_result == true {
            ret_result = match &*check_regix_in {
                r"*" => true,
                _ => {
                    let check_regix = Regex::new(&check_regix_in).unwrap();
                    check_regix.is_match(&args)
                }
            };
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        // 戻り値として返却
        //---------------------------------------------------------------------------------------------------------------------------------
        InputParametars {
            string_type: string_type_in,  // 文字列で扱うなら true それ以外なら false
            str_value: args,              // 文字列でのパンドラ値
            str_length: ret_str_length,   // 文字列長
            int_value: ret_intvalue,      //整数でのパンドラ値
            float_value: ret_float_value, // floatでのパンドラ値
            result: ret_result,           // 値チェックの結果 OK なら true NG なら false
            result_msg: message_in,       // ValidationBack時のメッセージ
        }
    } //メンバ関数:set_input_parametars ブロック

    //-------------------------------------------------------------------------------------------------------------------------------------
    // * トレイト内関数:sanitize
    // * 関数set_input_parametarsのコール爆 としてサイニタイズを実施する。 
    // * 目的:サニタイズ処理
    // * 境界値チェックや不正アクセスの精査を体系的に実施
    //-------------------------------------------------------------------------------------------------------------------------------------
    pub fn sanitize(&self) -> InputParametars {
        //---------------------------------------------------------------------------------------------------------------------------------
        // サニタイズ処理
        //---------------------------------------------------------------------------------------------------------------------------------        
        let ret_str_value :String = self.str_value.replace(";","；").replace("&","&amp;").replace("\"","&quot;").replace("'","&#39;").
            replace("$","&#36;").replace("<","&lt;").replace(">","&gt;").replace("/","&#47;").replace("|","&#124;");

        //---------------------------------------------------------------------------------------------------------------------------------
        // 戻り値として返却
        //---------------------------------------------------------------------------------------------------------------------------------
        InputParametars {
            string_type: self.string_type,
            str_value: ret_str_value,
            str_length: self.str_length,
            int_value: self.int_value,
            float_value: self.float_value,
            result: self.result, 
            result_msg: self.result_msg.to_string(), 
        }
    }
} //トレイト:InputParametars ブロック

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::action_base::InputParametars;

    #[test]
    fn test_set_input_parametars() {
        let mut input_params = seg4_common::HashMap::new();
        let mut input_result = seg4_common::HashMap::new();
        let mut valiback_detail = seg4_common::HashMap::new();
        input_result.insert(String::from("Result"), 0);
        input_params.insert(
            String::from(r"str_test"),
            InputParametars::set_input_parametars(
                true,
                String::from("野球サッカー機械西&\"';$<>/|"),//ハンドラ文字列(ハンドラ名はハッシュの添え字で判別)
                true,                                   //文字列で扱うなら true それ以外なら false
                r"".to_string(),                    //ヴァリテーションバック時のメッセージ文字列
                -1,                                         //最小値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
                -1,                                         //最大値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
                r"*".to_string(),                   //正規表現チェック。境界値もこれで行う。全スルーは *
            ).sanitize()
        );
        input_params.insert(
            String::from(r"val_test"),
            InputParametars::set_input_parametars(
                true,
                String::from(r"3.144"),//ハンドラ文字列(ハンドラ名はハッシュの添え字で判別)
                false,                              //文字列で扱うなら true それ以外なら false
                r"".to_string(),                //ヴァリテーションバック時のメッセージ文字列
                2,                                  //最小値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
                30,                                  //最大値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
                r"*".to_string(),           //正規表現チェック。境界値もこれで行う。全スルーは *
            ),
        );
        input_params.insert(String::from(r"regix_test"),InputParametars::set_input_parametars(
            true,String::from("test@hyda-crypto.local"),true,"所定の書式にて入力して下さい".to_string(),2,-1,
            "^[A-Za-z0-9]{1}[A-Za-z0-9_.-]*@{1}[A-Za-z0-9_.-]+.[A-Za-z0-9]+$".to_string(),),
        );
        for (key, value) in &input_params {
            if value.result == false {
                //詳細を追加
                valiback_detail.insert(key, &value.result_msg);
                //全体の戻り値を更新
                input_result.insert(String::from("Result"), 5);
            }
        }
        let &check_result = input_result.get(&String::from("Result")).unwrap();
        assert_eq!(check_result, 0);
    }
}
