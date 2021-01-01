#[allow(dead_code)]
mod lexer;
#[allow(dead_code)]
mod repl;
#[allow(dead_code)]
mod token;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

use token::Token;

fn main() {
    file_io(String::from("tokenizer_tests/Array.jack")).unwrap();
    file_io(String::from("tokenizer_tests/Square.jack")).unwrap();
    file_io(String::from("tokenizer_tests/SquareGame.jack")).unwrap();
}

fn file_io(input_path: String) -> std::io::Result<()> {
    let out_path = input_path.split(".").next().unwrap().to_string() + ".xml";
    let in_f = File::open(&input_path).expect("Couldn't open file!");
    let out_f = File::create(&out_path).expect("Couldn't create file!");
    let mut reader = BufReader::new(in_f);
    let mut writer = BufWriter::new(out_f);

    let mut line = String::new();
    let mut token: token::Token = Token {
        Type: String::from("("),
        Literal: String::from("("),
    };
    let mut tkn_xml: String;
    writer
        .write("<tokens>\n".as_bytes())
        .expect("Couldn't write <tokens>!");
    let mut num_b = reader.read_line(&mut line).expect("Error reading line!");
    let mut lex = lexer::Lexer::new(line.clone());
    line.clear(); // clear input buffer
    if let Some(t) = lex.next_token() {
        token = t;
        tkn_xml = lexer::Lexer::token_to_xml(&token);
        writer
            .write(&tkn_xml.as_bytes())
            .expect("Couldn't write token!");
        //writer.flush()?;
    }
    loop {
        if num_b == 0 {
            writer.flush()?;
            writer
                .write("</tokens>\n".as_bytes())
                .expect("Couldn't write </tokens>!");
            return Ok(());
        }
        while let Some(t) = lex.next_token() {
            //println!("t = {:?}", &t);
            token = t;
            tkn_xml = lexer::Lexer::token_to_xml(&token);
            writer
                .write(&tkn_xml.as_bytes())
                .expect("Couldn't write token!");
            //writer.flush()?;
        }
        num_b = reader.read_line(&mut line).expect("Error reading line!");
        lex = lexer::Lexer::new(line.clone());
        line.clear();
    }
}
