<p align="center"><img src="https://d3fy1q62gxauop.cloudfront.net/media/images/rectsql-logo.png" width="800" alt="ReactSQL"></p>
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
$ service postgresql start<br>
$ service nginx start<br>
$ su - www-data<br>
$ cd ./fasbone && cargo check<br>
$ printf "{IMAGE ID}" > ./public_html/js/apihost<br>
$ cargo run<br>
------ Docker Base ------<br>
$ echo "{CONTAINER IP Adress} {IMAGE ID}" >> /etc/hosts<br>
<h3>Let's access it with a browser!<code>http://{IMAGE ID}</code></h3>





