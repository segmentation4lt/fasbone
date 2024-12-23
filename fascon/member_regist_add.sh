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
#テーブル名 存在を確認
table_name=$1
if [ $(exec_sql "select count(*) from (select * from information_schema.tables where table_schema = 'public') t left join (select * from information_schema.columns where table_schema = 'public') c on t.table_name = c.table_name where t.table_name = '$table_name' and c.column_name = 'auth_prefix';") -eq 0 ]; then
    echo "No such table or No such column[auth_id](integer)."
    exit 9
fi

#一時保存コントローラーファイル
TMP_MAIN1_FILE=/tmp/make_resist1_rs.tmp
[ -f $TMP_MAIN1_FILE ] && rm -f $TMP_MAIN1_FILE
touch $TMP_MAIN1_FILE
TMP_MAIN2_FILE=/tmp/make_resist2_rs.tmp
[ -f $TMP_MAIN2_FILE ] && rm -f $TMP_MAIN2_FILE
touch $TMP_MAIN2_FILE

#一時保存ビジネスロジックファイル
TMP_BL_FILE=/tmp/make_resist3_rs.tmp
[ -f $TMP_BL_FILE ] && rm -f $TMP_BL_FILE
touch $TMP_BL_FILE

#雛型
RESIST_CONTROLLER_FILE=$JOBDIR/membership_resist_controller.txt
RESIST_BUSINESS_LOGIC_FILE=$JOBDIR/membership_resist_business_logic.txt

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

reqest_method="post"
reqest_struct_args="&postForm."

#ページングなし
sql_stmt_no_page=""
sql_exec_no_page=""

inser_column_name=""
inser_sql_no=""
doru="$"

sql_id=1
for member_sqlline in $(exec_sql "select t.table_name, c.column_name,replace(c.data_type,' ','_') as data_type ,  c.is_nullable  from (select * from information_schema.tables where table_schema = 'public') t left join (select * from information_schema.columns where table_schema = 'public') c on t.table_name = c.table_name where t.table_name = '$table_name'  and c.column_name != 'plimary' and c.column_name != 'auth_prefix' order by t.table_name, c.ordinal_position;"); do
    member_name=$(echo $member_sqlline | cut -d ";" -f 2)
    member_type=$(echo $member_sqlline | cut -d ";" -f 3)
    member_nulltable=$(echo $member_sqlline | cut -d ";" -f 4)
    required_check=$(test $member_nulltable == "NO" && echo "true" || echo "false")
    is_string=$(test $member_type == "character_varying" && echo "true" || echo "false")
    if [ $member_type == "integer" ]; then
        get_type="int_value"
        sql_type=",db_base::Type::INT8"
    elif [ $member_type == "numeric" ]; then
        get_type="float_value"
        sql_type=",db_base::Type::NUMERIC"
    else
        get_type="str_value"
        sql_type=",db_base::Type::TEXT"
    fi

    #StructParamの中身
    echo "    $member_name : String," >>$TMP_struct_param
    #member_check_loopの中身 ※Pathの場合は上部で定義済
    echo "    let $member_name = $reqest_struct_args$member_name; //$reqest_methodの場合" >>$TMP_member_check_loop
    #member_check_loopの中身 ※Pathの場合は上部で定義済
    echo "    input_params.insert(" >>$TMP_member_check_loop
    echo "        String::from(r\"$member_name\")," >>$TMP_member_check_loop
    echo "        action_base::InputParametars::set_input_parametars(" >>$TMP_member_check_loop
    echo "            $required_check,//必須項目。必須ならtrue" >>$TMP_member_check_loop
    echo "            $member_name.to_string(),//ハンドラ文字列" >>$TMP_member_check_loop
    echo "            $is_string,//文字列で扱うなら true それ以外なら false" >>$TMP_member_check_loop
    echo "            r\"\".to_string(),//ヴァリテーションバック時のメッセージ文字列" >>$TMP_member_check_loop
    echo "            -1,//最小値、文字列の場合は文字列数 -1で無視 " >>$TMP_member_check_loop
    echo "            -1,//最大値、文字列の場合は文字列数 -1で無視" >>$TMP_member_check_loop
    echo "            r\"*\".to_string(),//正規表現チェック。境界値もこれで行う。全スルーは *" >>$TMP_member_check_loop
    echo "        )," >>$TMP_member_check_loop
    echo "    );" >>$TMP_member_check_loop

    #TMP_member_get_loopの中身
    echo "        let $member_name = &input_params.get(\"$member_name\").expect(\"Input Value Error[$member_name].\").$get_type;" >>$TMP_member_get_loop

    #------------------------------------------------------------------------------
    # 各メンバのプリペアを定義
    #------------------------------------------------------------------------------
    sql_stmt_no_page="$sql_stmt_no_page$sql_type"

    #------------------------------------------------------------------------------
    # 各メンバの参照を定義
    #------------------------------------------------------------------------------
    sql_exec_no_page="$sql_exec_no_page,&$member_name"

    inser_sql_no="$inser_sql_no,$doru$sql_id"
    sql_id=$(expr $sql_id + 1)
done
inser_sql_no="$inser_sql_no,$doru$sql_id"
sql_exec_no_page="$sql_exec_no_page,&userid"
sql_stmt_no_page="$sql_stmt_no_page,db_base::Type::TEXT"

sql_stmt_no_page=$(echo $sql_stmt_no_page | cut -d "," -f2-)
sql_exec_no_page=$(echo $sql_exec_no_page | cut -d "," -f2-)

sql_exec_no_page=$(echo $sql_exec_no_page | sed 's@,@\\,@g' | sed 's@&@\\&@g')
sql_stmt_no_page=$(echo $sql_stmt_no_page | sed 's@,@\\,@g' | sed 's@&@\\&@g')

head -48 $RESIST_CONTROLLER_FILE >>$TMP_MAIN1_FILE
cat $TMP_struct_param >>$TMP_MAIN1_FILE
tail -183 $RESIST_CONTROLLER_FILE >>$TMP_MAIN1_FILE
resist_rs_rows=$(expr $(wc -l $TMP_MAIN1_FILE | cut -d " " -f1) - 70)
head -$resist_rs_rows $TMP_MAIN1_FILE >>$TMP_MAIN2_FILE
cat $TMP_member_check_loop >>$TMP_MAIN2_FILE
tail -70 $TMP_MAIN1_FILE >>$TMP_MAIN2_FILE
cp -p $TMP_MAIN2_FILE $JOBDIR/../src/controller/membership_resist.rs

inser_sql_no=$(echo $inser_sql_no | cut -d "," -f2-)
insert_columns=$(echo $sql_exec_no_page | sed 's@&@@g' | sed 's@\\@@g' | sed 's@,userid@@g')
bl_args_sql_begin=$(echo "insert into $table_name($insert_columns,auth_prefix) values ($inser_sql_no) returning auth_prefix;")

sql_exec_no_page=$(echo $sql_exec_no_page | sed 's@,@\,@g' | sed 's@&@\&@g')
head -127 $JOBDIR/../src/business_logic/membership_resist.rs >>$TMP_BL_FILE
cat $TMP_member_get_loop >>$TMP_BL_FILE
printf "        const table_name: &str = \"$table_name\";\n" >>$TMP_BL_FILE
printf "        const where_args: &str = \"\";\n" >>$TMP_BL_FILE
printf "        const QUERY_SQL: &str = \"$bl_args_sql_begin\";\n" >>$TMP_BL_FILE
tail -17 $JOBDIR/model_loggingsql.txt | head -9 | sed "s/### sql_stmt_no_page ###/$sql_stmt_no_page/g" | sed "s/### sql_exec_no_page ###/$sql_exec_no_page/g" >>$TMP_BL_FILE
tail -20 $RESIST_BUSINESS_LOGIC_FILE >>$TMP_BL_FILE
cat $TMP_BL_FILE | sed 's@let query_execute_query@let _query_execute_query@g' >$JOBDIR/../src/business_logic/membership_resist.rs
