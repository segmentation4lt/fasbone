#!/bin/bash
#------------------------------------------------------------------------------
# SQL実行関数。区切り文字は半角セミコロン[;]
#------------------------------------------------------------------------------
exec_sql() {
	eval "psql -U $PG_CONNECT_USER -h localhost -p5432 $PG_CONNECT_DATABASE -tA -F \";\" -c  \"$*\""
}

#------------------------------------------------------------------------------
# 初期設定
#------------------------------------------------------------------------------
#コマンドのパス
export PATH="/usr/sbin:/usr/bin:/usr/local/bin:/usr/local/sbin:/sbin:/bin:/usr/X11/bin:/usr/local/pgsql/bin"
#カレントディレクトリ
JOBNAME=$(basename $0) >/dev/null 2>&1
JOBDIR=$(echo $0 | sed "s/$JOBNAME//g") >/dev/null 2>&1
[ "$JOBDIR" = "./" ] && JOBDIR=$(pwd)
#プロジェクト名の取得
project_name=$(basename $(echo $JOBDIR | sed s'@/fascon@@g'))
#アクション名 存在を確認
action_name=$1
if [ $(exec_sql "select count(plimary) from fascon_parent_action where action_name='$action_name' and project_name='$project_name';") -eq 0 ]; then
    echo "No such action."
    exit 9
fi
#parent_actionのパラメタ取得
action_record=$(exec_sql "select reqest_method::text, reqest_uri::text,guest_access_allow::text,update_exists_allow::text,action_overview::text,bl_args_table_name::text,bl_args_orderby::text,bl_args_page_in::text,bl_args_content::text,bl_type::text,bl_pagenate_outstyle::text from fascon_parent_action where action_name='$action_name' and project_name='$project_name';")
#リクエストメソッド
reqest_method=$(echo $action_record | cut -d ";" -f 1)
#リクエストURI
reqest_uri=$(echo $action_record | cut -d ";" -f 2)
#ログイン必須ページかどうか
guest_access_allow=$(echo $action_record | cut -d ";" -f 3)
#多重登録禁止チェックかどうか
update_exists_allow=$(echo $action_record | cut -d ";" -f 4)
#アクション概要文
action_overview=$(echo $action_record | cut -d ";" -f 5)
#テーブル名
bl_args_table_name=$(echo $action_record | cut -d ";" -f 6)
#order by句
bl_args_orderby=$(echo $action_record | cut -d ";" -f 7)
#何ページ目どうか (10.31改修:pageで固定)
bl_args_page_in="page"
#ページあたりの表示件数
bl_args_content=$(echo $action_record | cut -d ";" -f 9)
#ビジネスロジックの雛型タイプ pagenate/loggingdb
bl_type=$(echo $action_record | cut -d ";" -f 10)
#pagenate時の出力スタイル。json又はcsv
bl_pagenate_outstyle=$(echo $action_record | cut -d ";" -f 11)
#------------------------------------------------------------------------------
# 一気に取れないSQLを生成
#------------------------------------------------------------------------------
bl_args_sql_begin=$(exec_sql "select bl_args_sql_begin from fascon_parent_action where action_name='$action_name' and project_name='$project_name';")
bl_args_where=$(exec_sql "select bl_args_where from fascon_parent_action where action_name='$action_name' and project_name='$project_name';")
cgi_dynamic_head=$(exec_sql "select cgi_dynamic_head from fascon_parent_action where action_name='$action_name' and project_name='$project_name';")
cgi_replace_body=$(exec_sql "select cgi_replace_body from fascon_parent_action where action_name='$action_name' and project_name='$project_name';")

#アクションメンバ 存在を確認
menber_count=$(exec_sql "select coalesce(max(fascon_action_members.sql_id),0) from fascon_action_members inner join fascon_parent_action on fascon_parent_action.plimary=fascon_action_members.action_id where fascon_parent_action.action_name='$action_name';")
#if [ $menber_count -eq 0 ]; then
#    echo "No such member."
#    exit 9
#fi
#一時保存アクションファイル
ACTION_2_FILE=$(test $menber_count -eq 0 && echo "action_2n.txt" || echo "action_2.txt")

BL_1_FILE=$(test $menber_count -eq 0 && echo "bl_1n.txt" || echo "bl_1.txt")

TMP_ACTION_FILE=/tmp/make_action.tmp
[ -f $TMP_ACTION_FILE ] && rm -f $TMP_ACTION_FILE
touch $TMP_ACTION_FILE

#一時保存ビジネスロジックファイル
TMP_BL_FILE=/tmp/make_bl.tmp
[ -f $TMP_BL_FILE ] && rm -f $TMP_BL_FILE
touch $TMP_BL_FILE

#一時保存メインファイル
TMP_MAIN_FILE=/tmp/make_main_rs.tmp
[ -f $TMP_MAIN_FILE ] && rm -f $TMP_MAIN_FILE
touch $TMP_MAIN_FILE

#一時保存controller/mod.rs
TMP_CONTROLLER_MOD_RS=/tmp/make_controller_mod_rs.tmp
[ -f $TMP_CONTROLLER_MOD_RS ] && rm -f $TMP_CONTROLLER_MOD_RS
touch $TMP_CONTROLLER_MOD_RS

#一時保存business_logic/mod.rs
TMP_BUSINESS_LOGIC_MOD_RS=/tmp/make_business_logic_mod_rs.tmp
[ -f $TMP_BUSINESS_LOGIC_MOD_RS ] && rm -f $TMP_BUSINESS_LOGIC_MOD_RS
touch $TMP_BUSINESS_LOGIC_MOD_RS

#StructParamの中身
TMP_struct_param=/tmp/struct_param.tmp
[ -f $TMP_struct_param ] && rm -f $TMP_struct_param
touch $TMP_struct_param

#member_check_loopの中身
TMP_member_check_loop=/tmp/member_check_loop.tmp
[ -f $TMP_member_check_loop ] && rm -f $TMP_member_check_loop
touch $TMP_member_check_loop

#member_get_loopの中身
TMP_member_get_loop=/tmp/member_get_loop.tmp
[ -f $TMP_member_get_loop ] && rm -f $TMP_member_get_loop
touch $TMP_member_get_loop

#ParhStrinArgs
path_string_args=$(exec_sql "select 'String' as member_type from fascon_action_members inner join fascon_parent_action on fascon_parent_action.plimary=fascon_action_members.action_id where fascon_parent_action.action_name='$action_name' order by fascon_action_members.sql_id asc;")
path_string_args=$(echo $path_string_args | sed "s@ @,@g")
#ParhMemberName
path_member_name=$(exec_sql "select member_name::text from fascon_action_members inner join fascon_parent_action on fascon_parent_action.plimary=fascon_action_members.action_id where fascon_parent_action.action_name='$action_name' order by fascon_action_members.sql_id asc;")
path_member_name=$(echo $path_member_name | sed "s@ @,@g")
#ParhMemberItarete
path_member_itarete=$([ $reqest_method == "Path" ] && echo "let ($path_member_name) = path.into_inner();" || echo "")
#REQEST_METHOD_FORM
reqest_method_form=""
#REQEST_STRUCT_ARGS
reqest_struct_args=""
#REQEST_METHOD_FORMの中身
#SQL_STMT 各メンバのプリペアを定義
#ページングなし
sql_stmt_no_page=""
#ページングあり
sql_stmt_pages=""
#SQL_EXECUTE 各メンバの参照を代入
#ページングなし
sql_exec_no_page=""
#ページングあり
sql_exec_pages=""
if [ $reqest_method == "Post" ]; then
    reqest_struct_args="&postForm."
    reqest_method_form="postForm: actix_web::web::Form<PostParam>,"
    echo "    //-------------------------------------------------------------------------------------------------------------------------------------" >>$TMP_member_check_loop
    echo "    // ハンドラをチェック関数を使って挿入する(_token) ※POSTでは必須" >>$TMP_member_check_loop
    echo "    //-------------------------------------------------------------------------------------------------------------------------------------" >>$TMP_member_check_loop
    echo "    let _token = &postForm._token;" >>$TMP_member_check_loop
    echo "    if &server_info.post_token_id != _token && server_info.reqest_method == \"POST\" {" >>$TMP_member_check_loop
    echo "        valiback_detail.insert(\"_token\", \"トークンが一致しません。\");" >>$TMP_member_check_loop
    echo "        input_result.insert(String::from(\"Result\"), 5);" >>$TMP_member_check_loop
    echo "" >>$TMP_member_check_loop
    echo "    };" >>$TMP_member_check_loop
    echo "//-----------------------------------------------------------------------------------------------------------------------------------------" >>$TMP_struct_param
    echo "// 画面遷移別個別対応" >>$TMP_struct_param
    echo "// PathParam →Post 又は Get" >>$TMP_struct_param
    echo "//----------------------------------------------------------------------------------------------------------------------------------------" >>$TMP_struct_param
    echo "#[derive(seg4_common::Serialize, seg4_common::Deserialize)]" >>$TMP_struct_param
    echo "pub struct PostParam {" >>$TMP_struct_param
    echo "    _token: String," >>$TMP_struct_param
elif [ $reqest_method == "Get" ]; then

    if [ $menber_count -eq 0 ]; then
        reqest_method_form="_getForm: actix_web::web::Query<GetParam>,"
    else
        reqest_method_form="getForm: actix_web::web::Query<GetParam>,"
    fi

    #reqest_method_form="getForm: actix_web::web::Query<GetParam>,"
    reqest_struct_args="&getForm."
    echo "//-----------------------------------------------------------------------------------------------------------------------------------------" >>$TMP_struct_param
    echo "// 画面遷移別個別対応" >>$TMP_struct_param
    echo "// PathParam →Post 又は Get" >>$TMP_struct_param
    echo "//----------------------------------------------------------------------------------------------------------------------------------------" >>$TMP_struct_param
    echo "#[derive(seg4_common::Serialize, seg4_common::Deserialize)]" >>$TMP_struct_param
    echo "pub struct GetParam {" >>$TMP_struct_param
    if [ $bl_type == "pagenate" ]; then
        reqest_method_form="getForm: actix_web::web::Query<GetParam>,"
        BL_1_FILE="bl_1.txt"
        ACTION_2_FILE="action_2.txt"
        echo "    page: String," >>$TMP_struct_param
        echo "        let page = &getForm.page; //Getの場合" >>$TMP_member_check_loop
        echo "        input_params.insert(" >>$TMP_member_check_loop
        echo "            String::from(r\"page\")," >>$TMP_member_check_loop
        echo "            action_base::InputParametars::set_input_parametars(" >>$TMP_member_check_loop
        echo "                true,//必須項目。必須ならtrue" >>$TMP_member_check_loop
        echo "                page.to_string(),//ハンドラ文字列" >>$TMP_member_check_loop
        echo "                false,//文字列で扱うなら true それ以外なら false" >>$TMP_member_check_loop
        echo "                r\"数字のみ有効です。\".to_string(),//ヴァリテーションバック時のメッセージ文字列" >>$TMP_member_check_loop
        echo "                0,//最小値、文字列の場合は文字列数 -1で無視 " >>$TMP_member_check_loop
        echo "                -1,//最大値、文字列の場合は文字列数 -1で無視" >>$TMP_member_check_loop
        echo "                r\"^[0-9\.]*$\".to_string(),//正規表現チェック。境界値もこれで行う。全スルーは *" >>$TMP_member_check_loop
        echo "            )," >>$TMP_member_check_loop
        echo "        );" >>$TMP_member_check_loop
        echo "        let page = &input_params.get(\"page\").expect(\"Input Value Error[page].\").int_value;" >>$TMP_member_get_loop
    fi
else
    reqest_method_form="path: actix_web::web::Path<($path_string_args)>,"
    reqest_struct_args="&path."
fi
#------------------------------------------------------------------------------
# 個別部分(メンバ依存部分)_1
#------------------------------------------------------------------------------
#メンバの数だけ動的にループする処理
count=1
while [ $count -le $menber_count ]; do
    member_sqlline=$(exec_sql "select sql_id::text,member_name::text,member_type::text,required_check::text,max_limit::text,min_limit::text,valiback_message::text,check_regist::text,cast(sql_id + 2 as text) as paged_id,wherelike_flg::text from fascon_action_members inner join fascon_parent_action on fascon_parent_action.plimary=fascon_action_members.action_id where fascon_parent_action.action_name='$action_name' and fascon_action_members.sql_id=$count order by fascon_action_members.sql_id asc;")
    #------------------------------------------------------------------------------
    # ユーザに非公開である自分の会員IDを引数に追加 ※$1固定
    #------------------------------------------------------------------------------
    if [ "$member_sqlline" == "" ] && [ $count -eq 1 ];then
        #------------------------------------------------------------------------------
        # 各メンバのプリペアを定義
        #------------------------------------------------------------------------------
        sql_stmt_no_page="$sql_stmt_no_page,db_base::Type::INT4"
        #ページングあり
        sql_stmt_pages="$sql_stmt_pages,db_base::Type::INT4"
        #------------------------------------------------------------------------------
        # 各メンバの参照を定義
        #------------------------------------------------------------------------------
        #ページングなし
        sql_exec_no_page="$sql_exec_no_page,&_server_info.business_login_id"
        #ページングあり
        sql_exec_pages="$sql_exec_pages,&_server_info.business_login_id"
        count=$(expr $count + 1)
        continue
    fi
    sql_id=$(echo $member_sqlline | cut -d ";" -f 1)
    member_name=$(echo $member_sqlline | cut -d ";" -f 2)
    member_type=$(echo $member_sqlline | cut -d ";" -f 3)
    required_check=$(echo $member_sqlline | cut -d ";" -f 4)
    max_limit=$(echo $member_sqlline | cut -d ";" -f 5)
    min_limit=$(echo $member_sqlline | cut -d ";" -f 6)
    valiback_message=$(echo $member_sqlline | cut -d ";" -f 7)
    check_regist=$(echo $member_sqlline | cut -d ";" -f 8)
    paged_id=$(echo $member_sqlline | cut -d ";" -f 9)
    wherelike_flg=$(echo $member_sqlline | cut -d ";" -f 10)
    is_string=$(test $member_type == "String" && echo "true" || echo "false")
    if [ $member_type == "i64" ]; then
        get_type="int_value"
        sql_type=",db_base::Type::INT8"
    elif [ $member_type == "f64" ]; then
        get_type="float_value"
        sql_type=",db_base::Type::NUMERIC"
    else
        get_type="str_value"
        sql_type=",db_base::Type::TEXT"
    fi
    #StructParamの中身
    if [ $reqest_method == "Get" ] || [ $reqest_method == "Post" ]; then
        echo "    $member_name : String," >>$TMP_struct_param
        #member_check_loopの中身 ※Pathの場合は上部で定義済
        echo "    let $member_name = $reqest_struct_args$member_name; //$reqest_methodの場合" >>$TMP_member_check_loop
    fi
    #member_check_loopの中身 ※Pathの場合は上部で定義済
    echo "    input_params.insert(" >>$TMP_member_check_loop
    echo "        String::from(r\"$member_name\")," >>$TMP_member_check_loop
    echo "        action_base::InputParametars::set_input_parametars(" >>$TMP_member_check_loop
    echo "            $required_check,//必須項目。必須ならtrue" >>$TMP_member_check_loop
    echo "            $member_name.to_string(),//ハンドラ文字列" >>$TMP_member_check_loop
    echo "            $is_string,//文字列で扱うなら true それ以外なら false" >>$TMP_member_check_loop
    echo "            r\"$valiback_message\".to_string(),//ヴァリテーションバック時のメッセージ文字列" >>$TMP_member_check_loop
    echo "            $min_limit,//最小値、文字列の場合は文字列数 -1で無視 " >>$TMP_member_check_loop
    echo "            $max_limit,//最大値、文字列の場合は文字列数 -1で無視" >>$TMP_member_check_loop
    echo "            r\"$check_regist\".to_string(),//正規表現チェック。境界値もこれで行う。全スルーは *" >>$TMP_member_check_loop
    echo "        )," >>$TMP_member_check_loop
    echo "    );" >>$TMP_member_check_loop
    #member_get_loopの中身
    #where句がlikeの場合はフォーマット処理
    if [ $wherelike_flg == "true" ]; then
        echo -n "        let $member_name = " >>$TMP_member_get_loop
        printf 'format!("' >>$TMP_member_get_loop
        echo -n "%{}%\"," >>$TMP_member_get_loop
        echo "&input_params.get(\"$member_name\").expect(\"Input Value Error[$member_name].\").$get_type);" >>$TMP_member_get_loop
    else
        echo "        let $member_name = &input_params.get(\"$member_name\").expect(\"Input Value Error[$member_name].\").$get_type;" >>$TMP_member_get_loop
    fi
    #------------------------------------------------------------------------------
    # 各メンバのプリペアを定義
    #------------------------------------------------------------------------------
    sql_stmt_no_page="$sql_stmt_no_page$sql_type"
    #ページングあり
    sql_stmt_pages="$sql_stmt_pages$sql_type"
    #------------------------------------------------------------------------------
    # 各メンバの参照を定義
    #------------------------------------------------------------------------------
    #ページングなし
    sql_exec_no_page="$sql_exec_no_page,&$member_name"
    #ページングあり
    sql_exec_pages="$sql_exec_pages,&$member_name"

    count=$(expr $count + 1)
done

if [ $reqest_method == "Get" ] || [ $reqest_method == "Post" ]; then
    echo "}" >>$TMP_struct_param
fi
sql_stmt_no_page=$(echo $sql_stmt_no_page | cut -d "," -f2-)
sql_stmt_pages=$(echo $sql_stmt_pages | cut -d "," -f2-)

sql_exec_pages=$(echo $sql_exec_pages | cut -d "," -f2-)
sql_exec_no_page=$(echo $sql_exec_no_page | cut -d "," -f2-)

sql_exec_no_page=$(echo $sql_exec_no_page | sed 's@,@\\,@g' | sed 's@&@\\&@g')
sql_exec_pages=$(echo $sql_exec_pages | sed 's@,@\\,@g' | sed 's@&@\\&@g')

sql_stmt_no_page=$(echo $sql_stmt_no_page | sed 's@,@\\,@g' | sed 's@&@\\&@g')
sql_stmt_pages=$(echo $sql_stmt_pages | sed 's@,@\\,@g' | sed 's@&@\\&@g')

#------------------------------------------------------------------------------
# TMP_ACTION_FILEの生成
#------------------------------------------------------------------------------
#action_1
cat $JOBDIR/action_1.txt | sed "s/### ACTION ###/$action_name/g" | sed "s/### METHOD ###/$reqest_method/g" >>$TMP_ACTION_FILE
#struct_param
cat $TMP_struct_param >>$TMP_ACTION_FILE
#action_2
cat $JOBDIR/$ACTION_2_FILE | sed "s/### REQEST_METHOD_FORM ###/$reqest_method_form/g" | sed "s/### GUEST_ACCESS_ALLOW ###/$guest_access_allow/g" | sed "s/### UPDATE_EXISTS_ALLOW ###/$update_exists_allow/g" | sed "s/### ParhMemberItarete ###/$path_member_itarete/g" >>$TMP_ACTION_FILE
#member_loop
cat $TMP_member_check_loop >>$TMP_ACTION_FILE
#action_3
func_left=$(echo "seg4_common::for_template_outtext(\"$action_name/head\",\&format\!("|sed 's@\\!@!@g')
func_right=")),"

if [ $menber_count -eq 0 ];then
 cat << EOF >>$TMP_ACTION_FILE
    //-------------------------------------------------------------------------------------------------------------------------------------
    // 入力チェック結果を集計
    //-------------------------------------------------------------------------------------------------------------------------------------
    for (_key, _value) in &input_params {
    }
    valiback_detail.insert(String::from("Result"), 0);
EOF
else
cat << EOF >>$TMP_ACTION_FILE
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
EOF
fi

cat "$JOBDIR/action_3.txt" | sed "s/### ACTION ###/$action_name/g" >>$TMP_ACTION_FILE
if [ "$cgi_dynamic_head" != "" ]; then
    eval "sed -i 's@//### JSON OBJECT ###@let obj: serde_json::Value = serde_json::from_str(\&json).unwrap();@' $TMP_ACTION_FILE"
    eval "sed -i 's@### DYNAMIC HEAD ###@$func_left$cgi_dynamic_head$func_right@' $TMP_ACTION_FILE"
    if [ "$cgi_replace_body" != "" ]; then
        eval "sed -i 's@### REPLACE BODY ###@$cgi_replace_body@' $TMP_ACTION_FILE"
    else
        eval "sed -i 's@### REPLACE BODY ###@@' $TMP_ACTION_FILE"
    fi
else
    eval "sed -i 's@### DYNAMIC HEAD ###@seg4_common::fs::read_to_string(format!(\"{}/head\",template_path)).expect(\"FileLoading is Failed.\"),@' $TMP_ACTION_FILE"
    eval "sed -i 's@### REPLACE BODY ###@@' $TMP_ACTION_FILE"
fi

#------------------------------------------------------------------------------
# オリジナルよりmain.rs生成 要区切り行をメンテナンス
#------------------------------------------------------------------------------
#分割対象ファイル
main_rs_file="$JOBDIR/../src/main.rs"
#区切り行(下から数えて【//F.A.C.S ココマデ】の部分)
kugiri_row=8
#区切り行(上から数えて【//F.A.C.S ココカラ】の部分)
zenhan_row=45
#行数
main_rs_rows=$(wc -l $main_rs_file | cut -d " " -f1)
#関数
main_web=$(test $reqest_method == "Post" && echo "post" || echo "get")
#main.rs前半
eval "head -$zenhan_row $main_rs_file" >>$TMP_MAIN_FILE
#main.rs前半
zenhan_row=$(expr $main_rs_rows - $zenhan_row)
#eval "head -$zenhan_row $main_rs_file" >> $TMP_MAIN_FILE

for line_args in $(exec_sql "select reqest_uri,case when reqest_method <> 'Post' then 'Get' else 'Post' end as reqest_method,action_name from fascon_parent_action where action_name = '$action_name';"); do
    loop_reqest_uri=$(echo $line_args | cut -d ";" -f 1)
    loop_main_web=$(echo $line_args | cut -d ";" -f 2 | tr [:upper:] [:lower:])
    loop_action_name=$(echo $line_args | cut -d ";" -f 3)
    echo "            .route(\"$loop_reqest_uri\",web::$loop_main_web().to(controller::$loop_action_name::execute))" >>$TMP_MAIN_FILE
done
#echo "            .route(\"$reqest_uri\",web::$main_web().to(controller::$action_name::execute))" >> $TMP_MAIN_FILE

eval "tail -$zenhan_row $main_rs_file|grep -v \"$action_name\"" >>$TMP_MAIN_FILE

#------------------------------------------------------------------------------
# オリジナルよりcontroller/mod.rs生成
#------------------------------------------------------------------------------
#対象ファイル
controller_mod_rs_file="$JOBDIR/../src/controller/mod.rs"
cat $controller_mod_rs_file >>$TMP_CONTROLLER_MOD_RS
for line_args in $(exec_sql "select action_name,action_overview from fascon_parent_action where action_name = '$action_name';"); do
    loop_action_name=$(echo $line_args | cut -d ";" -f 1)
    loop_action_overview=$(echo $line_args | cut -d ";" -f 2)
    echo "pub mod $loop_action_name;//$loop_action_overview" >>$TMP_CONTROLLER_MOD_RS
done
#echo "pub mod $action_name;//$action_overview" >> $TMP_CONTROLLER_MOD_RS

#------------------------------------------------------------------------------
# TMP_BL_FILEの生成
#------------------------------------------------------------------------------
#bl_1
cat $JOBDIR/$BL_1_FILE >>$TMP_BL_FILE
#get_param
cat $TMP_member_get_loop >>$TMP_BL_FILE
#model(pagenate/loggingdb)
printf "        const table_name: &str = \"$bl_args_table_name\";\n" >>$TMP_BL_FILE
printf "        const where_args: &str = \"$bl_args_where\";\n" >>$TMP_BL_FILE
if [ $bl_type == "loggingdb" ]; then
    printf "        const QUERY_SQL: &str =\"with tmp_table as (\n" >>$TMP_BL_FILE
    printf "    $bl_args_sql_begin\n" >>$TMP_BL_FILE
    printf "        )\n" >>$TMP_BL_FILE
    if [ $bl_pagenate_outstyle == "csv" ]; then
        printf "            select '\\\\\"' || trim(concat_ws(',',tmp_table.*),'()') || '\\\\\"' as out from tmp_table;\n" >>$TMP_BL_FILE
    else
        printf "            select cast(to_json(tmp_table.*) as text) as out from tmp_table;\n" >>$TMP_BL_FILE
    fi
    printf "        \";\n" >>$TMP_BL_FILE
    printf "        const orderby_args: &str = \"$bl_args_orderby\";\n" >>$TMP_BL_FILE
    cat $JOBDIR/model_loggingsql.txt | sed "s/### sql_stmt_no_page ###/$sql_stmt_no_page/g" | sed "s/### sql_exec_no_page ###/$sql_exec_no_page/g" >>$TMP_BL_FILE
elif [ $bl_type == "pagenate" ]; then
    printf "        const QUERY_SQL: &str =\"with tmp_table as (\n" >>$TMP_BL_FILE
    printf "    $bl_args_sql_begin\n" >>$TMP_BL_FILE
    printf "        )\n" >>$TMP_BL_FILE
    if [ $bl_pagenate_outstyle == "csv" ]; then
        printf "            select '\\\\\"' || trim(concat_ws(',',tmp_table.*),'()') || '\\\\\"' as out from tmp_table;\n" >>$TMP_BL_FILE
    else
        printf "            select cast(to_json(tmp_table.*) as text) as out from tmp_table;\n" >>$TMP_BL_FILE
    fi
    printf "        \";\n" >>$TMP_BL_FILE
    printf "        let page_in: i32 = *$bl_args_page_in as i32;\n" >>$TMP_BL_FILE
    printf "        let content: i32 = $bl_args_content;\n" >>$TMP_BL_FILE
    printf "        let LIMIT_OFFSET = format!(\"limit {} offset {}\",content,content * (page_in - 1));\n" >>$TMP_BL_FILE
    printf "        const orderby_args: &str = \"$bl_args_orderby\";\n" >>$TMP_BL_FILE
    if [ "$bl_args_page_in" == "" ] || [ $bl_args_content -lt 1 ]; then
        cat $JOBDIR/model_pagenate.txt | sed "s/### sql_stmt_pages ###/$sql_stmt_no_page/g" | sed "s/### sql_exec_pages ###/$sql_exec_no_page/g" | sed "s/### sql_stmt_no_page ###/$sql_stmt_no_page/g" | sed "s/### sql_exec_no_page ###/$sql_exec_no_page/g" >>$TMP_BL_FILE
    else
        cat $JOBDIR/model_pagenate.txt | sed "s/### sql_stmt_pages ###/$sql_stmt_pages/g" | sed "s/### sql_exec_pages ###/$sql_exec_pages/g" | sed "s/### sql_stmt_no_page ###/$sql_stmt_no_page/g" | sed "s/### sql_exec_no_page ###/$sql_exec_no_page/g" >>$TMP_BL_FILE
    fi
fi
#bl_2
cat $JOBDIR/bl_2.txt >>$TMP_BL_FILE

#------------------------------------------------------------------------------
# オリジナルよりbusiness_logic/mod.rs生成
#------------------------------------------------------------------------------
#対象ファイル
business_logic_mod_rs_file="$JOBDIR/../src/business_logic/mod.rs"
cat $business_logic_mod_rs_file >>$TMP_BUSINESS_LOGIC_MOD_RS
for line_args in $(exec_sql "select action_name,action_overview from fascon_parent_action where action_name = '$action_name';"); do
    loop_action_name=$(echo $line_args | cut -d ";" -f 1)
    loop_action_overview=$(echo $line_args | cut -d ";" -f 2)
    echo "pub mod $loop_action_name;//$loop_action_overview" >>$TMP_BUSINESS_LOGIC_MOD_RS
done
#echo "pub mod $action_name;//$action_overview" >> $TMP_BUSINESS_LOGIC_MOD_RS

#------------------------------------------------------------------------------
# 生成ファイルの反映
#------------------------------------------------------------------------------
if [ $(cat $JOBDIR/../src/main.rs | grep -c $action_name) -eq 0 ]; then
    mkdir -p $JOBDIR/../resorce/html_template/$action_name
    mkdir -p $JOBDIR/../resorce/mail_template/$action_name
    touch $JOBDIR/../resorce/html_template/$action_name/body
    touch $JOBDIR/../resorce/html_template/$action_name/head
    touch $JOBDIR/../resorce/html_template/$action_name/read_module
    cp -p $TMP_CONTROLLER_MOD_RS $JOBDIR/../src/controller/mod.rs
    cp -p $TMP_BUSINESS_LOGIC_MOD_RS $JOBDIR/../src/business_logic/mod.rs
fi
cp -p $TMP_MAIN_FILE $JOBDIR/../src/main.rs
cp -p $TMP_ACTION_FILE $JOBDIR/../src/controller/$action_name.rs
cp -p $TMP_BL_FILE $JOBDIR/../src/business_logic/$action_name.rs
echo "▼ Complete."

exit 0
