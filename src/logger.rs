use chrono::Utc;

pub struct Logger;

impl Logger {
    pub fn info(message: &str) {
        println!("[INFO] {} - {}", Utc::now().to_rfc3339(), message);
    }

    pub fn success(message: &str) {
        println!("[SUCCESS] {} - {}", Utc::now().to_rfc3339(), message);
    }

    pub fn error(message: &str) {
        eprintln!("[ERROR] {} - {}", Utc::now().to_rfc3339(), message);
    }

    #[allow(dead_code)]
    pub fn warn(message: &str) {
        eprintln!("[WARN] {} - {}", Utc::now().to_rfc3339(), message);
    }
}
