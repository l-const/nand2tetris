mod lexer;
mod repl;
mod token;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    //repl::start().expect("Error in start");
    let input = String::from("let x=5+yy; let city=\"Paris\";");
    //let input = String::from("let /**x=5+yy; let*/ city=\"Paris\";");

    //println!("{}", input);
    let mut l = lexer::Lexer::new(input);
    l.tokenize();
}

fn file_io(input_path: String) -> std::io::Result<()> {
    let out_path = input_path.split(".").next().unwrap().to_string() + ".xml";
    let in_f = File::open(&input_path).expect("Couldn't open file!");
    let out_f = File::create(&out_path).expect("Could't create file!");
    let mut reader = BufReader::new(in_f);
    let mut line = String::new();
    let num_b: usize;
    num_b = reader.read_line(&mut line).expect("Error reading line!");
    loop {
        if num_b == 0 {
            return Ok(());
        }
        let mut lex = lexer::Lexer::new(line.clone());
        line.clear(); // clear input buffer
        let mut token = lex.next_token();
        while token.Type != token::EOF {
            println!("{:?}", token);
            token = lex.next_token();
        }
    }
}
