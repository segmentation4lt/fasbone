/*
actix関連のmultipart処理が機能しない為、自力で実装。

セットHTMLは下記参照
<body>
 <div id="container">
<form method="post" action="/json/api/upload/" enctype="multipart/form-data">
<p>幅は参照ボタン</p>
<input type="file" name="form_filename">
<input type="hidden" name="_token">
<p><input type="submit" value="送信する" disabled></p>
</form>

<div>
	<form method="post" action="/json/api/member/cert/" method="POST">
	    <lavel>ログインID(メールアドレス)</lavel>
		<div>
			<input type="text" name="userid" placeholder="メールアドレス">
		</div>
	    <lavel>パスワード</lavel>
		<div>
			<input type="password" name="passwd" placeholder="パスワード">
		</div>
		<input type="hidden" name="_token">
		<p><input type="submit" value="送信する" disabled></p>
	</form>
</div>
<p><a href="/private/view/test/9/8a2c857d-9f6d-4740-be78-3f38e88fc0d1_1693247869.png">file_download</a></p>
</div>
<script type="text/javascript" charset="UTF-8">
$(document).ready(function() {
    //-----------------------------------------------------//
    // UUID
    //-----------------------------------------------------//
    $.ajax({
        url: 'http://192.168.23.13/json/api/',
        timeout: 10000,
        async:true,
        type:'GET',
    })
    .always(function(json) {
    console.log(json);
		//$('form[action="/json/api/upload/"]').find('input[name="_token"]').val(json.token);
		//$('form[action="/json/api/upload/"]').find('input[type="submit"]').prop('disabled', false);
		$('input[name="_token"]').val(json.token);
		$('input[type="submit"]').prop('disabled', false);
    });
});



</script>
</body>
*/

#![allow(non_snake_case)]
//-----------------------------------------------------------------------------------------------------------------------------------------
// actix_webは取り込むモジュールが異なるので各個呼び出し ※要検証
//-----------------------------------------------------------------------------------------------------------------------------------------
use actix_web::{HttpRequest, HttpResponse,ResponseError};
use thiserror::Error;

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
//  PAYLOAD
//-----------------------------------------------------------------------------------------------------------------------------------------
use futures_util::StreamExt as _;
use std::io::prelude::*;
use std::fs::{File,create_dir_all};

//-----------------------------------------------------------------------------------------------------------------------------------------
// ResponseError のラッパー宣言。独自のエラー処理に使用
//-----------------------------------------------------------------------------------------------------------------------------------------
#[derive(Error, Debug)]
pub enum MyError {}
impl ResponseError for MyError {}

/**
*  引数での構造体宣言
* 空はString固定。GET、POSTごとにハンドラー名を分解する
*/
 /*   ここから非共通部   */
//-----------------------------------------------------------------------------------------------------------------------------------------
// 画面遷移別個別対応
//-----------------------------------------------------------------------------------------------------------------------------------------

/*   非共通部 ココマデ  */


/**
*  引数での構造体宣言
* //※GETの場合    <ハンドラ名>_struct:web::get<<ハンドラ名>Param>
* //※POSTの場合    <ハンドラ名>_struct:web::get<<ハンドラ名>Param>
*  最後はHashMapが作成されてそれを使用するので命名規則がカオスでもよい。
* 手動で書くのは大変なので、ツールで生成するようにする
*/
//-----------------------------------------------------------------------------------------------------------------------------------------
// execute 処理開始
//-----------------------------------------------------------------------------------------------------------------------------------------
pub async fn execute(
    mut body: actix_web::web::Payload,
    req: HttpRequest,
) -> Result<HttpResponse, MyError> {

    //-------------------------------------------------------------------------------------------------------------------------------------
    // コントローラの先頭でDBインスタンスを確立
    //-------------------------------------------------------------------------------------------------------------------------------------
    let mut pg_client = db_base::db_connect();

    //-------------------------------------------------------------------------------------------------------------------------------------
    // サーバ関連の初期化。
    //-------------------------------------------------------------------------------------------------------------------------------------
    let server_info = action_base::ServerInfomation::set_server_infomation(req,&mut pg_client);

    //-------------------------------------------------------------------------------------------------------------------------------------
    // マルチパートでないか、容量オーバーの場合処理をしないで終了。
    //-------------------------------------------------------------------------------------------------------------------------------------
    if server_info.http_content_type.contains("multipart") == false && seg4_common::define::MULTIPART_MAX_BYTE > 
        server_info.http_content_length.parse().expect("変換できない文字列でした")  {
        seg4_common::info!("● Reqest is Inappropriate. or over of capacity. {}",serde_json::to_string(&server_info).unwrap());
        return Ok(HttpResponse::Forbidden()
        .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
        .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
        .header("Set-Cookie", server_info.cookie_line)
        .body(seg4_common::PARAM_ERROR))
    };

    //-------------------------------------------------------------------------------------------------------------------------------------
    // Payloadをバイト配列に変換
    //-------------------------------------------------------------------------------------------------------------------------------------
    let mut bytes = actix_web::web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }
    //キャラクタ変換
    let mut chars: Vec<char> = Vec::new();
    for binary_args in &bytes {
        chars.push(char::from_u32((*binary_args).into()).unwrap());
    }

    //-------------------------------------------------------------------------------------------------------------------------------------
    // バイト配列に変換したPayloadをループ(一回目:チェック用に各種値をs取得)
    //-------------------------------------------------------------------------------------------------------------------------------------
    //初期設定
    let mut bytes_connt  = 0;//バイト配列のカウント
    let content_type_b = actix_web::web::BytesMut::from(&b"Content-Type:"[..]);//「Content-Type:」文字列比較用※1バイトづつ比較する
    let token_b = actix_web::web::BytesMut::from(&b"name=\"_token\""[..]);//「_token」文字列比較用※1バイトづつ比較する
    let mut content_type_value = String::from("");//「Content-Type:」の値。拡張子を取得するのに使用する。
    let mut token_value = String::from("");//「_token」の値。post_token_idの整合性を確認するのに使用する。
    let mut file_put_start = 0;//ファイルに出力するガチバイナリの先頭位置
    let mut file_put_end = 0;//ファイルに出力するガチバイナリの終了位置

    //bytesループ
    for _value in &bytes {
        //---------------------------------------------------------------------------------------------------------------------------------
        // 「_token」の値を算出
        //---------------------------------------------------------------------------------------------------------------------------------
        let mut token_connt  = 0;//内部カウント
        //値の生成後は処理をしない
        if token_value == "" {
            for value3 in &token_b {
                if bytes.len() > bytes_connt + token_connt && bytes[bytes_connt+token_connt] == (*value3) {
                    token_connt+=1;
                }; 
            }
        }else{
            token_connt=0;
        };

        let mut token_value_start = bytes_connt+token_connt;
        //「_token」文字列が合致したらを_tokenの値を取得
        if token_connt == token_b.len() {
            loop {
                if ( chars[token_value_start] != '\r' && chars[token_value_start] != '\n' ) || 
                    token_value_start > bytes.len() { break; } //改行部分以外まで処理を実施
                token_value_start+=1;//インクリメント
            }
            loop {
                 if ( chars[token_value_start] == '\r' || chars[token_value_start] == '\n' ) || 
                     token_value_start > bytes.len() { break; } //改行部分まで処理を実施
                 //STRINGオブジェクトにPUSHする
                 token_value.push(chars[token_value_start]);
                 token_value_start+=1;//インクリメント
             }//loop
        }//文字列合致

        //---------------------------------------------------------------------------------------------------------------------------------
        // アップロードファイルの位置を算出
        //---------------------------------------------------------------------------------------------------------------------------------
        let mut content_type_connt  = 0;//内部カウント
        //値の生成後は処理をしない
        if content_type_value == "" {
            for value2 in &content_type_b {
                if bytes.len() > bytes_connt +content_type_connt && bytes[bytes_connt+content_type_connt] == (*value2) {
                    content_type_connt+=1;
                }; 
            }
        }else{
            content_type_connt=0;
        };

        let mut content_type_value_start = bytes_connt + content_type_connt;
        //「Content-Type:」文字列が合致したらContent-Typeの値(STRING)を取得
        if content_type_connt == content_type_b.len() {
           loop {
                if ( chars[content_type_value_start] == '\r' || chars[content_type_value_start] == '\n' ) || 
                    content_type_value_start > bytes.len() { break; } //改行部分まで処理を実施
                //STRINGオブジェクトにPUSHする
                content_type_value.push(chars[content_type_value_start]);
                content_type_value_start+=1;//インクリメント
            }//loop
            //ファイルに出力するガチバイナリの先頭位置
            file_put_start = content_type_value_start;
            loop {
                if ( chars[file_put_start] != '\r' && chars[file_put_start] != '\n' ) || 
                    file_put_start > bytes.len() { break; } //改行部分以外まで処理を実施
                file_put_start+=1;//インクリメント
            }
            file_put_end = file_put_start;
            loop {
                if (( chars[file_put_end] == '\r' || chars[file_put_end] == '\n' ) || 
                file_put_end > bytes.len()) && (chars[file_put_end + 2] == '-' && chars[file_put_end + 3] == '-' && 
                    chars[file_put_end + 4] == '-' && chars[file_put_end + 5] == '-' && chars[file_put_end + 6] == '-'
                ) { 
                    //file_put_end-=1;//改行コードまで差しかかっているのでデクリメント
                    break; 
                } //改行部分以外まで処理を実施
                file_put_end+=1;//インクリメント
            }
        };//文字列合致
        bytes_connt+=1;//インクリメント
    }//bytesループ

    //-------------------------------------------------------------------------------------------------------------------------------------
    // 不正アクセスのチェック。post_token_idと取得したtoken_valueを比較する
    //-------------------------------------------------------------------------------------------------------------------------------------
    if server_info.post_token_id != token_value || server_info.business_login_id < 0 || content_type_value.contains("application/octet-stream") == true {
        //エラーログ
        seg4_common::info!("● UUID is Noting. {}",serde_json::to_string(&server_info).unwrap());
        return Ok(HttpResponse::Forbidden()
        .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
        .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
        .header("Set-Cookie", server_info.cookie_line)
        .body(seg4_common::PARAM_ERROR))
    };

    //-------------------------------------------------------------------------------------------------------------------------------------
    // ファイル名等の環境を調整してバイナリをファイルに書き込む
    //-------------------------------------------------------------------------------------------------------------------------------------
    create_dir_all(seg4_common::define::FILE_UPLOAD_TMPDIR).unwrap();
    let save_filename = format!("{}_{}.{}",token_value ,server_info.timestamp ,seg4_common::contenttype_to_extnsis(&content_type_value));
    let mut buffer = File::create(format!("{}/{}",seg4_common::define::FILE_UPLOAD_TMPDIR , save_filename)).unwrap();
    for pos in file_put_start..file_put_end {
        buffer.write(&[bytes[pos]]).expect("File Output Failed.");
    }

    //-------------------------------------------------------------------------------------------------------------------------------------
    //  正常終了時のJSON出力
    //-------------------------------------------------------------------------------------------------------------------------------------
    Ok(HttpResponse::Ok()
    .header("Content-Type", seg4_common::HTTP_CONTENT_TYPE)
    .header("Cache-Control", seg4_common::HTTP_CACHE_CONTROL)
    .header("Set-Cookie", server_info.cookie_line)
    .body(format!("{{ \"result\":\"200 FileUpload Complete.\",\"server_filename\": {} }}", save_filename)))
} //execute 終端
