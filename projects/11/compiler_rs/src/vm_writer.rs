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
    MUL,
    DIV,
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
                .write(format!("push constant {}\n", index).as_bytes())
                .expect("error in push"),
            Segment::ARG => self
                .writer
                .write(format!("push argument {}\n", index).as_bytes())
                .expect("error in push"),
            Segment::LOCAL => self
                .writer
                .write(format!("push local {}\n", index).as_bytes())
                .expect("error in push"),
            Segment::STATIC => self
                .writer
                .write(format!("push static {}\n", index).as_bytes())
                .expect("error in push"),
            Segment::THIS => self
                .writer
                .write(format!("push this {}\n", index).as_bytes())
                .expect("couldn't write push"),
            Segment::THAT => self
                .writer
                .write(format!("push that {}\n", index).as_bytes())
                .expect("couldn't write push"),
            Segment::POINTER => self
                .writer
                .write(format!("push pointer {}\n", index).as_bytes())
                .expect("couldn't write push"),
            Segment::TEMP => self
                .writer
                .write(format!("push temp {}\n", index).as_bytes())
                .expect("couldn't write push"),
        };
    }

    pub(crate) fn write_pop(&mut self, seg: Segment, index: u8) {
        match seg {
            Segment::CONST => self
                .writer
                .write(format!("pop constant {}\n", index).as_bytes())
                .expect("couldn't write pop"),
            Segment::ARG => self
                .writer
                .write(format!("pop argument {}\n", index).as_bytes())
                .expect("couldn't write pop"),
            Segment::LOCAL => self
                .writer
                .write(format!("pop local {}\n", index).as_bytes())
                .expect("couldn't weite pop"),
            Segment::STATIC => self
                .writer
                .write(format!("pop static {}\n", index).as_bytes())
                .expect("couldn't weite pop"),
            Segment::THIS => self
                .writer
                .write(format!("pop this{}\n", index).as_bytes())
                .expect("couldn't weite push"),
            Segment::THAT => self
                .writer
                .write(format!("pop that {}\n", index).as_bytes())
                .expect("couldn't weite pop"),
            Segment::POINTER => self
                .writer
                .write(format!("pop pointer {}\n", index).as_bytes())
                .expect("couldn't weite pop"),
            Segment::TEMP => self
                .writer
                .write(format!("pop temp {}\n", index).as_bytes())
                .expect("couldn't write pop"),
        };
    }
    pub(crate) fn write_arithm(&mut self, cmd: Command) {
        match cmd {
            Command::ADD => self
                .writer
                .write("add\n".as_bytes())
                .expect("couldn't write add"),
            Command::SUB => self
                .writer
                .write("sub\n".as_bytes())
                .expect("couldn't write sub"),
            Command::MUL => self
                .writer
                .write("call Math.multiply 2\n".as_bytes())
                .expect("couldn't write mul"),
            Command::DIV => self
                .writer
                .write("call Math.divide 2\n".as_bytes())
                .expect("couldn't write div"),
            Command::NEG => self
                .writer
                .write("neg\n".as_bytes())
                .expect("couldn't write neg"),
            Command::EQ => self
                .writer
                .write("eq\n".as_bytes())
                .expect("couldn't write eq"),
            Command::GT => self
                .writer
                .write("gt\n".as_bytes())
                .expect("couldn't write gt"),
            Command::LT => self
                .writer
                .write("lt\n".as_bytes())
                .expect("couldn't write lt"),
            Command::AND => self
                .writer
                .write("and\n".as_bytes())
                .expect("couldn't write and"),
            Command::OR => self
                .writer
                .write("or\n".as_bytes())
                .expect("couldn't write or"),
            Command::NOT => self
                .writer
                .write("not\n".as_bytes())
                .expect("couldn't write not"),
        };
    }
    pub(crate) fn write_label(&mut self, label: &str) {
        self.writer
            .write(format!("label {}\n", label).as_bytes())
            .expect("Couldn't write label");
    }

    pub(crate) fn write_goto(&mut self, label: &str) {
        self.writer
            .write(format!("goto {}\n", label).as_bytes())
            .expect("Couldn't write goto");
    }

    pub(crate) fn write_if(&mut self, label: &str) {
        self.writer
            .write(format!("if-goto {}\n", label).as_bytes())
            .expect("Couldn't write if");
    }

    pub(crate) fn write_call(&mut self, name: &str, n_args: u8) {
        self.writer
            .write(format!("call {} {}\n", name, n_args).as_bytes())
            .expect("Couldn't write call");
    }

    pub(crate) fn write_function(&mut self, name: &str, n_args: u8) {
        self.writer
            .write(format!("function {} {}\n", name, n_args).as_bytes())
            .expect("Couldn't write function!");
    }

    pub(crate) fn write_return(&mut self) {
        self.writer
            .write("return\n".as_bytes())
            .expect("Couldn't write return");
    }

    pub(crate) fn close(&mut self) {
        self.writer.flush().expect("Couldn't flush");
    }
}
