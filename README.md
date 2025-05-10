<p align="center"><img src="https://d3fy1q62gxauop.cloudfront.net/media/images/rectsql-logo2.png" width="800" alt="ReactSQL"></p>
<p>当該リポジトリ「FasBone」は"既存のレンダリングエンジン"をunderscore.jsからreactに変更しリポジトリ名を「ReactSQL」に変更致しました。
後方互換性は担保されており、レンダリングエンジンをunderscore.jsとreactを併用する事も可能です。
詳細な資料については現在制作中で進展状況を随時READMEを更新いたします。
今後共何卒宜しくお願い致します。

旧合同会社 セグメンテーション・フォルト


The repository “「FasBone」 has changed the “existing rendering engine from underscore.js to react and renamed the repository “「ReactSQL」.
Backward compatibility is ensured, and it is possible to use both the underscore.js and react rendering engines.
Detailed documentation is currently under construction. We will update the README as soon as possible.
Thank you for your continued support.

Formerly Segmentation Fault, LLC
</p>


<h1>Quick Start</h1>
<h2>$←command prompt</h2>

$ docker pull segmentation4lt/reactsql:latest <br>
$ docker run -it -p 80:80 \`docker images|grep reactsql|awk '{print $3}'\`  /bin/bash<br>
------ Inside Docker ------<br>
$ C_IP={OUTSIDE NODE'S IP ADRESS}<br>
$ sed -i "s/OUTSIDE_NODE_IP_ADRESS/$C_IP/g" /etc/nginx/sites-available/reactsql<br>
$ echo $C_IP > /var/www/reactsql/public_html/js/apihost<br>
$ service postgresql start<br>
$ service nginx start<br>
$ su - www-data<br>
$ cd ./reactsql && cargo check<br>
$ cargo run<br>
------ Docker Base ------<br>
<h3>Let's access it with a browser!<code>http://{CONTAINER IP}</code></h3>

<h2>「ReactSQL」(react実装) 後の変更点とreactの扱い方について</h2>
<h3>Changes after “ReactSQL” (react implementation) and how to handle react</h3>
<pre>
・CSR：新しく追加された引数にreactかbone(underscore.js)を指定して下さい。
$ cd [puoject folder]
$ ./fascon/make_bone.sh [reqest uri] [any Action name] [default load api url] [react or bone]
・SSR/SSG：新しく追加された引数にreactかbone(underscore.js)を指定して下さい。
$ cd [puoject folder]
$ ./fascon/make_bone.sh [any Action name]  [react or bone]
※何れも  [react or bone] に指定が無い場合は reactが指定されます。

②reactの扱い方
・CSR：TSXファイル置き場として「esbuild_src」フォルダ配下に「component」「pages」フォルダを設置してあります。
TSXファイルを編集後手動でトランスパイルを実行して下さい。トランスパイル後の出力ファイル名は
./public_html/js/action/[any Action name].js となります。
$ cd [puoject folder]
$ node esbuild.config.js
・SSR/SSG：bone(underscore.js)同様に./resorce/html_template/[any Action name]フォルダ内に設置してあるbody.tsxファイルにて
TSXファイルを編集します。こちらはリクエスト毎にwebサーバがトランスパイルを実行します。
・用意されている雛型から編集して下さい。bone(underscore.js)同様に初期レンダリングの対象オブジェクトは<body>タグにして下さい。
※rect/bone(underscore.js)共に初期レンダリング時の<body>タグの挙動は書き換え、SSR/SSGの場合は
「read_module」の内容が下方追記されます。

③タスクランナーとモジュールのビルドについて
当該リポジトリはNext.jsと違いindex.htmlの編集はユーザの責務という方針に従い
タスクランナーやモジュールのビルドについても自動化の範囲外とします。
</pre>

<pre>
1) Routing added
CSR: Specify react or bone(underscore.js) as the newly added argument.
$ cd [puoject folder]
$ ./fascon/make_bone.sh [reqest uri] [any Action name] [default load api url] [react or bone]
SSR/SSG: Please specify react or bone (underscore.js) for the newly added argument.
$ cd [puoject folder]
$ ./fascon/make_bone.sh [any Action name] [react or bone] $ .
*If [any Action name] [react or bone] is not specified, react is specified.

(2) How to handle react
CSR: “component” and “pages” folders are placed under the “esbuild_src” folder as TSX file storage.
After editing TSX files, please execute transpile manually. The output file name after transpiling is
./public_html/js/action/[any Action name].js.
$ cd [puoject folder].
$ node esbuild.config.js
・SSR/SSG: bone(underscore.js) as well as . In the body.tsx file located in the /resorce/html_template/[any Action name] folder
Edit the TSX file. The web server will execute a transpile for each request.
As with bone(underscore.js), the <body> tag should be the target object for initial rendering.
In both rect/bone(underscore.js) and SSR/SSG, the behavior of <body> tag at initial rendering should be rewritten.
In the case of SSR/SSG, the contents of “read_module” will be added downward.

3) Task Runner and Module Builds
Unlike Next.js, this repository follows the policy that editing index.html is the responsibility of the user.
Task runners and module builds are also outside the scope of automation.

</pre>



