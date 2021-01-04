#[allow(dead_code)]
mod lexer;
#[allow(dead_code)]
mod parser;
#[allow(dead_code)]
mod repl;
#[allow(dead_code)]
mod token;

use std::fs;
use std::io;
//use token::Token;

struct Analyzer<'fpath> {
    filepath: &'fpath str,
}

impl<'a> Analyzer<'a> {
    pub fn new(s: &'a String) -> Self {
        Analyzer { filepath: s }
    }
    pub fn run(&self) -> io::Result<()> {
        if fs::metadata(&self.filepath)
            .expect("Couldn't get file meatadata")
            .file_type()
            .is_dir()
        {
            for entry in fs::read_dir(&self.filepath)? {
                let entry = entry?;
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                if path_str.split(".").nth(1).unwrap() == ".jack" {
                    let mut parser = parser::Parser::new(path_str);
                    parser.parse();
                }
            }
        } else {
            let mut parser = parser::Parser::new(self.filepath);
            parser.parse();
        }
        Ok(())
    }
}
fn main() {
    if let Some(fpath) = std::env::args().nth(1) {
        let analyzer = Analyzer::new(&fpath);
        analyzer.run().expect("Error in analyzer!");
    } else {
        println!("Usage: cargo run <path>");
    }
}
