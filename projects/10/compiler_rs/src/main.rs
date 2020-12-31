mod lexer;
mod repl;
mod token;
fn main() {
    println!("Hello!This is the Monkey programming language!");
    println!("Feel free to type in commands");
    //repl::start().expect("Error in start");
    let input = String::from("let x=5+yy; let city=\"Paris\";");
    println!("{}", input);
    let mut l = lexer::Lexer::new(input);
    l.tokenize();
}
