<p align="center"><img src="https://d3fy1q62gxauop.cloudfront.net/media/images/bone-logo_2.png" width="400" alt="FasBone"></p>
<p align="center"><a href="https://d3fy1q62gxauop.cloudfront.net/">Sample SPA website created with FasBone</a><code>docker pull segmentation4lt/fasbone:latest</code></p>

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

・FasBoneは、定義済みのSQLからバックエンドAPIを自動生成する軽量なウェブ開発手法です。<br>
・フロントエンドはBackbone.JSの後継という位置付けでWebAssemblyを使用し、CGI、SSG、CSRをサポートしています。<br>
・Amazon CloudFrontと親和性の高い構成となっており、静的JSONファイルによるAPIのコールドスタンバイを実装しています。<br>
・タスクランナーとSSRはありません。AltJS/CSSのトランスパイルは別途実施する必要がありますが、開発中のホットリロードによるストレスから解放されます。<br>
・ReactやJquery等他のライブラリと組み合わせて運用するのも容易な構成となっております。<br>

<p>インストール方法、概要の説明等詳細は
<a href="https://github.com/segmentation4lt/fasbone/wiki/">wiki</a>を参照下さい</p>


FasBone is a lightweight web development methodology that automatically generates backend APIs from predefined SQL.<br>
The front end uses WebAssembly, the successor to Backbone.JS, and supports CGI, SSG, and CSR.<br>
It is highly compatible with Amazon CloudFront and implements cold standby for APIs using static JSON files.<br>
There is no task runner or SSR, and AltJS/CSS transpiling needs to be done separately, but it frees you from the stress of hot reloading during development.<br>
It is also easy to combine with other libraries such as React and Jquery.<br>
<p>If you have any questions or problems, <a href="https://github.com/segmentation4lt/fasbone/issues">please write here</a>.</p>

<hr>
<h2>Usage environment</h2>
Front-end: underscore.js, webassembly<br>
Backend: Rust<br>
Command tools: bash<br>
Database: Postgresql<br>
Web server: nginx<br>

