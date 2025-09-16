use std::{
    fs::File,
    io::{self, BufRead},
};

use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    file: String,
}

fn main() {
    let cli = Cli::parse();
    let file = File::open(cli.file).unwrap();
    let results = find_in_file(cli.pattern, file);
    show_results(results);
}

fn find_in_file(pattern: String, file: File) -> Vec<String> {
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| match line {
            Ok(l) => {
                if l.contains(&pattern) {
                    Some(l)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}

fn show_results(results: Vec<String>) {
    results.iter().for_each(|x| println!("{}", x));
}
