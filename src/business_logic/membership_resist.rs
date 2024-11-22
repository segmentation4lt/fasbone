/*
手順
①touch ./src/business_logic/<アクション名>.rs
②vi ./src/business_logic/mod.rs
[pub mod <アクション名>; //<コメント>]
③./src/business_logic/<アクション名>.rs を編集




curl -XPOST -d '_token=00000000-0000-0000-0000-000000000000' -d 'userid=test@test.jp' -d 'passwd=nishi8888' -b 'laravel_session=OE5wN2wqVEV3aUIrR1UwcYvTR5OQ7W55gtJkwbDvT6i1iyGd06m/LJoOAfSLx7mv+T8pZ78XW5WDUkrkIehvpg==' http://127.0.0.1/json/api/member/resist/

*/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
//-----------------------------------------------------------------------------------------------------------------------------------------
// COMMON モジュール(SEG4)
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::seg4_common;

//-----------------------------------------------------------------------------------------------------------------------------------------
// DB モジュール
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::db_base;

//-----------------------------------------------------------------------------------------------------------------------------------------
// Mail モジュール
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::mail_base;

//-----------------------------------------------------------------------------------------------------------------------------------------
//  action_baseの読み込み
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::action_base;

/**
* ビジネスロジック
* 
* 
* 
* 
*/
//-----------------------------------------------------------------------------------------------------------------------------------------
// 構造体:BusinessLogic
//-----------------------------------------------------------------------------------------------------------------------------------------
#[derive(seg4_common::Serialize, seg4_common::Deserialize)]
pub struct BusinessLogic {
    pub result: i64,  // 0→正常出力 5→権限不足 9→ロジックエラー
    pub data: String,  // JSON化済の戻り値 ※エラー時はメッセージ
}

//-----------------------------------------------------------------------------------------------------------------------------------------
// トレイト (構造体:BusinessLogic)
//-----------------------------------------------------------------------------------------------------------------------------------------
impl BusinessLogic {
    //-------------------------------------------------------------------------------------------------------------------------------------
    // 画面個別SQL
    //-------------------------------------------------------------------------------------------------------------------------------------
    // 会員登録INSERT
    const INSERT_AUTH_BASIC: &str = "insert into public.seg4planet_auth_basic(
        auth_prefix, auth_password, regist_confirm_data, confirm_uuid,register_datetime)
        values ($1, $2, $3, $4 ,$5) returning auth_prefix || ',' || confirm_uuid as out;";
    // 既存のprefixが存在しているかどうかチェック
    const PREFIX_EXISTS_CHECK: &str = "select exists (select auth_prefix from seg4planet_auth_basic where auth_prefix = $1);";

    //-------------------------------------------------------------------------------------------------------------------------------------
    // execute 処理開始
    //-------------------------------------------------------------------------------------------------------------------------------------
    pub fn execute(
        server_info: &action_base::ServerInfomation,
        input_params: &seg4_common::HashMap::<String,action_base::InputParametars>,
        pg_client: &mut db_base::postgres::Client,
    ) -> BusinessLogic {
        //---------------------------------------------------------------------------------------------------------------------------------
        // ビジネスロジックをここに記載
        //---------------------------------------------------------------------------------------------------------------------------------
        /* 画面別個別機能 NACS生成時にで扱う型を決める */
        let userid =&input_params.get("userid").expect("Input Value Error[userid].").str_value;
        let passwd =&input_params.get("passwd").expect("Input Value Error[passwd].").str_value;
        //※_tokenは展開しない。
        // 既存のprefixが存在しているかどうかチェック
        let exists_stmt = pg_client.prepare_typed(&Self::PREFIX_EXISTS_CHECK, &[db_base::Type::TEXT]).unwrap();
        let exists_query = pg_client.query_one(&exists_stmt, &[&userid]);
        let is_exists_id = if exists_query.is_err() == true {
            true
        } else{
            match exists_query.expect("Prefix Exists Chekking Error.").get(0) {
                Some(value) => value,
                None => true
            }
        };

        //既存のprefixが存在していない場合は各種入力値を追記 
        if is_exists_id ==false {
            //-----------------------------------------------------------------------------------------------------------------------------
            // 認証テーブル仮登録
            //-----------------------------------------------------------------------------------------------------------------------------
            let membership_resist_stmt = pg_client.prepare_typed(&Self::INSERT_AUTH_BASIC, &[
                db_base::Type::TEXT,db_base::Type::TEXT,db_base::Type::TEXT,db_base::Type::TEXT,db_base::Type::INT8
            ]).unwrap();
            let membership_resist_query = pg_client.query_one(&membership_resist_stmt, &[
                &userid,&seg4_common::hashout(&passwd),&server_info.user_agent,&seg4_common::Uuid::new_v4().to_string(),&server_info.timestamp
            ]);
            let return_strings :String = if membership_resist_query.is_err() == true {
                String::from("")
            } else{
                match membership_resist_query.expect("LoggingDB Execute Failed.").get(0) {
                    Some(value) => value,
                    None => String::from("")
                }
            };
            //-----------------------------------------------------------------------------------------------------------------------------
            // 確認メイル配信
            //-----------------------------------------------------------------------------------------------------------------------------
            //BL戻り値
            if mail_base::build_email(
                seg4_common::define::MAIL_FROM, "会員登録のお知らせ","/member_resist/touroku.txt",&format!("{};{}",userid,return_strings)
            ) == true {
                //-----------------------------------------------------------------------------------------------------------------------------
                // 追加登録
                //-----------------------------------------------------------------------------------------------------------------------------

                /* 追加登録ココマデ */
                return BusinessLogic {
                    result:0,
                    data:String::from("{\"result\":\"200 Reqest Compalete.\"}") , 
                }
            } else {
                return BusinessLogic {
                    result:9,
                    data:String::from("{\"result\":\"500 Mail Send Failed.\"}"), 
                }
            };
        } else {
            //既存のprefixが存在しているので処理をしないで終了
            return BusinessLogic {
                result:5,
                data:String::from("{\"result\":\"400 userid Exists.\"}"), 
            }
        };
    } //execute 終端
}//impl 終端
