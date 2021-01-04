mod code_gen;
mod parser;
mod symbol;

use code_gen::Code;
use parser::{Cmd, Parser};
use symbol::SymbolTable;

use std::fs::File;
use std::io::Write;
use std::{env, io::BufWriter};

struct Assembler {
    filename: String,
    parser: Parser,
    code_gen: Code,
    sym_table: SymbolTable,
    out: Vec<String>,
    symbol_adress: usize,
}

impl Assembler {
    pub fn new(filename: String) -> Self {
        let file = filename.clone();
        let mut p = Parser::new(file);
        p.init();
        Assembler {
            filename,
            parser: p,
            code_gen: Code {},
            sym_table: SymbolTable::new(),
            out: vec![],
            symbol_adress: 16,
        }
    }

    fn first_pass(&mut self) {
        while self.parser.has_more_commands() {
            if self.parser.is_label() {
                self.sym_table
                    .add_entry(self.parser.cur_inst().to_string(), self.parser.counter);
                self.parser.lines.remove(self.parser.counter);
            } else {
                self.parser.advance();
            }
        }
        self.parser.counter = 0;
    }

    fn second_pass(&mut self) {
        while self.parser.has_more_commands() {
            let cur_symbol = self.parser.symbol().unwrap_or("None");

            if self.parser.command_type() == Cmd::ACommand
                && cur_symbol.chars().any(|x| !x.is_numeric())
            {
                // @sum
                if !self.sym_table.contains(cur_symbol) {
                    self.sym_table
                        .add_entry(cur_symbol.to_owned(), self.symbol_adress);
                    let f_addr = format!("{:#018b}", self.symbol_adress);
                    let f_out = f_addr.split('b').nth(1).unwrap().to_owned();
                    self.out.push(f_out);
                    self.symbol_adress += 1;
                } else {
                    let addr = self.sym_table.get_address(cur_symbol).unwrap();
                    let f_addr = format!("{:#018b}", addr);
                    let f_out = f_addr.split('b').nth(1).unwrap().to_owned();
                    self.out.push(f_out);
                }
            } else if self.parser.command_type() == Cmd::ACommand
                && cur_symbol.chars().all(|x| x.is_numeric())
            {
                // @100
                let f_sym = format!("{:#018b}", cur_symbol.parse::<usize>().unwrap());
                let f_out = f_sym.split('b').nth(1).unwrap().to_owned();
                self.out.push(f_out);
            } else {
                // C command
                let jump_asm = self.parser.jump();
                let dest_asm = self.parser.dest();
                let comp_asm = self.parser.comp().unwrap();
                let jump_hack = self.code_gen.jump(jump_asm);
                let dest_hack = self.code_gen.dest(dest_asm);
                let comp_hack = self.code_gen.comp(comp_asm);
                let mut output = String::from("111");
                output.push_str(comp_hack);
                output.push_str(dest_hack);
                output.push_str(jump_hack);
                self.out.push(output);
            }

            self.parser.advance();
        }
    }

    fn gen(&mut self) {
        let mut name = self.filename.split('.').next().unwrap().to_owned();
        name.push_str(".hack");
        let out = File::create(name).expect("Problem opening output.file");
        let mut buf = BufWriter::new(&out);
        self.out.iter().for_each(|s| {
            buf.write(format!("{}\n", s).as_bytes())
                .expect("Error writing line in buffer!");
        });
        buf.flush().expect("error in flush");
    }

    fn run(&mut self) {
        self.first_pass();
        self.second_pass();
        self.gen();
    }
}

fn main() {
    if env::args().count() > 1 {
        let filename = env::args().nth(1).unwrap();
        let mut assembler = Assembler::new(filename);
        assembler.run();
    } else {
        println!("Please provide a filepath! -> $ cargo run  [filepath]")
    }
}
