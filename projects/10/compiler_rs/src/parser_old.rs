use crate::lexer::Lexer;
use crate::token::{self, Token, TokenKind};

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

struct Parser {
    reader: BufReader<File>,
    writer: BufWriter<File>,
    lex: Lexer,
    cur_line: String,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub(crate) fn new(file_path: String) -> Self {
        let out_path = file_path.split(".").next().unwrap().to_string() + ".xml";
        let in_f = File::open(&file_path).expect("Couldn't open file!");
        let out_f = File::create(&out_path).expect("Couldn't create file!");
        let reader = BufReader::new(in_f);
        let writer = BufWriter::new(out_f);
        let lex = Lexer::new(String::from(""));
        let mut p = Parser {
            reader,
            writer,
            lex,
            cur_line: String::from(""),
            cur_token: Token {
                Type: String::from("("),
                Literal: String::from("("),
            },
            peek_token: Token {
                Type: String::from("("),
                Literal: String::from("("),
            },
        };
        p.init();
        p
    }

    pub(crate) fn init(&mut self) {
        self.read_new_line();
        //self.next_token();
    }

    fn compile_class(&mut self) {
        // <class>
        self.write("<class>\n");
        //compiles a complete class.
        if self.peek_token_is(token::CLASS) {
            self.compile_term(); // keyword class
        }
        // identifier className
        if self.peek_token_is(token::IDENT) {
            self.compile_term();
        }
        // symbol '{'
        if self.peek_token_is(token::LBRACE) {
            self.compile_term();
        }

       
        while self.peek_token_is(token::FIELD) || self.peek_token_is(token::STATIC) {
                self.compile_class_vardec();
        }
        
        while self.peek_token_is(token::CONSTRUCTOR) || self.peek_token_is(token::FUNCTION) || self.peek_token_is(token::METHOD) {
                self.compile_subroutine();
                //self.next_token();
        } 
               
        // symbol '}'
        if self.peek_token_is(token::RBRACE) {
            self.compile_term();
        } else {
            panic!("Error in compile class {:?}", self.cur_token.Type);
        
        }
        // </class>
        self.write("</class>\n");
    }

    fn compile_class_vardec(&mut self) {
        //compiles a static declaration or a field declaration.
        // <classVarDec>
        self.write("<classVarDec>\n");
        //keyword (static| field)
        self.compile_term();
        //keyword type or identifier className
        self.compile_term();

        while !self.peek_token_is(token::SEMICOLON) {
            self.compile_term();
        }

        if self.peek_token_is(token::SEMICOLON) {
            self.compile_term();
        }

        // </classVarDec>
        self.write("</classVarDec>\n");
    }

    fn compile_subroutine(&mut self) {
        // compiles a complete method, function,
        // or constructor.
        //<subroutineDec>
        self.write("<subroutineDec>\n");

        //keyword (constructor| function| method)
        self.compile_term();
        // keyword or identifier (void| type)
        self.compile_term();
        // identifier subroutinename
        self.compile_term();
        // LPAREN
        //self.compile_term();
        self.next_token();
        self.writer.write("<symbol> ( </symbol>\n".as_bytes());

        if self.cur_token_is(token::LPAREN) {
            self.compile_parameter_list();
        } else {
            panic!("parameter list {:?}", self.cur_token.Type);
        }

        // RPAREN
        self.compile_term();
        self.write("<subroutineBody>\n");
        // LBRACE
        self.compile_term();

     

        while self.peek_token_is(token::VAR) {
            self.compile_vardec();
        }

        self.compile_statements();

        // RBRACE
        self.compile_term();
        self.write("</subroutineBody>\n");
        self.write("</subroutineDec>\n");
        //</subroutineDec>
    }
    fn compile_parameter_list(&mut self) {
        // compiles a (possibly empty) parameter
        //list, not including the enclosing “()”.
        self.write("<parameterList>\n");

        if !self.cur_token_is(token::LPAREN) {
           
        }

        while !self.peek_token_is(token::RPAREN) {
            self.compile_term();
        }

        self.write("</parameterList>\n");
    }

    fn compile_vardec(&mut self) {
        self.write("<varDec>\n");

        if !self.peek_token_is(token::VAR) {
            panic!("error in vard_dec");
        }

        while !self.cur_token_is(token::SEMICOLON) {
            self.compile_term();
        }
        // SEMICOLON
        self.compile_term();
        self.write("</varDec>\n");
    }

    fn compile_statements(&mut self) {
        self.write("<statements>\n");
        while !self.peek_token_is(token::RBRACE) {
            if self.peek_token_is(token::IF) {
                self.compile_if();
            } else if self.peek_token_is(token::DO) {
                self.compile_do();
            } else if self.peek_token_is(token::WHILE) {
                self.compile_while();
            } else if self.peek_token_is(token::LET) {
                self.compile_let();

            } else if self.peek_token_is(token::RETURN){
                self.compile_return();
            } else {
                panic!("Error in compile statements!{:?}", self.cur_token.Type);
            }
        }

        self.write("</statements>\n");
    }

    fn compile_do(&mut self) {
        self.write("<doStatement>\n");
        // keyword do
        self.compile_term();
        while !self.cur_token_is(token::LPAREN) {
            self.compile_term();
        }
        self.compile_expression_list();
        // RPAREN SYMBOL
        //self.compile_term();
        
        // while !self.peek_token_is(token::SEMICOLON) {
        //     self.compile_term();
        // }

        if self.peek_token_is(token::SEMICOLON) {
            panic!("compile_do {:?}", self.cur_token.Type)
        }
        // SEMICOLON SYMBOL
        self.compile_term();
        self.write("</doStatement>\n");
    }

    fn compile_let(&mut self) {
        // 'let' varName ('[' expression ']')? '=' expression ';'
        self.write("<letStatement>\n");
        self.compile_term(); //let keyword'
        self.compile_term(); //varName identifier'
        if self.peek_token_is(token::LBRACKET) {
            // lparen
            self.compile_term();
            self.compile_expression();
            // rparen
            self.compile_term();
        }
        if self.peek_token_is(token::EQ) {
            self.compile_term(); // -> = symbol equal
        }
        self.compile_expression();
        if self.peek_token_is(token::SEMICOLON) {
            self.compile_term(); //-> semicolon
        }
        self.write("</letStatement>\n");
    }

    fn compile_while(&mut self) {
        //compiles a sequence of statements, not
        // including the enclosing “{}”.
        self.write("<whileStatement>\n");
        self.compile_term(); //while keyword'
        self.compile_term(); // '(' symbol
        self.compile_expression();
        self.compile_term(); // ')' keyword'
        self.compile_term(); // '{' symbol
        self.compile_statements();
        self.compile_term(); // '}' symbol
        self.write("</whileStatement>\n");
    }

    fn compile_return(&mut self) {
        self.write("<returnStatement>\n");
        self.compile_term(); //return keyword'
        if !self.peek_token_is(token::SEMICOLON) {
            self.compile_expression();
        }
        self.compile_term(); //-> semicolon
        self.write("</returnStatement>\n");
    }

    fn compile_if(&mut self) {
        // compiles an if statement, possibly
        //with a trailing else clause.
        self.write("<ifStatement>\n");
        self.compile_term(); //if  keyword'
        self.compile_term(); // '(' symbol
        self.compile_expression();
        self.compile_term(); // ')' keyword'
        self.compile_term(); // '{' symbol
        self.compile_statements();
        self.compile_term(); // '}' symbol
        if self.peek_token_is(token::ELSE) {
            self.compile_term(); // else keyword
            self.compile_term(); // '{' symbol
            self.compile_statements();
            self.compile_term(); // '}' symbol
        }
        self.write("</ifStatement>\n");
    }

    fn compile_expression(&mut self) {
        // expression: term (op term)*
        self.write("<expression>\n");
        self.write("<term>\n");
        self.compile_term();
        while self.cur_token_is(token::PLUS)||
        self.cur_token_is(token::MINUS)||
        self.cur_token_is(token::ASTERISK)||
        self.cur_token_is(token::SLASH)||
        self.cur_token_is(token::AND)||
        self.cur_token_is(token::OR)||
        self.cur_token_is(token::GT)||
        self.cur_token_is(token::LT)||
        self.cur_token_is(token::EQ)
        {
            //self.compile_term(); // symbol operator
            self.write("<term>\n");
            self.compile_term();
            self.write("</term>\n");
        }
        self.write("</term>\n");
        
        self.write("</expression>\n");
    }

    fn compile_term(&mut self) {
        let prev_tok_type = self.cur_token.Type.clone();
        self.next_token();
        
        match self.cur_token.token_type() {
            TokenKind::Keyword(s) => {
                let s = format!("<keyword> {} </keyword>\n", &s);
                self.write(&s);
            }
            TokenKind::Symbol(s) => {
                let s = format!("<symbol> {} </symbol>\n", &s);
                self.write(&s);
            }
            TokenKind::Integer(s) => {
                let s = format!("<integerConstant> {} </integerConstant>\n", &s);
                self.write(&s);
            }
            TokenKind::StringC(s) => {
                let s = format!("<stringConstant> {} </stringConstant>\n", &s);
                self.write(&s);
            }
            TokenKind::Identifier(s) => {
                let s = format!("<identifier> {} </identifier>\n", &s);
                self.write(&s);
            }
        }
        if self.peek_token_is(token::LBRACKET) && self.cur_token_is(token::IDENT) {
            // varName '[' expression ']'
            self.compile_term(); // symbol [
            self.compile_expression(); 
            self.compile_term(); // symbol ]
        } else if self.peek_token_is(token::DOT) && self.cur_token_is(token::IDENT) {
           self.compile_term();  // dot
           self.compile_term(); // subroutiname ident
           self.compile_term(); // lparen
        } else if  self.cur_token_is(token::NOT) || self.cur_token_is(token::MINUS){
            // unaryOp term
            self.compile_term();
        } else if  self.cur_token_is(token::LPAREN) {
            // (' expression ')' || ( expression_list )
            if prev_tok_type != token::IDENT {
                // (' expression ')'
                self.compile_expression();
                self.compile_term();
            } else {
                self.compile_expression_list();
                self.compile_term();
            }
        } else {
            // don't know
        }
    }
   
    fn compile_expression_list(&mut self) {
        // compiles a (possibly empty) comma-
        // separated list of expressions.
        self.write("<expressionList>\n");
        while !self.peek_token_is(token::RPAREN) {
            self.compile_expression();
            if self.peek_token_is(token::COMMA) {
                self.compile_term();
            }
           
        }
        self.write("</expressionList>\n");
    }
    fn next_token(&mut self) {
        if let Some(t) = self.lex.next_token() {
            self.cur_token = self.peek_token.clone();
            self.peek_token = t;
        } else {
            self.read_new_line();
        }
    }

    fn cur_token_is(&self, typ: &str) -> bool {
        self.cur_token.Type == typ
    }

    fn peek_token_is(&self, typ: &str) -> bool {
        self.peek_token.Type == typ
    }

    fn read_new_line(&mut self) {
        let mut line = String::from("");
        let num_bytes_read = self
            .reader
            .read_line(&mut line)
            .expect("Error reading line!");
        if num_bytes_read == 0 {
            self.writer.flush().expect("Couldn't flush");
            return;
        }
        self.lex = Lexer::new(line);
        self.cur_line = self.lex.input.clone();
        println!("{:?}", self.cur_line);
        self.next_token();
    }

    fn write(&mut self, info: &str) {
        self.writer
            .write(info.as_bytes())
            .expect("Error writing buffer!");
    }

    pub(crate) fn parse(&mut self) {
        self.compile_class();
    }
}

//#[test]
// fn parser_test() {
//     let mut pars = Parser::new("Square.jack".to_string());
//     //println!("{:?} , {:?}", pars.cur_token, pars.peek_token);
//     pars.parse();
// }
