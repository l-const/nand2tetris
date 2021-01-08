use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
pub(crate) struct VmWriter {
    writer: BufWriter<File>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Segment {
    CONST,
    ARG,
    LOCAL,
    STATIC,
    THIS,
    THAT,
    POINTER,
    TEMP,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Command {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
}

impl VmWriter {
    pub(crate) fn new(fd: &str) -> Self {
        let out_str = String::from(fd) + ".vm";
        let out_f = File::create(out_str).expect("Could't create vm file!");
        let writer = BufWriter::new(out_f);
        VmWriter { writer }
    }

    pub(crate) fn write_push(&mut self, seg: Segment, index: u8) {
        match seg {
            Segment::CONST => self
                .writer
                .write(format!("push constant {}", index).as_bytes())
                .expect("error in push"),
            Segment::ARG => self
                .writer
                .write(format!("push argument {}", index).as_bytes())
                .expect("error in push"),
            Segment::LOCAL => self
                .writer
                .write(format!("push local {}", index).as_bytes())
                .expect("error in push"),
            Segment::STATIC => self
                .writer
                .write(format!("push static  {}", index).as_bytes())
                .expect("error in push"),
            Segment::THIS => self
                .writer
                .write(format!("push this {}", index).as_bytes())
                .expect("couldn't write push"),
            Segment::THAT => self
                .writer
                .write(format!("push that {}", index).as_bytes())
                .expect("couldn't write push"),
            Segment::POINTER => self
                .writer
                .write(format!("push pointer {}", index).as_bytes())
                .expect("couldn't write push"),
            Segment::TEMP => self
                .writer
                .write(format!("push temp {}", index).as_bytes())
                .expect("couldn't write push"),
        };
    }

    pub(crate) fn write_pop(&mut self, seg: Segment, index: u8) {
        match seg {
            Segment::CONST => self
                .writer
                .write(format!("pop constant {}", index).as_bytes())
                .expect("couldn't write pop"),
            Segment::ARG => self
                .writer
                .write(format!("pop argument {}", index).as_bytes())
                .expect("couldn't write pop"),
            Segment::LOCAL => self
                .writer
                .write(format!("pop local {}", index).as_bytes())
                .expect("couldn't weite pop"),
            Segment::STATIC => self
                .writer
                .write(format!("pop static {}", index).as_bytes())
                .expect("couldn't weite pop"),
            Segment::THIS => self
                .writer
                .write(format!("pop this{}", index).as_bytes())
                .expect("couldn't weite push"),
            Segment::THAT => self
                .writer
                .write(format!("pop that {}", index).as_bytes())
                .expect("couldn't weite pop"),
            Segment::POINTER => self
                .writer
                .write(format!("pop pointer {}", index).as_bytes())
                .expect("couldn't weite pop"),
            Segment::TEMP => self
                .writer
                .write(format!("pop temp {}", index).as_bytes())
                .expect("couldn't write pop"),
        };
    }
    pub(crate) fn write_arithm(&mut self, cmd: Command) {
        match cmd {
            Command::ADD => self
                .writer
                .write("add".as_bytes())
                .expect("couldn't write add"),
            Command::SUB => self
                .writer
                .write("sub".as_bytes())
                .expect("couldn't write sub"),
            Command::NEG => self
                .writer
                .write("neg".as_bytes())
                .expect("couldn't write neg"),
            Command::EQ => self
                .writer
                .write("eq".as_bytes())
                .expect("couldn't write eq"),
            Command::GT => self
                .writer
                .write("gt".as_bytes())
                .expect("couldn't write gt"),
            Command::LT => self
                .writer
                .write("lt".as_bytes())
                .expect("couldn't write lt"),
            Command::AND => self
                .writer
                .write("and".as_bytes())
                .expect("couldn't write and"),
            Command::OR => self
                .writer
                .write("or".as_bytes())
                .expect("couldn't write or"),
            Command::NOT => self
                .writer
                .write("not".as_bytes())
                .expect("couldn't write not"),
        };
    }
    pub(crate) fn write_label(&mut self, label: &str) {
        self.writer
            .write(format!("label {}", label).as_bytes())
            .expect("Couldn't write label");
    }

    pub(crate) fn write_goto(&mut self, label: &str) {
        self.writer
            .write(format!("goto {}", label).as_bytes())
            .expect("Couldn't write goto");
    }

    pub(crate) fn write_if(&mut self, label: &str) {
        self.writer
            .write(format!("if-goto {}", label).as_bytes())
            .expect("Couldn't write if");
    }

    pub(crate) fn write_call(&mut self, name: &str, n_args: u8) {
        self.writer
            .write(format!("call {} {}", name, n_args).as_bytes())
            .expect("Couldn't write call");
    }

    pub(crate) fn write_function(&mut self, name: &str, n_args: u8) {
        self.writer
            .write(format!("function {} {}", name, n_args).as_bytes())
            .expect("Couldn't write function!");
    }

    pub(crate) fn write_return(&mut self) {
        self.writer
            .write("return".as_bytes())
            .expect("Couldn't write return");
    }

    fn close(&mut self) {
        self.writer.flush().expect("Could't flush");
    }
}
