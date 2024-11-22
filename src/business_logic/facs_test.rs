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
//  action_baseの読み込み
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::action_base;

//-----------------------------------------------------------------------------------------------------------------------------------------
// JSON出力
//-----------------------------------------------------------------------------------------------------------------------------------------
use std::io::prelude::*;
use std::fs::{File,create_dir_all};

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
        /* 画面別個別機能:変数定義  */
        let passwd = &input_params.get("passwd").expect("Input Value Error[passwd].").str_value;
        let user_id = &input_params.get("user_id").expect("Input Value Error[user_id].").str_value;
        const table_name: &str = "word_press";
        const where_args: &str = "";
        const QUERY_SQL: &str ="with tmp_table as (
                select
            plimary,
            last_update,
            site_fqdn,
            site_name,
            id,
            date,
            modified,
            slug,
            title,
            content,
            excerpt,
            author,
            categories,
            tags,
            keywords

        from ### TABLE_NAME ### ### WHERE ### ### ORDER BY ### ### LIMIT_OFFSET ###
        )
            select cast(to_json(tmp_table.*) as text) as out from tmp_table;
        ";
        const LIMIT_OFFSET: &str = "limit $1 offset $2";
        let page_in: i32 = 2;
        let content: i32 = 2;
        const orderby_args: &str = "order by plimary asc";
        //-------------------------------------------------------------------------------------------------------------------------------------
        // オフセットの取得
        //-------------------------------------------------------------------------------------------------------------------------------------
        let offset = content * (page_in - 1);
        let modeltest_queryload_stmt = pg_client.prepare_typed(&QUERY_SQL.replace("### WHERE ###",&where_args)
        .replace("### TABLE_NAME ###",&table_name).replace("### ORDER BY ###",&orderby_args)
        .replace("### LIMIT_OFFSET ###",&if content < 1 || page_in < 1 {String::from("")} else{
            LIMIT_OFFSET.to_string()}), &[
                //stmt も F.A.C.Sにて動的に作成 固定は $1:content $2:offsetの順
                db_base::Type::INT4,db_base::Type::INT4,db_base::Type::TEXT,db_base::Type::TEXT
            ]).unwrap();
        
        // F.A.C.S用にqueryも変数で定義
        let modeltest_queryload_query = pg_client.query(&modeltest_queryload_stmt,&[
            &content,&offset,&passwd,&user_id  
        ]).expect("Select Error.");

        //総件数をカウント テーブル名は F.A.C.Sにて生成
        const COUNT_SQL: &str = "select count(*) as count from ### TABLE_NAME ###;";

        //-------------------------------------------------------------------------------------------------------------------------------------
        // 総件数の取得
        //-------------------------------------------------------------------------------------------------------------------------------------
        let record_count_stmt = pg_client.prepare_typed(&COUNT_SQL.replace("### WHERE ###",&where_args).
            replace("### TABLE_NAME ###",&table_name), &[
                db_base::Type::TEXT,db_base::Type::TEXT
            ]).unwrap();
        let record_count_query = pg_client.query_one(&record_count_stmt, &[
                &passwd,&user_id
            ]);
        let record_count_id : i64 = if record_count_query.is_err() == true {
            -1
        } else{
            match record_count_query.expect("Record Count Chekking Error.").get(0) {
                Some(value) => value,
                None => -1
            }
        };

        //-------------------------------------------------------------------------------------------------------------------------------------
        // マックスページの取得
        //-------------------------------------------------------------------------------------------------------------------------------------
        //let record_count = record_count_id as u64;
        let max_page = ((record_count_id as f64/ content as f64) as f64).ceil();
        let mut return_strings = String::from("{\"data\":[");
        let mut count :i32 = 0;
        let ptn1 = ",";
        for rows in modeltest_queryload_query.into_iter(){
            let stn :String = rows.get("out");
            return_strings +=format!("{}{}",stn,ptn1).as_str();  
            count +=1;
        }
        if count == 0{
                return_strings=String::from("{\"data\":{}, \"rows\":0}");
        };
        return_strings.pop();
        return_strings +=format!("]{} \"all_rows\":{}{} \"max_page\":{}}}",ptn1,record_count_id,ptn1,max_page).as_str();
        /* 画面別個別機能ココマデ */

        //-------------------------------------------------------------------------------------------------------------------------------------
        // 件数が無い場合は NOTFOUND_ERROR
        //-------------------------------------------------------------------------------------------------------------------------------------
        if return_strings =="" {
            return BusinessLogic {
                result:5,
                data:seg4_common::NOTFOUND_ERROR.to_string(), 
            }
        };

        //-------------------------------------------------------------------------------------------------------------------------------------
        // 永続JSONの出力
        //-------------------------------------------------------------------------------------------------------------------------------------
        //user_agent 
        if _server_info.is_debug == true && _server_info.user_agent.contains("curl") == true && _server_info.query_string == "permanent" {
            let permanent_dir = &format!("{}/{}",seg4_common::define::JSON_PERMANENT_DIR,_server_info.reqest_uri.replace("api","static"));
            create_dir_all(permanent_dir).expect("mkdir[JSON_PERMANENT_DIR]  is Failed");
            let mut file = File::create(format!("{}/index.json",&permanent_dir)).expect("Static JsonFile Create is Failed");
            file.write_all(return_strings.as_bytes()).expect("CreatedJsonFile Output is Failed");
        }

        //---------------------------------------------------------------------------------------------------------------------------------
        // 戻り値として返却:※正常終了
        //---------------------------------------------------------------------------------------------------------------------------------
        BusinessLogic {
            result:0,
            data:format!("{{\"result\":{}}}",return_strings),
        }
    } //execute 終端
}//impl 終端

