use uuid::Uuid;
use colored::*;

// A success println
pub fn success(msg: &str) {
    println!("{} {}", "✔".green(), msg);
}

// A warning println
pub fn warning(msg: &str) {
    println!("{} {}", "⚠".yellow(), msg);
}

// An error println
pub fn error(msg: &str) {
    println!("{} {}", "✖".red(), msg);
}

// Generate a unique UUID for the file name
pub fn generate_uuid() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}