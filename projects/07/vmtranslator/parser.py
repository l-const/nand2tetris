"""
Parser for the Hack  Vm language.
"""
import sys
from typing import List, Optional
from pprint import pformat
from constants import *


class Parser:
    def __init__(self, filename: str):
        contents: List[str] = list(map(str.strip, open(filename, "r").readlines()))
        self.filename = filename
        self.lines = contents
        self.__clean_line_comments()
        self.__clean_inline_comments()
        self.lines = [s.split(" ") for s in self.lines]
        self.counter = 0

    def __str__(self) -> str:
        return pformat(
            f"Parser([{self.counter}/{len(self.lines) - 1}]) ,Contents: {self.lines}."
        )

    def __clean_line_comments(self):
        """
        Cleans line comments and empty lines.
        """
        self.lines = [l for l in self.lines if not l.startswith("//") and len(l) != 0]

    def __clean_inline_comments(self):
        """
        Cleans inline comments.
        """
        self.lines = [l.split("//")[0] if "//" in l else l for l in self.lines]

    def _write_to_file(self):
        """
        Helper function.
        Writes intermidiate results from the different parsing stages.
        """
        with open(self.filename + ".asm", "w+") as file:
            file.writelines(
                ["\n" + l if p != 0 else l for p, l in enumerate(self.lines)]
            )

    def _cur(self):
        return self.lines[self.counter]

    def line_len(self) -> int:
        return len(self._cur())

    def has_more_commands(self):
        """
        Are there more commands in the input?
        """
        return self.counter < len(self.lines)

    def advance(self):
        """
        Reads the next command from the input and makes it the current
        command. Should be called only if hasMoreCommands() is true.
        Initially there is no current command.
        """
        if self.has_more_commands():
            self.counter += 1

    def command_type(self) -> int:
        """
        Returns the type of the current command.
        """
        cur_inst = self._cur()
        if "push" in cur_inst:
            return C_PUSH
        elif "pop" in cur_inst:
            return C_POP
        elif "if" in cur_inst:
            return C_IF
        elif "label" in cur_inst:
            return C_LABEL
        elif "goto" in cur_inst:
            return C_GOTO
        elif "return" in cur_inst:
            return C_RETURN
        elif "call" in cur_inst:
            return C_CALL
        elif "function" in cur_inst:
            return C_FUNCTION
        else:
            return C_ARITHMETIC

    def arg1(self) -> Optional[str]:
        """
        Returns the first argument of the current
        command. In the case of C_ARITHMETIC,
        the command itself ( add , sub , etc.) is
        returned. Should not be called if the current
        command is C_RETURN
        """
        if self.line_len() > 1:
            return self._cur()[1]

    def arg2(self) -> Optional[int]:
        """
        Returns the second argument of the current
        command. Should be called only if the
        current command is C_PUSH, C_POP,
        C_FUNCTION, or C_CALL.
        """
        c_type = self.command_type()
        if c_type == 2 or c_type == 3 or c_type == 7 or c_type == 9:
            return int(self._cur()[2])


def parser_main():

    if len(sys.argv) > 1:
        par = Parser(sys.argv[1])
        # par._write_to_file()
        while par.has_more_commands():

            print(par._cur(), par.command_type(), par.arg2())
            par.advance()
    else:
        print("Please specify filename: => python3 parser.py [filename] ")


if __name__ == "__main__":
    parser_main()
