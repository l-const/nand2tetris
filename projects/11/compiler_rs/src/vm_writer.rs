pub(crate) struct VmWriter<'fname> {
    fd: &'fname str,
}

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

impl<'a> VmWriter<'a> {
    pub(crate) fn new(fd: &'a str) -> Self {
        VmWriter { fd }
    }

    pub(crate) fn write_push(&self, seg: Segment) {}

    pub(crate) fn write_pop(&self, seg: Segment) {}

    pub(crate) fn write_arithm(&self, cmd: Command) {}

    pub(crate) fn write_label(&self, label: &str) {}

    pub(crate) fn write_goto(&self, label: &str) {}

    pub(crate) fn write_if(&self, label: &str) {}

    pub(crate) fn write_call(&self, name: &str, n_args: u8) {}

    pub(crate) fn write_function(&self, name: &str, n_args: u8) {}

    pub(crate) fn write_return(&self) {}

    fn close(&self) {}
}
