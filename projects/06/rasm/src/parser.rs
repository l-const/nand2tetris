use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
pub(crate) enum Cmd {
    ACommand,
    CCommand,
    LCommand,
}

pub(crate) struct Parser {
    pub(crate) counter: usize,
    filename: String,
    pub(crate) lines: Vec<String>,
}

#[allow(dead_code)]
impl Parser {
    pub(crate) fn new(filename: String) -> Self {
        Parser {
            counter: 0,
            filename,
            lines: vec![],
        }
    }

    pub(crate) fn init(&mut self) {
        let f = File::open(self.filename.clone()).expect("error parsing files");
        let buf = BufReader::new(f);
        self.lines = buf
            .lines()
            .map(|l| l.expect("could not parse line"))
            .collect();

        self.clean_lines()
    }

    fn my_filter(s: String) -> String {
        if s.contains("//") {
            let vec: Vec<&str> = s.split("//").collect();
            return vec[0].to_string();
        }
        s
    }

    fn clean_lines(&mut self) {
        // Remove line comments and empty lines
        let _it: Vec<String> = self
            .lines
            .clone()
            .into_iter()
            .filter(|x| !x.starts_with('/') && !x.is_empty())
            .collect();
        // Remove whitespaces and inline comments from lines
        let _it: Vec<String> = _it
            .into_iter()
            .map(|x| Parser::my_filter(x))
            .map(|x| x.replace(" ", ""))
            .collect();
        self.lines = _it;
    }

    pub(crate) fn is_label(&self) -> bool {
        self.lines[self.counter].starts_with('(') && self.lines[self.counter].ends_with(')')
    }

    fn is_a(&self) -> bool {
        self.cur_inst().starts_with('@')
    }

    fn is_c(&self) -> bool {
        !self.is_a() || self.is_label()
    }

    pub(crate) fn cur_inst(&self) -> &str {
        &self.lines[self.counter]
    }

    pub(crate) fn has_more_commands(&self) -> bool {
        self.counter < self.lines.len()
    }

    pub(crate) fn advance(&mut self) {
        if self.has_more_commands() {
            self.counter += 1
        }
    }

    pub(crate) fn command_type(&self) -> Cmd {
        if self.is_a() {
            return Cmd::ACommand;
        }
        if self.is_c() {
            return Cmd::CCommand;
        }
        Cmd::LCommand
    }

    pub(crate) fn symbol(&self) -> Option<&str> {
        if self.is_a() || self.is_label() {
            return Some(self.cur_inst().split('@').nth(1).unwrap());
        }
        None
    }

    pub(crate) fn dest(&self) -> Option<&str> {
        if self.is_c() && self.cur_inst().contains('=') {
            return Some(self.cur_inst().split('=').next().unwrap());
        }
        None
    }

    pub(crate) fn comp(&self) -> Option<&str> {
        if self.is_c() {
            let inst = self.cur_inst();
            if inst.contains('=') {
                let inst = inst.split('=').nth(1).unwrap();
                if inst.contains(';') {
                    return Some(inst.split(';').next().unwrap());
                }
                return Some(inst);
            }
            if inst.contains(';') {
                return Some(inst.split(';').next().unwrap());
            }

            return Some(inst);
        }
        None
    }

    pub(crate) fn jump(&self) -> Option<&str> {
        if self.is_c() && self.cur_inst().contains(';') {
            return Some(self.cur_inst().split(';').nth(1).unwrap());
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_jump() {
        let name = "./input/MaxL.asm".to_string();
        let mut p = Parser::new(name);
        p.init();
        let test_arr = [
            None,
            None,
            None,
            None,
            None,
            Some("JGT"),
            None,
            None,
            None,
            Some("JMP"),
            None,
            None,
            None,
            None,
            None,
            Some("JMP"),
        ];
        let mut count = 0;
        while p.has_more_commands() {
            assert_eq!(p.jump(), test_arr[count]);
            p.advance();
            count += 1;
        }
    }

    #[test]
    fn test_dest() {
        let name = "./input/MaxL.asm".to_string();
        let mut p = Parser::new(name);
        p.init();
        let test_arr = [
            None,
            Some("D"),
            None,
            Some("D"),
            None,
            None,
            None,
            Some("D"),
            None,
            None,
            None,
            Some("D"),
            None,
            Some("M"),
            None,
            None,
        ];
        let mut count = 0;
        while p.has_more_commands() {
            assert_eq!(p.dest(), test_arr[count]);
            p.advance();
            count += 1;
        }
    }

    #[test]
    fn test_comp() {
        let name = "./input/MaxL.asm".to_string();
        let mut p = Parser::new(name);
        p.init();
        let test_arr = [
            None,
            Some("M"),
            None,
            Some("D-M"),
            None,
            Some("D"),
            None,
            Some("M"),
            None,
            Some("0"),
            None,
            Some("M"),
            None,
            Some("D"),
            None,
            Some("0"),
        ];
        let mut count = 0;
        while p.has_more_commands() {
            assert_eq!(p.comp(), test_arr[count]);
            p.advance();
            count += 1;
        }
    }
}
