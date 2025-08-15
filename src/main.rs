use minigrep::*;
use std::io::Write;
use std::process::exit;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1)
    });

    if let Err(e) = run(&file_path) {
        eprintln!("Error: {}", e);
        exit(1)
    }

    print!("按回车键退出...");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
