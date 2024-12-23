/*
手順
①touch ./src/business_logic/membership_confirm.rs
②vi ./src/business_logic/mod.rs
[pub mod membership_confirm; //<コメント>]
③./src/business_logic/membership_confirm.rs を編集


*/

#![allow(non_snake_case)]
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
// implementations (構造体:BusinessLogic)
//-----------------------------------------------------------------------------------------------------------------------------------------
impl BusinessLogic {
    //-------------------------------------------------------------------------------------------------------------------------------------
    // 画面個別SQL
    //-------------------------------------------------------------------------------------------------------------------------------------
    //期日超過レコードの削除
    const DELETE_AUTH_BASIC: &str = "delete from seg4planet_auth_basic where 
        auth_id = -1 and  confirm_uuid is not null and register_datetime + $1  < cast(extract(epoch from now()) as integer);
    ";

    // 確認対象抽出 & ID抽出
    const CONFIRM_AUTH_BASIC_EXISTS: &str = "select plimary from seg4planet_auth_basic where 
        auth_id = -1 and confirm_uuid=$1  and register_datetime + $2 > cast(extract(epoch from now()) as integer) 
        order by register_datetime desc limit 1;
        ";
    //ログインID更新 & 制限管理解除
    const CONFIRM_AUTH_BASIC_UPDATE: &str = "update seg4planet_auth_basic set auth_id = $3 ,register_datetime = 0 ,confirm_uuid=NULL where
    auth_id = -1 and confirm_uuid=$1  and register_datetime + $2 > cast(extract(epoch from now()) as integer) returning auth_id;";

    //-------------------------------------------------------------------------------------------------------------------------------------
    // execute 処理開始
    //-------------------------------------------------------------------------------------------------------------------------------------
    pub fn execute(
        _server_info: &action_base::ServerInfomation,
        input_params: &seg4_common::HashMap::<String,action_base::InputParametars>,
        pg_client: &mut db_base::postgres::Client,
    ) -> BusinessLogic {
        //---------------------------------------------------------------------------------------------------------------------------------
        // ビジネスロジックをここに記載
        //---------------------------------------------------------------------------------------------------------------------------------
        /* 画面別個別機能  */
        let confirm_uuid = input_params.get("confirm_uuid").expect("Input Value Error[confirm_uuid].").str_value.to_string();
        /* 画面別個別機能ココマデ */

        //---------------------------------------------------------------------------------------------------------------------------------
        // 時間経過したレコードを削除
        //---------------------------------------------------------------------------------------------------------------------------------
        pg_client.execute(
            Self::DELETE_AUTH_BASIC,
            &[&seg4_common::define::REGISTER_CONFIRM_TIME],
        ).expect("DELETE_AUTH_BASIC is Failed");

        //---------------------------------------------------------------------------------------------------------------------------------
        // リクエストのuuidが存在しているかどうかのチェック
        //---------------------------------------------------------------------------------------------------------------------------------
        let confirm_check_stmt = pg_client.prepare_typed(
            Self::CONFIRM_AUTH_BASIC_EXISTS, &[db_base::Type::TEXT,db_base::Type::INT8]
        ).unwrap();
        let confirm_check_query = pg_client.query_one(&confirm_check_stmt,&[&confirm_uuid,&seg4_common::define::REGISTER_CONFIRM_TIME]);
        let confirm_check_id : i32 = if confirm_check_query.is_err() == true {
            return BusinessLogic {
                result:0,
                data:String::from("{\"result\":\"201 NO AUTH DATA\"}"), 
            };
        } else {
            match confirm_check_query.expect("Login Data Chekking Error.").get(0) {
                Some(value) => value,
                None =>             return BusinessLogic {
                    result:0,
                    data:String::from("{\"result\":\"201 NO AUTH DATA\"}"), 
                },
            }        
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        // 取得したIDを付与
        //---------------------------------------------------------------------------------------------------------------------------------
        let confirm_update_stmt = pg_client.prepare_typed(
            Self::CONFIRM_AUTH_BASIC_UPDATE, &[db_base::Type::TEXT,db_base::Type::INT8,db_base::Type::INT4]
        ).unwrap();
        let confirm_update_query = pg_client.query_one(
            &confirm_update_stmt,&[&confirm_uuid,&seg4_common::define::REGISTER_CONFIRM_TIME,&confirm_check_id]
        );
        let confirm_update_id :i32 = if confirm_update_query.is_err() == true {
            return BusinessLogic {
                result:9,
                data:String::from("{\"result\":\"503 LOGINID UPDATE FAILED.\"}"), 
            };
        } else {
            match confirm_update_query.expect("Login Data Chekking Error.").get(0) {
                Some(value) => value,
                None => -1 
            }        
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        // 件数が無い場合処理を中断
        //---------------------------------------------------------------------------------------------------------------------------------
        if confirm_check_id != confirm_update_id {
            return BusinessLogic {
                result:9,
                data:String::from("{\"result\":\"503 MENBERSHIP CONFIRM FAILED.\"}"), 
            }
        } 

        //---------------------------------------------------------------------------------------------------------------------------------
        // 戻り値として返却:※正常終了
        //---------------------------------------------------------------------------------------------------------------------------------
        BusinessLogic {
            result:0,
            data:String::from("{\"result\":\"200 Menbership Confirm Compalete.\"}"), 
        }
    } //execute 終端
}//impl 終端

#[cfg(test)]
mod tests {
    use crate::base::seg4_common;
    use crate::base::db_base;
    use crate::base::action_base;
    use crate::business_logic::membership_confirm;
    #[test]
    fn test_bl_membership_confirm() {
          let server_info : action_base::ServerInfomation = action_base::ServerInfomation {
            reqest_method: String::from("GET"),
            user_agent: String::from("curl/7.77.0"),
            http_referer: String::from(""),
            realip_remote_addr: String::from("127.0.0.1"),
            http_content_length: String::from("94"),
            http_content_type: String::from("application/x-www-form-urlencoded"),
            reqest_uri: String::from("/json/api/member/confirm/06316faf-14f9-4364-a9b0-f9b70920de92"),
            query_string: String::from(""),
            is_mobile: false,
            is_exists_check: false,
            http_authenticate: String::from(""),
            http_x_remote_addr: String::from("127.0.0.1"),
            http_x_forwarded_for: String::from("127.0.0.1"),
            post_token_id: String::from("00000000-0000-0000-0000-000000000000"),
            is_debug: true,
            cookie_line: String::from("laravel_session=OE5wN2wqVEV3aUIrR1UwcYvTR5OQ7W55gtJkwbDvT6i1iyGd06m/LJoOAfSLx7mv+T8pZ78XW5WDUkrkIehvpg==;Domain=127.0.0.1;HttpOnly"),
            last_access: String::from("2023-06-25 01:40:11"),
            timestamp: 1687624811,
            business_login_id:-1,
        };
        let mut input_params = seg4_common::HashMap::<String,action_base::InputParametars>::new();
        input_params.insert(
        String::from(r"confirm_uuid"),
        action_base::InputParametars::set_input_parametars(
            true,
            String::from("06316faf-14f9-4364-a9b0-f9b70920de92"),//ハンドラ文字列(ハンドラ名はハッシュの添え字で判別)
            true,                     //文字列で扱うなら true それ以外なら false
            r"".to_string(),          //ヴァリテーションバック時のメッセージ文字列
            -1,                       //最小値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
            -1,                       //最大値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
            r"*".to_string(),         //正規表現チェック。境界値もこれで行う。全スルーは *
        ),
        );
        assert_eq!(0, membership_confirm::BusinessLogic::execute(&server_info,&input_params,&mut db_base::db_connect()).result);
    }
}

