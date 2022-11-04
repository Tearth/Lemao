use super::time::DateTime;

pub fn debug(message: &str) {
    #[cfg(debug_assertions)]
    log_internal("DEBUG", message);
}

pub fn error(message: &str) {
    log_internal("ERROR", message);
}

pub fn log_internal(level: &str, message: &str) {
    let now = DateTime::now();
    let date_formatted = format!("{}-{:0>2}-{:0>2}", now.year, now.month, now.day);
    let time_formatted = format!("{:0>2}:{:0>2}:{:0>2}:{:0>3}", now.hour, now.minute, now.second, now.millisecond);
    println!("[{} {}] [{}] {}", date_formatted, time_formatted, level, message);
}
