//-----------------------------------------------------------------------------------------------------------------------------------------
// COMMON モジュール(SEG4)
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::seg4_common;

//-----------------------------------------------------------------------------------------------------------------------------------------
// Postgresクレート
//-----------------------------------------------------------------------------------------------------------------------------------------
pub extern crate postgres;
pub use postgres::{Client,  NoTls,types::Type};

//-----------------------------------------------------------------------------------------------------------------------------------------
// データベースに接続
//-----------------------------------------------------------------------------------------------------------------------------------------
pub fn db_connect()->Client{
    let db_url = format!(
        "host={} port={} user={} password={} dbname={} ",
        seg4_common::define::PG_CONNECT_HOST, seg4_common::define::PG_CONNECT_PORT,
        seg4_common::define::PG_CONNECT_USER, seg4_common::define::PG_CONNECT_PASS, seg4_common::define::PG_CONNECT_DATABASE
    );
    let pg_client = Client::connect(&db_url, NoTls).expect("failed to connect to postgres");
    pg_client
}

/*
/**
* ダイレクトに値(戻り値)を取得するselectを実行(insert/update/delete含む)
* query_oneを実施して UPDAT/DELETE時にログを取得。又、戻り値はSTRING型に変換して返却。※生文字列
* @param $where    : WHERE句
* @return return_strings : 文字列(STRING)
*/
pub fn seg4_loggingdb(
    table_name: String,//対象テーブル名:バックアップや件数カウントで使用する。
    where_args: String,//WHERE句 ※ハンドラから入力した値を元に動的に生成する
    pg_client: &mut Client,
    //以下、動的に引数を追加
)->String{
    //SQLはconstで記載。F.A.C.Sにて動的に変動。
    //※戻り値はPostgresのtext型にキャストする事。(returning cast(plimary as text),Rust側はString) 
    // 実行SQL
    const QUERY_SQL: &str = "update ### TABLE_NAME ### set auth_id=3 ### WHERE ###;";
    // バックアップ用のSQL
    const BACKUP_SQL: &str = "insert into seg4planet_logging_record
    (backup_table_name, backup_record) with tmp_table as (
            select *
             from ### TABLE_NAME ###  
            )  
            select '### TABLE_NAME ###' as name , cast(to_json(tmp_table.*) as text) as out from tmp_table ### WHERE ###::int8;
    ";

    //-------------------------------------------------------------------------------------------------------------------------------------
    // バックアップの取得
    //-------------------------------------------------------------------------------------------------------------------------------------
    if QUERY_SQL.to_lowercase().contains("update") == true || QUERY_SQL.to_lowercase().contains("delete") == true {
        let loggingdb_backup_stmt = pg_client.prepare_typed(&BACKUP_SQL.replace("### WHERE ###",&where_args).replace("### TABLE_NAME ###",&table_name), &[]).unwrap();
        let loggingdb_backup_query = pg_client.query_one(&loggingdb_backup_stmt, &[]);
        let loggingdb_backup_id : i64 = if loggingdb_backup_query.is_err() == true {
            -1
        } else{
            match loggingdb_backup_query.expect("LoggingDB Backup Failed.").get(0) {
                Some(value) => value,
                None => -1
            }
        };
        // loggingdb_backup_idが-1ということは対象件数0なので処理を中断
        if loggingdb_backup_id < 0 {
            // update/deleteで対象が無いのは許されない。
            return String::from("[Souce Code Error!!!] Record Seach Failed.");
        }
    }

    //-------------------------------------------------------------------------------------------------------------------------------------
    // SQLの実施
    //-------------------------------------------------------------------------------------------------------------------------------------
    let query_execute_stmt = pg_client.prepare_typed(&QUERY_SQL.replace("### WHERE ###",&where_args).replace("### TABLE_NAME ###",&table_name), &[]).unwrap();
    let query_execute_query = pg_client.query_one(&query_execute_stmt, &[]);
    let return_strings :String = if query_execute_query.is_err() == true {
        String::from("")
    } else{
        match query_execute_query.expect("LoggingDB Execute Failed.").get(0) {
            Some(value) => value,
            None => String::from("")
        }
    };
    return_strings
}


/**
* ページャーを意識したselectを実行(フェッチオール前提)
* PageNate 雛型　★関数名:<アクション名>_pagenate_(json|concat)　引数も連動。
* @param $page_in    : 何ページ目どうか nullだと件数取得 ※MAX ページは Service で処理! $maxPage=ceil($max/$content);
* @param $content : ページあたりの表示件数 (初期値が5)
* @param $order 並び順
* @param $table : テーブル名
* @param $select : セレクト部分
* @param $option   : オプション部分
* @param $
* @return $return[検索結果]:[件数]:[現在のページ]:[最大ページ]
*/
pub fn seg4_pagenate(
    table_name: String,//対象テーブル名:バックアップや件数カウントで使用する。
    where_args: String,//WHERE句 ※ハンドラから入力した値を元に動的に生成する
    orderby_args: String,//ORDER BY句 ※ハンドラから入力した値を元に動的に生成する
    page_in: &i32,//何ページ目どうか
    content: &i32,//ページあたりの表示件数
    pg_client: &mut Client,//接続ハンドル
    //以下、動的に引数を追加
)->String{ //複数の情報はJSONで出す 件数/最大ページ/表示したページ/ページあたりの表示件数
    //SQLはconstで記載。F.A.C.Sにて動的に変動。
    // jsonの場合
    // select cast(to_json(tmp_table.*) as text) as out from tmp_table;";  
    // concatの場合
    // select '\"\"' || trim(concat_ws(',',tmp_table.*),'()') || '\"\"' as out from tmp_table;";
    const QUERY_SQL: &str = "with tmp_table as (
        select 
            plimary,
            auth_id,
            test1,
            test2,
            test3,
            test4 
        from ### TABLE_NAME ### ### WHERE ### ### ORDER BY ### ### LIMIT_OFFSET ### 
        )
        select cast(to_json(tmp_table.*) as text) as out from tmp_table;
    ";
    const LIMIT_OFFSET: &str = "limit $1 offset $2";

    //-------------------------------------------------------------------------------------------------------------------------------------
    // オフセットの取得
    //-------------------------------------------------------------------------------------------------------------------------------------
    let offset = content * (page_in - 1);
    let modeltest_queryload_stmt = pg_client.prepare_typed(&QUERY_SQL.replace("### WHERE ###",&where_args)
    .replace("### TABLE_NAME ###",&table_name).replace("### ORDER BY ###",&orderby_args)
    .replace("### LIMIT_OFFSET ###",&if content < &1 || page_in < &1 {String::from("")} else{
        LIMIT_OFFSET.to_string()}), &[
        //stmt も F.A.C.Sにて動的に作成 固定は $1:content $2:offsetの順
            Type::INT4,Type::INT4,//※content、offsetを定義しないケースもある
        ]).unwrap();
    
    // F.A.C.S用にqueryも変数で定義
    let modeltest_queryload_query = pg_client.query(&modeltest_queryload_stmt,&[
        &content,&offset,//※content、offsetを定義しないケースもある  
    ]).expect("Select Error.");

    //総件数をカウント テーブル名は F.A.C.Sにて生成
    const COUNT_SQL: &str = "select count(*) as count from ### TABLE_NAME ###;";

    //-------------------------------------------------------------------------------------------------------------------------------------
    // 総件数の取得
    //-------------------------------------------------------------------------------------------------------------------------------------
    let record_count_stmt = pg_client.prepare_typed(&COUNT_SQL.replace("### WHERE ###",&where_args).
        replace("### TABLE_NAME ###",&table_name), &[]).unwrap();
    let record_count_query = pg_client.query_one(&record_count_stmt, &[]);
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
    let max_page = ((record_count_id as f64/ *content as f64) as f64).ceil();
    let mut return_query = String::from("{\"result\":[");
    let mut count :i32 = 0;
    let ptn1 = ",";
    for rows in modeltest_queryload_query.into_iter(){
        let stn :String = rows.get("out");
        return_query +=format!("{}{}",stn,ptn1).as_str();  
        count +=1;
    }
    if count == 0{
            return String::from("{\"result\":{}, \"rows\":0}");
    };
    return_query.pop();
    return_query +=format!("]{} \"all_rows\":{}{} \"max_page\":{}}}",ptn1,record_count_id,ptn1,max_page).as_str();
    return_query
}

#[cfg(test)]
mod tests {
    //テスト専用
    use crate::base::db_base;
    #[test]
    fn test_seg4_loggingdb() {
        let where_args = String::from("where plimary = 1 returning cast(plimary as text)");//WHERE句 ※ハンドラから入力した値を元に動的に生成する
        let table_name = String::from("seg4planet_modeltest");//対象テーブル名:バックアップや件数カウントで使用する。
        let mut pg_client = db_base::db_connect();
        assert_ne!("{}", db_base::seg4_loggingdb(table_name,where_args,&mut pg_client));
    }

    #[test]
    fn test_seg4_pagenate() {
        let where_args = String::from("");//WHERE句 ※ハンドラから入力した値を元に動的に生成する
        let table_name = String::from("seg4planet_modeltest");//対象テーブル名:バックアップや件数カウントで使用する。
        let orderby_args = String::from("");//ORDER BY句 ※ハンドラから入力した値を元に動的に生成する
        let page_in: i32 = 2; //何ページ目どうか
        let content: i32 = 2; //ページあたりの表示件数
        let mut pg_client = db_base::db_connect();
        assert_ne!("{}", db_base::seg4_pagenate(table_name,where_args,orderby_args,&page_in,&content,&mut pg_client));
    }
}
*/
