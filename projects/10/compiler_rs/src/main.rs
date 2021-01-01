mod lexer;
mod repl;
mod token;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

fn main() {
    //repl::start().expect("Error in start");
    //let input = String::from("let x=5+yy; let city=\"Paris\";");
    //let input = String::from("let /**x=5+yy; let*/ city=\"Paris\";");
    //let input = String::from("File name projects/1/ArrayTest/Main.jack\n");

    //println!("{}", input);
    //let mut l = lexer::Lexer::new(input);
    //l.tokenize();
    file_io(String::from("Main.jack"));
}

fn file_io(input_path: String) -> std::io::Result<()> {
    let out_path = input_path.split(".").next().unwrap().to_string() + ".xml";
    let in_f = File::open(&input_path).expect("Couldn't open file!");
    let out_f = File::create(&out_path).expect("Couldn't create file!");
    let mut reader = BufReader::new(in_f);
    let mut writer = BufWriter::new(out_f);
    let mut line = String::new();
    let mut num_b: usize;
    let mut token: token::Token;
    let mut tkn_xml: String;
    num_b = reader.read_line(&mut line).expect("Error reading line!");
    loop {
        if num_b == 0 {
            return Ok(());
        }
        let mut lex = lexer::Lexer::new(line.clone());
        line.clear(); // clear input buffer
        token = lex.next_token().unwrap();
        tkn_xml = lexer::Lexer::token_to_xml(&token);
        writer.write(&tkn_xml.as_bytes());
        writer.flush();
        while token.Type != token::EOF {
            println!("{:?}", token);
            token = lex.next_token().unwrap();
            tkn_xml = lexer::Lexer::token_to_xml(&token);
            writer.write(&tkn_xml.as_bytes());
            writer.flush();
        }
        num_b = reader.read_line(&mut line).expect("Error reading line!");
    }
}
