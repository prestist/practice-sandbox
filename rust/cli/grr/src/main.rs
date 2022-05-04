
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Config {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,

}

impl Config {
    fn new(_path: &str,_pattern: &str) -> Config {
        Config {
            path: PathBuf::from(_path),
            pattern: _pattern.to_string(),
        }
    }
}
fn main() {
    //let pattern = std::env::args().nth(1).expect("no pattern given");
    //let path = std::env::args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };
    //Instead of having to map the cli args to the struct, thanks to clap's #[derive(Parser)] we serialize the struct using struct::parse();
    let args = Config::parse();

    let content = std::fs::read_to_string(&args.path).expect("could no open file");

    let match_found = pattern_search(&content,&args.pattern);
    
    if match_found {
        println!("Match found");
    }
}

fn pattern_search(contents: &str, pattern: &str) -> bool {
    let mut list_of_matches = Vec::new();
    let standard_pattern = pattern.to_lowercase();
    let standard_contents = contents.to_lowercase();

    for ln in standard_contents.lines() {
        if ln.contains(&standard_pattern) {
           list_of_matches.push(ln);
        }
    }
    
    for i in &list_of_matches {
        println!("{}", i);
    }
    
    list_of_matches.len() > 0
}

#[test]
fn ensure_standardized_pattern() {
    let pattern = "THIS";
    let file_contents = "This is not all caps!!! ";

    assert!(pattern_search(&file_contents,pattern),"Should have found the match");
}