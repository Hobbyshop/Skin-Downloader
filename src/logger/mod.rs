use colorful::{Colorful, Color};

const NAME: &str = "SkinDownloader";

pub fn log(msg: &str) {
    let text = format!("[{} - INFO] {}", NAME, msg);
    println!("{}", text.color(Color::LightBlue))
}

pub fn warn(msg: &str) {
    let text = format!("[{} - WARNING] {}", NAME, msg);
    println!("{}", text.color(Color::Yellow))
}

pub fn error(msg: &str) {
    let text = format!("[{} - ERROR] {}", NAME, msg);
    println!("{}", text.color(Color::Red))
}
