#[allow(dead_code)]
mod lexer;
#[allow(dead_code)]
mod parser;
#[allow(dead_code)]
mod repl;
#[allow(dead_code)]
mod symbol;
#[allow(dead_code)]
mod token;
#[allow(dead_code)]
mod vm_writer;

use std::fs;
use std::io;
use std::thread;

struct Analyzer<'fpath> {
    filepath: &'fpath str,
}

impl<'a> Analyzer<'a> {
    pub fn new(s: &'a String) -> Self {
        Analyzer { filepath: s }
    }
    pub fn run(&self) -> io::Result<()> {
        if fs::metadata(&self.filepath)
            .expect("Couldn't get file metadata")
            .file_type()
            .is_dir()
        {
            let mut handles = vec![];
            for entry in fs::read_dir(&self.filepath)? {
                let entry = entry?;
                let path = entry.path();
                let path_str = path.to_str().unwrap().to_string();
                if path_str.split(".").last().unwrap() == "jack" {
                    handles.push(thread::spawn( move || {
                    let mut parser = parser::Parser::new(path_str);
                    parser.parse();
                    }));
                }
            }
            for handle in handles {
                handle.join().unwrap();
            }
        } else {
            let mut parser = parser::Parser::new(self.filepath.to_string());
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
