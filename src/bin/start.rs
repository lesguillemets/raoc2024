#![feature(let_chains)]
use std::env;
use std::fs;

static TEMPLATE_FILE: &str = "./src/bin/00.rs";

fn main() {
    let args: Vec<String> = env::args().collect();
    let given_n: Option<u32> = args.get(1).and_then(|arg| arg.parse().ok());
    if let Some(n) = given_n {
        try_start(n);
    } else {
        println!("Day not given: {given_n:?}")
    }
}

fn try_start(n: u32) -> bool {
    assert!(fs::exists(TEMPLATE_FILE).unwrap());
    let dest_file = format!("./src/bin/{n:02}.rs");
    if let Ok(b) = fs::exists(&dest_file)
        && !b
    {
        println!("Copying the source to {dest_file}");
        fs::copy(TEMPLATE_FILE, dest_file).unwrap();
        println!("Enjoy!");
        true
    } else {
        println!("Either the destination file exists or not able to check");
        false
    }
}
