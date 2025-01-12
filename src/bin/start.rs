#![feature(let_chains)]
use std::env;
use std::fs;
use std::process::ExitCode;

static TEMPLATE_FILE: &str = "./src/bin/00.rs";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let given_n: Option<u32> = args.get(1).and_then(|arg| arg.parse().ok());
    if let Some(n) = given_n {
        try_start(n)
    } else {
        eprintln!("Day not given: {given_n:?}");
        ExitCode::FAILURE
    }
}

fn try_start(n: u32) -> ExitCode {
    assert!(fs::exists(TEMPLATE_FILE).unwrap());
    let dest_file = format!("./src/bin/{n:02}.rs");
    if let Ok(b) = fs::exists(&dest_file)
        && !b
    {
        eprintln!("Copying the source to {dest_file}");
        fs::copy(TEMPLATE_FILE, &dest_file).unwrap();
        eprintln!("Enjoy!");
        println!("{dest_file}");
        ExitCode::SUCCESS
    } else {
        eprintln!("Either the destination file exists or not able to check");
        ExitCode::FAILURE
    }
}
