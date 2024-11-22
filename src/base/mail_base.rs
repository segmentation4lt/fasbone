//-----------------------------------------------------------------------------------------------------------------------------------------
// COMMON モジュール(SEG4)
//-----------------------------------------------------------------------------------------------------------------------------------------
use crate::base::seg4_common;

//-----------------------------------------------------------------------------------------------------------------------------------------
// lettreクレート
//-----------------------------------------------------------------------------------------------------------------------------------------
pub use lettre::message::header::ContentType;
pub use lettre::transport::smtp::authentication::Credentials;
pub use lettre::{Message, SmtpTransport, Transport};


//-----------------------------------------------------------------------------------------------------------------------------------------
// 件名をエンコードする
//-----------------------------------------------------------------------------------------------------------------------------------------
//pub fn mime_encode(txt: &str) -> String {
//    format!("=?UTF-8?B?{}?=", base64::encode(txt.as_bytes()))
//}

//-----------------------------------------------------------------------------------------------------------------------------------------
// メイルを送信 ※SMTP AUTH <差出人>,<件名>,<テンプレートファイル名>,<置き換え文言 ※半角;(セミコロン)区切り,0番は宛先>
//-----------------------------------------------------------------------------------------------------------------------------------------
#[allow(dead_code)]
pub fn build_email(mail_from:&str,mail_subject:&str,mail_template:&str,func_args:&str) -> bool  {
    let func_args_array: Vec<&str> = func_args.split(';').collect();
    let mut i :i32 =0;
    let mut messgages = seg4_common::fs::read_to_string(format!("{}/{}/{}",seg4_common::define::PACKAGE_PATH,seg4_common::define::MAIL_TEMPLATE_DIR,mail_template)).expect("FileLoading is Failed.");
    for line_args in &func_args_array {
        if i > 0 {
            messgages = messgages.replace(&format!("### VEC{} ###", i), line_args);
        }
        i+=1;
    }
    let email = Message::builder()
        .from(mail_from.parse().unwrap())
        .to(func_args_array[0].parse().unwrap())
        .subject(mail_subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(messgages))
        .unwrap();
        let creds = Credentials::new(seg4_common::define::SMTP_AUTH_USER.to_owned(), seg4_common::define::SMTP_AUTH_KEY.to_owned());
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    // Send the email
    match mailer.send(&email) {
        Ok(_) => true,
        //_ => false,
        _ => true,
    }
}
