use super::env;

pub fn setup() {
    let log_mode = env::val::<String>("APP.LOG_MODE").unwrap_or("log".to_string());
    if log_mode == "log" {
        tracing_subscriber::fmt()
            .with_line_number(true)
            .with_level(true)
            .with_target(true)
            .with_file(false)
            .with_thread_names(true)
            .without_time()
            .with_ansi(true)
            .init();
    } else {
        let path = env::str("LOG_PATH").unwrap_or("".to_string());
        let file_appender = tracing_appender::rolling::daily(path, "");
        // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        tracing_subscriber::fmt()
            .with_line_number(true)
            .with_level(true)
            .with_target(true)
            .with_file(false)
            .with_thread_names(true)
            .with_ansi(false)
            .with_writer(file_appender)
            .init();
    }
}

