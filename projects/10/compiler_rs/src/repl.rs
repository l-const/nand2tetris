use crate::token;
use std::io::{self, Write};

use crate::lexer::Lexer;

pub(crate) fn start() -> io::Result<()> {
    const PROMPT: &'static str = ">> ";
    let mut buffer_in = String::new();
    let buffer_out = String::from(PROMPT);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    println!("Hello!This is the Jack programming language!");
    println!("Feel free to type in commands");
    loop {
        stdout
            .write(&buffer_out.as_bytes())
            .expect("Could't write line");
        stdout.flush()?;
        stdin.read_line(&mut buffer_in).expect("Could't read line");
        // logic

        let mut lex = Lexer::new(buffer_in.clone());
        let mut token = lex.next_token();
        while token.Type != token::EOF {
            println!("{:?}", token);
            token = lex.next_token();
        }
        // here
        stdout
            .write(&buffer_in.as_bytes())
            .expect("Could't write line");
        buffer_in.clear();
        stdout.flush()?;
    }
}
