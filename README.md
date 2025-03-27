<p align="center"><img src="https://d3fy1q62gxauop.cloudfront.net/media/images/bone-logo_2.png" width="400" alt="FasBone"></p>
<p align="center"><a href="https://d3fy1q62gxauop.cloudfront.net/">Sample site combining SPA and SSG in a free serverless hosting environment created with FasBone</a></p>

<h1>Quick Start</h1>
<h2>$←command prompt</h2>

$ docker pull segmentation4lt/fasbone:latest <br>
$ docker run -it -p 80:80 \`docker images|grep fasbone|awk '{print $3}'\`  /bin/bash<br>
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

・FasBone は、JAMstack を意識したプレーン JavaScript 構成のフロントエンド/バックエンド API 一体型の動的 Web サイトジェネレーターです。<br>
・バックエンド開発では、定義済みの SQL からRustのソースコードを自動生成します。<br>
・フロントエンドは Backbone.js の後継という位置付けでWebAssembly を使用し、CGI/SSG/CSR をサポートしています。<br>
・Amazon CloudFront および Google Cloud Run と親和性の高い構成となっており、代替 JSON ファイルによる API のコールドスタンバイを実装しています。<br>
・タスクランナーはありません。AltJS や SCSS のトランスパイルは別途実施する必要がありますが、タスクランナーのホットリロードによるストレスから解放されます。<br>
・React や jQuery などの他のライブラリと組み合わせて運用するのも容易な構成となっています。<br>
<p>インストール方法、概要の説明等詳細は
<a href="https://github.com/segmentation4lt/fasbone/wiki/">wiki</a>を参照下さい</p>


FasBone is a dynamic website generator that integrates a frontend/backend API with a plain JavaScript structure that is designed with JAMstack in mind.
FasBone is a dynamic website generator with integrated frontend and backend APIs, built on plain JavaScript with JAMstack in mind.
The frontend is positioned as the successor to Backbone.js, using WebAssembly and supporting CGI/SSG/CSR.
It is highly compatible with Amazon CloudFront and Google Cloud Run, and implements cold standby for APIs using alternative JSON files.
There is no task runner; AltJS and SCSS transpiles must be performed separately, but this frees you from the stress of hot reloading the task runner.
It is also easy to use in combination with other libraries such as React and jQuery.

<p>If you have any questions or problems, <a href="https://github.com/segmentation4lt/fasbone/issues">please write here</a>.</p>

<hr>
<h2>Usage environment</h2>
Front-end: underscore.js, webassembly<br>
Backend: Rust<br>
Command tools: bash<br>
Database: Postgresql<br>
Web server: nginx<br>

