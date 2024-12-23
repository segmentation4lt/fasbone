/*
手順
①touch ./src/business_logic/membership_certification.rs
②vi ./src/business_logic/mod.rs
[pub mod membership_certification; //<コメント>]
③./src/business_logic/membership_certification.rs を編集

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
// トレイト (構造体:BusinessLogic)
//-----------------------------------------------------------------------------------------------------------------------------------------
impl BusinessLogic {
    //-------------------------------------------------------------------------------------------------------------------------------------
    // 画面個別SQL
    //-------------------------------------------------------------------------------------------------------------------------------------
    //ログイン認証
    const GET_AUTH_BASIC_ID : &str = "
    with tmp_table as (
        select auth_id,auth_password from seg4planet_auth_basic where auth_prefix = $1  
     ) select trim(concat_ws(',',tmp_table.*),'()')  as out from tmp_table;";
    //セッションにログインIDを付与
    const SET_AUTH_BASIC_ID : &str = "update seg4planet_session_managements set auth_id=$1 where uuid=$2;";

    //-------------------------------------------------------------------------------------------------------------------------------------
    // execute 処理開始
    //-------------------------------------------------------------------------------------------------------------------------------------
    pub fn execute(
        server_info: &action_base::ServerInfomation,
        input_params: &seg4_common::HashMap::<String,action_base::InputParametars>,
        pg_client: &mut db_base::postgres::Client,
    ) -> BusinessLogic {
        //-----------------------------------------------------------------------------------------------------------------------------
        // ビジネスロジックをここに記載
        //-----------------------------------------------------------------------------------------------------------------------------
        /* 画面別個別機能 NACS生成時にで扱う型を決める */
        let userid =input_params.get("userid").expect("Input Value Error[userid].").str_value.to_string();
        let passwd =input_params.get("passwd").expect("Input Value Error[passwd].").str_value.to_string();
        //※_tokenは展開しない。
        /* 画面別個別機能ココマデ */

        //-----------------------------------------------------------------------------------------------------------------------------
        // 認証テーブルに対してprefixをキーにして問い合わせを実施。
        //-----------------------------------------------------------------------------------------------------------------------------
        let login_stmt = pg_client.prepare_typed(Self::GET_AUTH_BASIC_ID, &[db_base::Type::TEXT]).unwrap();
        let login_query = pg_client.query_one(&login_stmt, &[&userid]);
        let login_result_binding = if login_query.is_err() == true {
            String::from("-1,")
        } else {
            match login_query.expect("Login Data Chekking Error.").get(0) {
                Some(value) => value,
                None => String::from("-1,"),
            }        
        };
        let login_result: Vec<&str> = login_result_binding.split(",").collect();
        let login_id : i32  = login_result[0].parse().expect("変換できない文字列でした");
        let hash_data = login_result[1];

        //-----------------------------------------------------------------------------------------------------------------------------
        // 認証の判定
        //-----------------------------------------------------------------------------------------------------------------------------
        let Data : String = if login_id > 0 && seg4_common::hashverify(&passwd,&hash_data) ==true {
            //-------------------------------------------------------------------------------------------------------------------------
            // ログイン成功
            //-------------------------------------------------------------------------------------------------------------------------
            pg_client.execute(
                Self::SET_AUTH_BASIC_ID,&[
                    &login_id,&server_info.post_token_id
                ],
            ).expect("auth_basic insert Error.");
            String::from("{\"result\":\"200 Login Compalete.\"}")
        } else {
            //-------------------------------------------------------------------------------------------------------------------------
            //ログイン失敗
            //-------------------------------------------------------------------------------------------------------------------------
            String::from("{\"result\":\"250 Login Failed.\"}")       
        };

        //-----------------------------------------------------------------------------------------------------------------------------
        // 戻り値として返却:
        //-----------------------------------------------------------------------------------------------------------------------------
        BusinessLogic {
            result:0,
            data:Data, 
        }
    } //execute 終端
}//impl 終端

#[cfg(test)]
mod tests {
    use crate::base::seg4_common;
    use crate::base::db_base;
    use crate::base::action_base;
    use crate::business_logic::membership_certification;
    #[test]
    fn test_bl_membership_certification() {
          let server_info : action_base::ServerInfomation = action_base::ServerInfomation {
            reqest_method: String::from("GET"),
            user_agent: String::from("curl/7.77.0"),
            http_referer: String::from(""),
            realip_remote_addr: String::from("127.0.0.1"),
            http_content_length: String::from("94"),
            http_content_type: String::from("application/x-www-form-urlencoded"),
            reqest_uri: String::from("/json/api/member/resist/"),
            query_string: String::from(""),
            is_mobile: false,
            http_authenticate: String::from(""),
            http_x_remote_addr: String::from("127.0.0.1"),
            http_x_forwarded_for: String::from("127.0.0.1"),
            post_token_id: String::from("00000000-0000-0000-0000-000000000000"),
            is_debug: true,
            is_exists_check: false,
            cookie_line: String::from("laravel_session=OE5wN2wqVEV3aUIrR1UwcYvTR5OQ7W55gtJkwbDvT6i1iyGd06m/LJoOAfSLx7mv+T8pZ78XW5WDUkrkIehvpg==;Domain=127.0.0.1;HttpOnly"),
            last_access: String::from("2023-06-25 01:40:11"),
            timestamp: 1687624811,
            business_login_id:-1,
        };
        let mut input_params = seg4_common::HashMap::<String,action_base::InputParametars>::new();
        input_params.insert(String::from(r"passwd"),action_base::InputParametars::set_input_parametars(
            true,String::from("abcdefghijk"),true,"英数字及び一部の記号のみ使用可能。".to_string(),7,-1,"^[A-Za-z0-9--_/*+.,!#$%&()~|]*$".to_string(),),
        );
        input_params.insert(String::from(r"userid"),action_base::InputParametars::set_input_parametars(
            true,String::from("test@localhost.localdomain"),true,"所定の書式にて入力して下さい。".to_string(),2,-1,
            "^[A-Za-z0-9]{1}[A-Za-z0-9_.-]*@{1}[A-Za-z0-9_.-]+.[A-Za-z0-9]+$".to_string(),),
        );
        input_params.insert(String::from(r"_token"),action_base::InputParametars::set_input_parametars(
            true,String::from("00000000-0000-0000-0000-000000000000"),true,"laravel tokenの値が不正です。".to_string(),35,37,r"^[a-z0-9-]*$".to_string(),),
        );
        assert_eq!(0, membership_certification::BusinessLogic::execute(&server_info,&input_params,&mut db_base::db_connect()).result);
    }
}
