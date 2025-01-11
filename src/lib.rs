use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// [f0, f1] それぞれ &str -> () を受け取って日に応じて実行する
pub fn run(parts: &[fn(&str); 2]) {
    let args: Vec<String> = env::args().collect();
    let exec_file: &str = &args[0];
    let day = &exec_file[exec_file.len() - 2..exec_file.len()];
    let part: usize;
    if let Some(pst) = args.get(1) {
        match &pst[..] {
            "1" | "01" => part = 0,
            "2" | "02" => part = 1,
            _ => part = 0,
        };
    } else {
        part = 0;
    }
    let f = File::open(format!("./input/{day}.txt")).unwrap();
    let mut reader = BufReader::new(f);
    let mut input: String = String::new();
    reader.read_to_string(&mut input).unwrap();
    parts[part](&input);
}
