//-----------------------------------------------------------------------------------------------------------------------------------------
// 外部モジュールの読み込み ※クレートにする必要が薄いのでソースで管理
// 全体で一回だけ
//-----------------------------------------------------------------------------------------------------------------------------------------
mod resorce_module; //定数及び、ログとかDBとかのモジュール類 
mod base;//ベース
mod controller;//コントローラ
mod business_logic;//ビジネスロジック

/* use は相対で指定できない場合は crateをトップにして絶対パスを指定 */
//-----------------------------------------------------------------------------------------------------------------------------------------
// actix_web モジュール
//-----------------------------------------------------------------------------------------------------------------------------------------
use actix_web::{web, App, HttpServer,middleware};//WEB

//-----------------------------------------------------------------------------------------------------------------------------------------
// 定数関連
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::resorce_module::define;

//-----------------------------------------------------------------------------------------------------------------------------------------
// URIは /json/api/<アクション名>
//-----------------------------------------------------------------------------------------------------------------------------------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    resorce_module::logs::log4rs_init(define::RUST_LOG, define::LOG_FILE);
    HttpServer::new(move || {
        App::new()
            //テスト(削除予定)
            .route("/json/api/req_test/{_user_id}",web::get().to(controller::req_test::execute))
            //session発行|マイグレーション
            .route("/json/api/",web::get().to(controller::index::execute))
            //メンバー登録
            .route("/json/api/member/resist/",web::post().to(controller::membership_resist::execute))
            //メンバーログイン
            .route("/json/api/member/cert/",web::post().to(controller::membership_certification::execute))
            //メンバー登録メールアドレス確認※画面描画
            .route("/cgi-bin/member/confirm/{confirm_uuid}",web::get().to(controller::membership_confirm::execute))
            //ファイルアップロード
            .route("/json/api/upload/",web::post().to(controller::file_upload::execute))
            //メンバーシップ閲覧(形式は拡張子に依存)
            .route("/private/auth/",web::get().to(controller::index::execute))
            //ここから F.A.C.S
            //F.A.C.S ココマデ
            .wrap(middleware::Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" "%{cookie}i""#))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
