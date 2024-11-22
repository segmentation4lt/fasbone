//-----------------------------------------------------------------------------------------------------------------------------------------
// 汎用
//-----------------------------------------------------------------------------------------------------------------------------------------
pub use log::info;
pub use serde::{Deserialize ,Serialize};
pub use std::collections::HashMap;

//-----------------------------------------------------------------------------------------------------------------------------------------
// 環境別定数関連
//-----------------------------------------------------------------------------------------------------------------------------------------
pub use crate::resorce_module::define;

//-----------------------------------------------------------------------------------------------------------------------------------------
// 日付関連
//-----------------------------------------------------------------------------------------------------------------------------------------
extern crate chrono;
pub use chrono::prelude::*;

//-----------------------------------------------------------------------------------------------------------------------------------------
// UUID関連
//-----------------------------------------------------------------------------------------------------------------------------------------
extern crate uuid;
pub use uuid::Uuid;

//-----------------------------------------------------------------------------------------------------------------------------------------
// 暗号化関連
//-----------------------------------------------------------------------------------------------------------------------------------------
extern crate block_modes;
extern crate pwhash;
pub use aes::Aes256;
pub use block_modes::{BlockMode, Cbc};
pub use block_modes::block_padding::Pkcs7;
pub use rand::seq::SliceRandom;
pub type AesCbc = Cbc<Aes256, Pkcs7>;
use pwhash::bcrypt;

//-----------------------------------------------------------------------------------------------------------------------------------------
// 画面出力用
//-----------------------------------------------------------------------------------------------------------------------------------------
pub use std::fs;

//-----------------------------------------------------------------------------------------------------------------------------------------
// 暗号化ベース
//-----------------------------------------------------------------------------------------------------------------------------------------
pub const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789*+][-/";

//-----------------------------------------------------------------------------------------------------------------------------------------
// JSONエラーリザルト
//-----------------------------------------------------------------------------------------------------------------------------------------
// 1.HTTP HEADER  
pub const HTTP_CONTENT_TYPE: &str = "application/json; charset=UTF-8";
pub const HTTP_CONTENT_TYPE_HTML: &str = "text/html; charset=UTF-8";
pub const HTTP_CACHE_CONTROL: &str = "no-cache,no-store";
// 2.result json output
pub const VALIDATION_ALLOK: &str = "{\"result\":\"200 VALIDATION ALLOK\"}";
pub const SAME_REQEST: &str = "{\"result\":\"200 SAME REQEST\"}";
pub const PARAM_ERROR: &str = "{\"result\":\"400 PARAM ERROR\"}";
pub const NO_LOGIN_ERROR: &str = "{\"result\":\"401 NO LOGIN ERROR\"}";
pub const NOTFOUND_ERROR: &str = "{\"result\":\"404 File Not Found\"}";
pub const FORBIDDEN_ERROR: &str = "{\"result\":\"403 FORBIDDEN ERROR\"}";
pub const FOTAL_ERROR: &str = "{\"result\":\"500 FOTAL_ERROR\"}";

//-----------------------------------------------------------------------------------------------------------------------------------------
// ローカル定数
//-----------------------------------------------------------------------------------------------------------------------------------------
const CONTENT_TYPES_ARY: [&str; 102] = ["text/html","text/html","text/html","text/html","text/css","text/xml","image/gif","image/jpeg","image/jpeg","application/x-javascript","application/x-javascript","application/atom+xml","application/rss+xml","application/font-woff2","application/x-font-ttf","application/x-font-ttf","text/mathml","text/plain","text/vnd.sun.j2me.app-descriptor","text/vnd.wap.wml","text/x-component","image/png","image/tiff","image/tiff","image/vnd.wap.wbmp","image/x-icon","image/x-jng","image/x-ms-bmp","image/svg+xml","image/svg+xml","image/webp","application/java-archive","application/java-archive","application/java-archive","application/mac-binhex40","application/msword","application/pdf","application/postscript","application/postscript","application/postscript","application/rtf","application/vnd.ms-excel","application/vnd.ms-powerpoint","application/vnd.wap.wmlc","application/vnd.google-earth.kml+xml","application/vnd.google-earth.kmz","application/x-7z-compressed","application/x-cocoa","application/x-java-archive-diff","application/x-java-jnlp-file","application/x-makeself","application/x-perl","application/x-perl","application/x-pilot","application/x-pilot","application/x-rar-compressed","application/x-redhat-package-manager","application/x-sea","application/x-shockwave-flash","application/x-stuffit","application/x-tcl","application/x-tcl","application/x-x509-ca-cert","application/x-x509-ca-cert","application/x-x509-ca-cert","application/x-xpinstall","application/xhtml+xml","application/zip","application/octet-stream","application/octet-stream","application/octet-stream","application/octet-stream","application/octet-stream","application/octet-stream","application/octet-stream","application/octet-stream","application/octet-stream","application/octet-stream","application/octet-stream","application/wasm","audio/midi","audio/midi","audio/midi","audio/mpeg","audio/ogg","audio/x-m4a","audio/x-realaudio","video/3gpp","video/3gpp","video/mp4","video/mpeg","video/mpeg","video/quicktime","video/webm","video/x-flv","video/x-m4v","video/x-mng","video/x-ms-asf","video/x-ms-asf","video/x-ms-wmv","video/x-msvideo","application/json"];
const EXTENS_ARY: [&str; 102] = ["html","htm","shtm","jhtml","css","xml","gif","jpeg","jpg","js","jnt","atom","rss","woff2","ttc","ttf","mml","txt","jad","wml","htc","png","tiff","tif","wbmp","ico","jng","bmp","svgz","svg","webp","jar","war","ear","hqx","doc","pdf","eps","ps","ai","rtf","xls","ppt","wmlc","kml","kmz","7z","cco","jardiff","jnlp","run","pl","pm","prc","pdb","rar","rpm","sea","swf","sit","tk","tcl","der","pem","crt","xpi","xhtml","zip","bin","exe","dll","deb","dmg","eot","img","iso","msi","msp","msm","wasm","mid","midi","kar","mp3","ogg","m4a","ra","3gpp","3gp","mp4","mpg","mpeg","mov","webm","flv","m4v","mng","asf","asx","wmv","avi","json"];

//-----------------------------------------------------------------------------------------------------------------------------------------
// * クレート内関数:gen_ascii_chars
// * IV用にランダム文字列生成
//-----------------------------------------------------------------------------------------------------------------------------------------
pub fn gen_ascii_chars(size: usize) -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        BASE_STR.as_bytes()
            .choose_multiple(&mut rng, size)
            .cloned()
            .collect()
    ).expect("failed of gen_ascii_chars.")
}

//-----------------------------------------------------------------------------------------------------------------------------------------
// * クレート内関数:encrypt
// * 暗号化を実施
//-----------------------------------------------------------------------------------------------------------------------------------------
pub fn encrypt(data: &str) -> String {
    let iv_str = gen_ascii_chars(16);
    let iv = iv_str.as_bytes();
    let cipher = AesCbc::new_from_slices(define::ENCRYPT_KEY.as_bytes(), iv).expect("failed for new_from_slices(encrypt error).");
    let ciphertext = cipher.encrypt_vec(data.as_bytes());
    let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv);
    buffer.write_bytes(&ciphertext);
    base64::encode(buffer.to_bytes())
}

//-----------------------------------------------------------------------------------------------------------------------------------------
// * クレート内関数:decrypt
// * 復号化を実施
//-----------------------------------------------------------------------------------------------------------------------------------------
pub fn decrypt(data: &str) -> String {
    let bytes = base64::decode(data).expect("failed for base64::decode(decrypt error)");
    let cipher = AesCbc::new_from_slices(define::ENCRYPT_KEY.as_bytes(), &bytes[0..16]).expect("failed for new_from_slices(decrypt error).");
    String::from_utf8(cipher.decrypt_vec(&bytes[16..]).expect("failed for from_utf8(decrypt error)")).expect("failed of from_utf8 outside.")
}

//-----------------------------------------------------------------------------------------------------------------------------------------
// * クレート内関数:hashout
// * ハッシュを実施
//-----------------------------------------------------------------------------------------------------------------------------------------
pub fn hashout(str_args: &str) -> String {
    bcrypt::hash(str_args).expect("failed of hash.")
}

//-----------------------------------------------------------------------------------------------------------------------------------------
// * クレート内関数:hashverify
// * ハッシュを比較
//-----------------------------------------------------------------------------------------------------------------------------------------
pub fn hashverify(str_args: &str,hash_args: &str) -> bool {
    bcrypt::verify(str_args, hash_args)
}

//-----------------------------------------------------------------------------------------------------------------------------------------
// * クレート内関数:
// * content_typeを元にファイル拡張子を返却
//-----------------------------------------------------------------------------------------------------------------------------------------
pub fn contenttype_to_extnsis(content_type_value: &str) -> String {
    let mut extens_count =0;
    let mut save_file_extension = String::from("");
    for line_args in &CONTENT_TYPES_ARY {
        if content_type_value.contains(line_args) == true {
            save_file_extension = EXTENS_ARY[extens_count].to_string();
            break;
        }
        extens_count +=1;//インクリメント
    }
    save_file_extension
}

//-----------------------------------------------------------------------------------------------------------------------------------------
// * クレート内関数:
// * ファイル拡張子を元にcontent_typeを返却
//-----------------------------------------------------------------------------------------------------------------------------------------
pub fn extnsis_to_contenttype(extnsis_value: &str) -> String {
    let mut contenttype_count =0;
    let mut content_type = String::from("");
    for line_args in &EXTENS_ARY {
        if extnsis_value.contains(line_args) == true {
            content_type = CONTENT_TYPES_ARY[contenttype_count].to_string();
            break;
        }
        contenttype_count +=1;//インクリメント
    }
    content_type
}

//-----------------------------------------------------------------------------------------------------------------------------------------
// * クレート内関数:
// * テンプレートファイルの変換 <テンプレートファイル名>,<置き換え文言 ※半角;(セミコロン)区切り>
//-----------------------------------------------------------------------------------------------------------------------------------------
#[allow(dead_code)]
pub fn for_template_outtext(html_template:&str,content_args:&str) -> String {
    let content_args_array: Vec<&str> = content_args.split(';').collect();
    let mut i :i32 =0;
    let mut messgages = fs::read_to_string(format!("{}/{}/{}",define::PACKAGE_PATH,define::CGI_TEMPLATE_DIR,html_template)).expect("FileLoading is Failed.");
    for line_args in &content_args_array {
        if i > 0 {
            messgages = messgages.replace(&format!("### VEC{} ###", i), line_args);
        }
        i+=1;
    }
    messgages
}

#[cfg(test)]
mod tests {
    use crate::base::seg4_common;
    #[test]
    fn test_encrypt_decrypt() {
        assert!(seg4_common::encrypt("00000000-0000-0000-0000-000000000000").chars().count() > 0);
        assert!(seg4_common::decrypt("WXNlUC8qYXJ1dlZCaFhGXZiwYORj5irAXw48+ams8ZYlzjZ5YkR452ysJjSSGZLXb++ou8pC4hsyi5LnS8Ra3A==").chars().count() > 0);
    }
    #[test]
    fn test_hashout_hashverify() {
        assert!(seg4_common::hashout("abcdefghijk").chars().count() > 0);
        assert!(seg4_common::hashverify("2XTFyYEjwCcr","$2b$10$CE017CK0psCSUCKs/Xr2EufqFGmKDDQmFVFT/xHCToU.L4IVGz40O"));
    }   
    #[test]
    fn test_contenttype_to_extnsis() {
        assert!(seg4_common::contenttype_to_extnsis("image/png").chars().count() > 0);
    }
    #[test]  
    fn test_extnsis_to_contenttype() {
        assert!(seg4_common::extnsis_to_contenttype("png").chars().count() > 0);
    }  
    #[test]  
    fn test_for_template_outtext() {
        assert!(seg4_common::for_template_outtext("dummy.txt","aaaa;bbbb").chars().count() > 0);
    }  
}


