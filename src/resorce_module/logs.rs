use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

/*
 * log4rsの初期化 汎用性を重視して作成する事。
 * 引数:rust_log_input(String型) ⇒RUST環境変数
 *     log_path(String型)      ⇒出力するログのパス
 *
 */
pub fn log4rs_init(rust_log_input: &str, log_path: &str) {
    //引数を反映する。※本番モード対応はdefineで制御
    let level = match rust_log_input {
        "Off" => LevelFilter::Off,
        "Error" => LevelFilter::Error,
        "Warn" => LevelFilter::Warn,
        "Info" => LevelFilter::Info,
        "Debug" => LevelFilter::Debug,
        _ => LevelFilter::Trace
    };
    let file_path = log_path.to_string();

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}]:{m}\n",
        )))
        .build(file_path)
        .expect("Logfile is problem of permission.");

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(level),
        )
        .unwrap();
    std::env::set_var("RUST_LOG", rust_log_input.to_string());
    log4rs::init_config(config).unwrap();
}
