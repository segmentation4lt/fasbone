#![allow(non_snake_case)]
//-----------------------------------------------------------------------------------------------------------------------------------------
// COMMON モジュール(SEG4)
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::seg4_common;

//-----------------------------------------------------------------------------------------------------------------------------------------
// DB モジュール
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::db_base;

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
    // SEG4初期テーブルか存在しているかどうかのチェック
    const SEG4_TABLE_CHECK: &str = "select count(*) count from pg_tables where tableowner=$1 and ( 
            tablename='seg4planet_auth_basic' or tablename='seg4planet_session_managements' or 
            tablename='seg4planet_logging_record'
        );
    ";

    // SEG4初期テーブル:セッション管理テーブル作成
    const SEG4PLANET_SESSION_MANAGEMENTS_SETUP: &str = "
        create table if not exists seg4planet_session_managements (
            plimary  serial unique ,
            uuid char(36)  not null  unique primary key,
            auth_id integer not null default -1,
            http_referer varchar  not null ,
            user_agent varchar  not null ,
            realip_remote_addr varchar  not null ,
            reqest_uri varchar  not null ,
            last_update bigint not null
        );
    ";

    // SEG4初期テーブル:セッション管理テストデータ
    const SEG4PLANET_SESSION_MANAGEMENTS_INSERT: &str = " 
        insert into public.seg4planet_session_managements(
            uuid, auth_id, http_referer, user_agent, realip_remote_addr, reqest_uri,last_update)
            values ('00000000-0000-0000-0000-000000000000', 1, '', 'curl/7.77.0', '192.168.23.13', '/json/api/', 0);
    ";

    // SEG4初期テーブル:メンバーシップ管理基幹テーブル作成
    const SEG4PLANET_AUTH_BASIC_SETUP: &str = "    
        create table if not exists seg4planet_auth_basic (
            plimary  serial unique ,
            auth_id integer not null default -1 ,
            auth_prefix varchar not null ,
            auth_password varchar not null ,
            regist_confirm_data  varchar default null ,
            confirm_uuid char(36) default null ,
            register_datetime bigint not null
        );
    ";

    // SEG4初期テーブル:メンバーシップ管理テストデータ
    const SEG4PLANET_AUTH_BASIC_INSERT: &str = "
        insert into public.seg4planet_auth_basic(
            auth_id, auth_prefix, auth_password, regist_confirm_data, confirm_uuid, register_datetime)
            values (1, 'test@hyda-crypt.local', '$2b$10$CE017CK0psCSUCKs/Xr2EufqFGmKDDQmFVFT/xHCToU.L4IVGz40O', 'curl/7.77.0', NULL, 0);   
    ";

    // SEG4初期テーブル:ロギングDBテーブル作成
    const SEG4PLANET_LOGGING_RECORD_SETUP: &str = "
        create table if not exists seg4planet_logging_record (
            plimary  serial unique ,
            backup_table_name varchar not null ,
            backup_record text not null ,
            backup_datetime timestamp not null default now()
        );
    ";

/*
        create table if not exists seg4planet_modeltest (
            plimary  serial unique ,
            auth_id int4 not null default -1,
            test1 varchar  default null ,
            test2 varchar  default null ,
            test3 varchar  default null ,
            test4 varchar  default null 
        );
insert into public.seg4planet_modeltest(
	 test1, test2, test3, test4)
	values  ('test1-1','test2-1','test3-1','test4-1'),
			('test1-2','test2-2','test3-2','test4-2'),
			('test1-3','test2-3','test3-3','test4-3'),
			('test1-4','test2-4','test3-4','test4-4'),
			('test1-5','test2-5','test3-5','test4-5');

*/

    //-------------------------------------------------------------------------------------------------------------------------------------
    // execute 処理開始
    //-------------------------------------------------------------------------------------------------------------------------------------
    pub fn execute(
        pg_client: &mut db_base::postgres::Client,
    ) -> BusinessLogic {
        //---------------------------------------------------------------------------------------------------------------------------------
        // SEG4初期テーブルか存在しているかどうかのチェック
        //---------------------------------------------------------------------------------------------------------------------------------
        let exists_stmt = pg_client.prepare_typed(&Self::SEG4_TABLE_CHECK, &[db_base::Type::TEXT]).unwrap();
        let exists_query = pg_client.query_one(&exists_stmt, &[&seg4_common::define::PG_CONNECT_DATABASE]);
        let is_exists_id :i64 = match exists_query.expect("seg4table Exists Chekking Error.").get(0) {
            Some(value) => value,
            None => -1 
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        // SEG4初期テーブルか存在していければテストデータと共に作成
        //---------------------------------------------------------------------------------------------------------------------------------
        if is_exists_id != 3 {
            pg_client.execute(Self::SEG4PLANET_SESSION_MANAGEMENTS_SETUP,&[]).expect("seg4planet_session_managements setup Error.");
            pg_client.execute(Self::SEG4PLANET_SESSION_MANAGEMENTS_INSERT,&[]).expect("seg4planet_session_managements insert Error.");
            pg_client.execute(Self::SEG4PLANET_AUTH_BASIC_SETUP,&[]).expect("seg4planet_auth_basic setup Error.");
            pg_client.execute(Self::SEG4PLANET_AUTH_BASIC_INSERT,&[]).expect("seg4planet_auth_basic insert Error.");
            pg_client.execute(Self::SEG4PLANET_LOGGING_RECORD_SETUP,&[]).expect("seg4planet_logging_record setup Error.");
        };

        //---------------------------------------------------------------------------------------------------------------------------------
        // 戻り値として返却:※正常終了
        //---------------------------------------------------------------------------------------------------------------------------------
        BusinessLogic {
            result:0,
            data:String::from("{\"result\":\"200 TABLE CHECK Complete.\"}"), 
        }
    } //execute 終端
}//impl 終端

#[cfg(test)]
mod tests {
    use crate::base::db_base;
    use crate::business_logic::index;
    #[test]
    fn test_bl_index() {
        assert_eq!(0, index::BusinessLogic::execute(&mut db_base::db_connect()).result);
    }
}
