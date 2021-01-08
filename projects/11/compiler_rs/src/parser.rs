use crate::lexer::Lexer;
use crate::symbol::{self, IdKind, SymbolTable};
use crate::token::{self, Token, TokenKind};
use crate::vm_writer::{self, Command, Segment, VmWriter};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

pub(crate) struct Parser {
    reader: BufReader<File>,
    writer: BufWriter<File>,
    lex: Lexer,
    cur_line: String,
    cur_token: Token,
    peek_token: Token,
    class_name: String,
    vm_writer: VmWriter,
    s_table: SymbolTable,
}

impl Parser {
    pub(crate) fn new(file_path: String) -> Self {
        let file_path = &file_path;
        let out_path = file_path.split("/").last().unwrap().to_string() + ".xml";
        let in_f = File::open(&file_path).expect("Couldn't open file!");
        let out_f = File::create(&out_path).expect("Couldn't create file!");
        let reader = BufReader::new(in_f);
        let writer = BufWriter::new(out_f);
        let lex = Lexer::new(String::from(""));
        let s_table = SymbolTable::new();
        let vm_writer = VmWriter::new(&file_path);
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
            class_name: String::from(""),
            s_table,
            vm_writer,
        };
        p.init();
        p
    }

    pub(crate) fn init(&mut self) {
        self.read_new_line();
        //self.next_token();
    }

    fn compile_class(&mut self) {
        self.write("<class>\n"); // <class>
                                 // keyword class
        self.require(token::CLASS);
        // identifier className
        self.class_name = self.peek_token.Literal.clone();
        self.require(token::IDENT);
        // symbol '{'
        self.require(token::LBRACE);
        while self.peek_token_is(token::FIELD) || self.peek_token_is(token::STATIC) {
            self.compile_class_vardec();
        }
        while self.peek_token_is(token::CONSTRUCTOR)
            || self.peek_token_is(token::FUNCTION)
            || self.peek_token_is(token::METHOD)
        {
            self.compile_subroutine();
        }
        // symbol '}'
        self.require(token::RBRACE);
        self.write("</class>\n");
    }

    fn compile_class_vardec(&mut self) {
        //compiles a static declaration or a field declaration.
        // <classVarDec>
        self.write("<classVarDec>\n");
        //keyword (static| field)
        let kind: symbol::IdKind;
        if self.peek_token.Type == token::STATIC {
            kind = IdKind::STATIC;
        } else {
            kind = IdKind::FIELD;
        }
        self.terminal();
        //keyword type or identifier className
        //self.require(token::IDENT);
        let type_k = &self.peek_token.Literal;
        self.terminal();
        let mut name_k = &self.peek_token.Literal;
        self.terminal();
        // call self.s_table.define(name_k, type_k, kind)
        while self.peek_token_is(token::COMMA) {
            self.require(token::COMMA);
            name_k = &self.peek_token.Literal;
            // call self.s_table.define(name_k, type_k, kind) every time
            self.require(token::IDENT);
        }
        self.require(token::SEMICOLON);
        // </classVarDec>
        self.write("</classVarDec>\n");
    }

    fn compile_subroutine(&mut self) {
        // compiles a complete method, function,
        // or constructor.
        //<subroutineDec>
        self.write("<subroutineDec>\n");
        self.s_table.start_subroutine();
        //keyword (constructor| function| method)
        if self.peek_token.Type == token::METHOD {
            self.s_table.define("this", &self.class_name, IdKind::ARG);
        }
        self.terminal();
        // keyword or identifier (void| type)
        self.terminal();
        // identifier subroutinename
        self.require(token::IDENT);
        // LPAREN
        self.require(token::LPAREN);
        self.compile_parameter_list();

        // RPAREN
        self.require(token::RPAREN);
        self.write("<subroutineBody>\n");
        // LBRACE
        self.require(token::LBRACE);

        while self.peek_token_is(token::VAR) {
            self.compile_vardec();
        }

        self.compile_statements();

        // RBRACE
        self.require(token::RBRACE);
        self.write("</subroutineBody>\n");
        self.write("</subroutineDec>\n");
        //</subroutineDec>
    }
    fn compile_parameter_list(&mut self) {
        // compiles a (possibly empty) parameter
        //list, not including the enclosing “()”.
        self.write("<parameterList>\n");
        self.compile_parameter();
        while self.peek_token_is(token::COMMA) {
            self.terminal(); // comma
            self.compile_parameter();
        }
        self.write("</parameterList>\n");
    }

    fn compile_parameter(&mut self) {
        if self.is_type() {
            self.terminal();
            let type_k = &self.cur_token.Literal; // Type -> Point
            let name_k = &self.peek_token.Literal; //ident -> other
            self.s_table.define(name_k, type_k, IdKind::ARG);
            self.require(token::IDENT);
        }
    }

    fn compile_vardec(&mut self) {
        self.write("<varDec>\n");
        self.require(token::VAR);
        // var int dx, dy, dz;
        let type_k = self.peek_token.Literal.clone();
        while !self.cur_token_is(token::SEMICOLON) {
            self.terminal();
            if self.cur_token.Type != token::COMMA {
                let name_k = &self.cur_token.Literal; //ident -> other
                self.s_table.define(name_k, &type_k, IdKind::LOCAL);
            }
        }
        self.is(token::SEMICOLON);
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
            } else if self.peek_token_is(token::RETURN) {
                self.compile_return();
            } else {
                panic!("Error in compile statements!{:?}", self.cur_token.Type);
            }
        }
        self.write("</statements>\n");
    }

    //fn is_statement

    fn compile_do(&mut self) {
        // do' subroutineCall ';'
        self.write("<doStatement>\n");
        self.require(token::DO);
        self.require(token::IDENT);
        self.compile_subroutine_call();
        self.require(token::SEMICOLON);
        self.write("</doStatement>\n");
    }

    fn compile_let(&mut self) {
        // 'let' varName ('[' expression ']')? '=' expression ';'
        self.write("<letStatement>\n");
        self.require(token::LET); //let keyword'
        self.require(token::IDENT); //varName identifier'
        if self.peek_token_is(token::LBRACKET) {
            // lbracket
            self.require(token::LBRACKET);
            self.compile_expression();
            // rbracket
            self.require(token::RBRACKET);
        }
        self.require(token::EQ);
        self.compile_expression();
        self.require(token::SEMICOLON);
        self.write("</letStatement>\n");
    }

    fn compile_while(&mut self) {
        //compiles a sequence of statements, not
        // including the enclosing “{}”.
        self.write("<whileStatement>\n");
        self.require(token::WHILE); //while keyword'
        self.cond_expression();
        self.write("</whileStatement>\n");
    }

    fn compile_return(&mut self) {
        self.write("<returnStatement>\n");
        self.require(token::RETURN); //return keyword'
        if !self.peek_token_is(token::SEMICOLON) {
            self.compile_expression();
        }
        self.require(token::SEMICOLON); //-> semicolon
        self.write("</returnStatement>\n");
    }

    fn compile_if(&mut self) {
        // compiles an if statement, possibly
        //with a trailing else clause.
        self.write("<ifStatement>\n");
        self.require(token::IF); //if  keyword'
        self.cond_expression();
        if self.peek_token_is(token::ELSE) {
            self.require(token::ELSE); // else keyword
            self.require(token::LBRACE); // '{' symbol
            self.compile_statements();
            self.require(token::RBRACE); // '}' symbol
        }
        self.write("</ifStatement>\n");
    }

    fn cond_expression(&mut self) {
        self.require(token::LPAREN); // '(' symbol
        self.compile_expression();
        self.require(token::RPAREN); // ')' keyword'
        self.require(token::LBRACE); // '{' symbol
        self.compile_statements();
        self.require(token::RBRACE); // '}' symbol
    }

    fn compile_expression(&mut self) {
        // expression: term (op term)*
        if !self.is_terminal() {
            return;
        }
        self.write("<expression>\n");
        self.write("<term>\n");
        self.compile_term();
        while self.is_op() {
            self.terminal(); // symbol operator
            self.write("<term>\n");
            self.compile_term();
            self.write("</term>\n");
        }
        self.write("</term>\n");
        self.write("</expression>\n");
    }

    fn compile_term(&mut self) {
        if self.peek_token_is(token::INT_CONST)
            || self.peek_token_is(token::STRING_CONST)
            || self.is_keyword_const()
        {
            self.terminal();
        } else if self.peek_token_is(token::LPAREN) {
            self.terminal();
            self.compile_expression();
            self.require(token::RPAREN);
        } else if self.is_unary_op() {
            self.terminal();
            self.compile_term();
        } else if self.is_ident() {
            self.terminal();
            if self.peek_token_is(token::LBRACKET) {
                self.require(token::LBRACKET);
                self.compile_expression();
                self.require(token::RBRACKET);
            } else if self.peek_token_is(token::LPAREN) || self.peek_token_is(token::DOT) {
                self.compile_subroutine_call();
            } else {
                println!("shit!!\t: {:?}", self.peek_token.Type);
            }
        } else {
            println!("shit2\t : {:?}", self.peek_token.Type);
        }
    }

    fn compile_subroutine_call(&mut self) {
        if self.peek_token_is(token::DOT) {
            self.require(token::DOT);
            self.require(token::IDENT);
        }
        self.require(token::LPAREN);
        self.compile_expression_list();
        self.require(token::RPAREN);
    }

    fn terminal(&mut self) {
        self.next_token();
        match self.cur_token.token_type() {
            TokenKind::Keyword(s) => {
                let s = format!("<keyword> {} </keyword>\n", &s);
                self.write(&s);
            }
            TokenKind::Symbol(s) => {
                let s = match s.as_str() {
                    "<" => "&lt;",
                    ">" => "&gt;",
                    "&" => "&amp;",
                    "\"" => "&quot;",
                    s => s,
                };
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
    }

    fn compile_expression_list(&mut self) {
        // compiles a (possibly empty) comma-
        // separated list of expressions.
        self.write("<expressionList>\n");
        self.compile_expression();
        while self.peek_token_is(token::COMMA) {
            self.terminal();
            self.compile_expression();
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

    fn require(&mut self, typ: &str) {
        if self.peek_token_is(typ) {
            self.terminal();
        } else {
            panic!(
                "Require error!Expected:\t{:?} , Real:\t{:?}",
                typ, self.peek_token.Type
            );
        }
    }

    fn is_op(&mut self) -> bool {
        self.peek_token_is(token::PLUS)
            || self.peek_token_is(token::MINUS)
            || self.peek_token_is(token::ASTERISK)
            || self.peek_token_is(token::SLASH)
            || self.peek_token_is(token::AND)
            || self.peek_token_is(token::OR)
            || self.peek_token_is(token::GT)
            || self.peek_token_is(token::LT)
            || self.peek_token_is(token::EQ)
    }

    fn is_unary_op(&mut self) -> bool {
        self.peek_token_is(token::NOT) || self.peek_token_is(token::MINUS)
    }

    fn is_terminal(&mut self) -> bool {
        self.peek_token_is(token::INT_CONST)
            || self.peek_token_is(token::STRING_CONST)
            || self.is_unary_op()
            || self.is_ident()
            || self.peek_token_is(token::LPAREN)
            || self.is_keyword_const()
    }

    fn is_ident(&mut self) -> bool {
        self.peek_token_is(token::IDENT)
    }

    fn is_keyword_const(&mut self) -> bool {
        self.peek_token_is(token::THIS)
            || self.peek_token_is(token::TRUE)
            || self.peek_token_is(token::FALSE)
            || self.peek_token_is(token::NULL)
    }

    fn is_type(&mut self) -> bool {
        self.peek_token_is(token::INT_K)
            || self.peek_token_is(token::CHAR_K)
            || self.peek_token_is(token::IDENT)
            || self.peek_token_is(token::BOOLEAN_K)
    }

    fn is(&self, typ: &str) {
        if !self.cur_token_is(typ) {
            panic!(
                "Is error!Expected:\t{:?} , Real:\t{:?}",
                typ, self.cur_token.Type
            );
        }
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

#[test]
fn parser_test1() {
    let mut pars = Parser::new("MainArray.jack".to_string());
    //println!("{:?} , {:?}", pars.cur_token, pars.peek_token);
    pars.parse();
}
#[test]
fn parser_test2() {
    let mut pars = Parser::new("SquareGame.jack".to_string());
    //println!("{:?} , {:?}", pars.cur_token, pars.peek_token);
    pars.parse();
}

#[test]
fn parser_test3() {
    let mut pars = Parser::new("MainES.jack".to_string());
    //println!("{:?} , {:?}", pars.cur_token, pars.peek_token);
    pars.parse();
}
