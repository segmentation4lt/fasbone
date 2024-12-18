#!/bin/bash
#------------------------------------------------------------------------------
# 初期設定
#------------------------------------------------------------------------------
#コマンドのパス
export PATH="/usr/sbin:/usr/bin:/usr/local/bin:/usr/local/sbin:/sbin:/bin:/usr/X11/bin"
#カレントディレクトリ
JOBNAME=$(basename $0) >/dev/null 2>&1
JOBDIR=$(echo $0 | sed "s/$JOBNAME//g") >/dev/null 2>&1
[ "$JOBDIR" = "./" ] && JOBDIR=$(pwd)
#プロジェクト名の取得
project_name=$(basename $(echo $JOBDIR | sed s'@/fascon@@g'))
#一時保存ルーティングファイル
TMP_ROUTE_FILE=/tmp/make_route_json.tmp
[ -f $TMP_ROUTE_FILE ] && rm -f $TMP_ROUTE_FILE

IFS=$'\n'
if [[ "$1" == "" ]] || [[ "$2" == "" ]] || [[ "$3" == "" ]]; then
    echo "usage:[uri] [action name] [json reqest path]"
    exit 5
fi
echo "{" >>$TMP_ROUTE_FILE
for line_args in $(cat $JOBDIR/../public_html/json/static/route.json); do
    if [[ "$line_args" != "{" ]] && [[ "$line_args" != "}" ]]; then
        echo $line_args >>$TMP_ROUTE_FILE
    fi
done
printf ",\n    \"$1\":{\"action_name\":\"$2\",     \"api_urlandpath\":\"$3\"}" >>$TMP_ROUTE_FILE
echo "}" >>$TMP_ROUTE_FILE
cat $TMP_ROUTE_FILE | sed -z 's/\}\n\,/\}\,/g' | sed -z 's/\}\}/\}\n\}/g' >$JOBDIR/../public_html/json/static/route.json
cp -p ../public_html/js/action/404.js ../public_html/js/action/$2.js
touch $JOBDIR/../public_html/template/action/$2.template
