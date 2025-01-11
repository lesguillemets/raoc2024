use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

// [f0, f1] それぞれ &str -> () を受け取って日に応じて実行する
pub fn run_silently(parts: &[fn(&str); 2]) {
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
    let input_filename: String;
    if let Some(true) = args.get(2).map(|a| a == "ex") {
        input_filename = format!("./input/ex_{day}.txt");
    } else {
        input_filename = format!("./input/{day}.txt");
    }
    let f = File::open(input_filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut input: String = String::new();
    reader.read_to_string(&mut input).unwrap();
    parts[part](&input);
}

// デフォルトで時間も表示するようにしましょう
pub fn run(parts: &[fn(&str); 2]) {
    run_and_report_time(parts);
}

pub fn run_and_report_time(parts: &[fn(&str); 2]) {
    let start = Instant::now();
    run_silently(parts);
    println!("Elapsed time: {:?}", start.elapsed());
}
