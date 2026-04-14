// Cargo Features: Implement conditional compilation with features

fn main() {
    println!("Code App Running!!");

    #[cfg(feature = "logging")]
    log_message("App Started!");

    #[cfg(feature = "debug_mode")]
    println!("Debug mode ON!!");
}

#[cfg(feature = "logging")]
fn log_message(msg: &str) {
    println!("[LOG]: {}", msg);
}
