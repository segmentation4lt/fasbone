pub mod index; // GET専用。uuidとクッキーを新規取得
pub mod req_test; // URIのパラメータ化のテスト
pub mod membership_resist; //新規会員登録（FW標準）
pub mod membership_certification; //ログイン認証（FW標準）
pub mod membership_confirm; //新規会員登録（FW標準） 最終確認
pub mod file_upload; //ファイルアップロード
pub mod membership_webview; //会員のみ閲覧可能リクエスト(レスポンスは対象ファイル拡張子)

