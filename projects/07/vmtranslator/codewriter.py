import os


class CodeWriter:

    cmd_dict = {
        1: "arithmetic",
        2: "push",
        3: "pop",
        4: "label",
        5: "goto",
        6: "if",
        7: "function",
        8: "return",
        9: "call",
    }

    def __init__(self, source: str) -> None:
        super().__init__()
        self.source = source
        self.is_dir = os.path.isdir(self.source)
        self._output = []
        self.out_file = open(self._get_out_file(), "w+")
        self.counter = 0

    def set_filename(self, name: str):
        pass

    def _get_sp(self):  # @SP
        self.out_file.write("\n@SP\nA=M")

    def inc_sp(self):  # SP++
        self.out_file.write("\n@SP\nM=M+1")

    def dec_sp(self):  # SP--
        self.out_file.write("\n@SP\nM=M-1")

    def get_constant(self, index: int):  # D = index
        self.out_file.write(f"\n@{index}\nD=A")

    def push(self, reg="D"):  # *SP=D,A , SP++
        self._get_sp()
        self.out_file.write(f"\nM={reg}")  # M = D, M = A
        self.inc_sp()

    def write_comment(self, cmd, segment=None, index=None):
        
        if segment is None:
            segment = ""
        if index is None:
            index = ""
        self.out_file.write(f"\n//{cmd} {segment} {index}")

    def pop(self, reg="M"):  # *D,A = *SP , SP--
        self._get_sp()
        self.out_file.write(f"\nD={reg}")  # D-M, D=A
        self.dec_sp()

    def binary(self, cmd: str):
        if cmd == "add":
            self.write_comment(f"pop into D")
            self.pop() # pop into D
            self.write_comment(f"pop into A")
            self.pop("A") # pop into A
            self.write_comment(f"add operands D=D+A")
            self.out_file.write("\nD=D+A") # add
            self.write_comment(f"push D into stack *SP = D")
            self.push()  # push D into stack  *SP = D

    def unary(self, cmd: str):
        pass


    def write_push_pop(self, cmd: int, segment: str, index: int):
        self.write_comment(CodeWriter.cmd_dict[cmd], segment, index)
        if segment == "constant" and cmd == 2:
            self.get_constant(index)
            self.push()
        if cmd == 3:
            self.pop()

    def write_arithmetic(self, cmd: str):
        self.write_comment(cmd)
        if cmd != "not" and cmd != "neg":
            self.binary(cmd)
        else:
            self.unary(cmd)

    def _get_out_file(self):
        if self.is_dir:
            out_file = self.source + ".asm"
        else:
            out_file = self.source.replace(".vm", ".asm")
        return out_file

    def close(self):
        self.out_file.close()
