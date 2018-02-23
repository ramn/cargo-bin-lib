use std::process::{Command, exit};

fn main() {
    let mut args = vec!["new".into(), "--bin".into()];
    args.extend(std::env::args().skip(2));
    match Command::new("cargo").args(&args).status() {
        Ok(status) => exit(status.code().unwrap_or(1)),
        Err(err) => {
            eprintln!("cargo new --bin failed: {}", err);
            exit(1);
        }
    }
}
