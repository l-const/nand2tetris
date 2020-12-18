"""
Module that deals with I/O.
"""
import sys
from typing import List

from parser import Command, Parser
from codegen import Code
from symbol_table import SymbolTable


class Assembler:
    def __init__(self, filename) -> None:
        self.filename = filename
        self.parser = Parser(filename)
        self.code_gen = Code()
        self.sym_table = SymbolTable()
        self._output: List = []
        self._base_symbol_addr = 16

    def first_pass(self):
        parser = self.parser
        s_table = self.sym_table
        while parser.has_more_commands():
            if parser._is_label():
                # if is label then:
                # the label instruction must be removed
                # and its position will  represent the new position
                # the position that the next instruction will hold once removed
                # the counter does not need to be increased
                s_table.add_entry(parser._cur(), parser.counter)
                parser.lines.remove(parser._cur())

            else:
                parser.advance()
        # Re-initialize the parser
        parser.counter = 0

    def second_pass(self):
        parser = self.parser
        s_table = self.sym_table
        while parser.has_more_commands():
            sym = parser.symbol()
            if parser.command_type() == Command.A_COMMAND and not sym.isnumeric():
                # @sum
                if not s_table.contains(sym):
                    s_table.add_entry(sym, self._base_symbol_addr)
                    self._output.append(f"{self._base_symbol_addr:016b}")
                    self._base_symbol_addr += 1
                else:
                    addr = s_table.get_address(sym)
                    self._output.append(f"{addr:016b}")
            elif parser.command_type() == Command.A_COMMAND and sym.isnumeric():
                # @100
                # print(parser.counter)
                self._output.append(f"{int(sym):016b}")
            else:
                # C command
                jump_asm = parser.jump()
                dest_asm = parser.dest()
                comp_asm = parser.comp()
                jump_hack = self.code_gen.jump(jump_asm)
                dest_hack = self.code_gen.dest(dest_asm)
                comp_hack = self.code_gen.comp(comp_asm)
                self._output.append("111" + comp_hack + dest_hack + jump_hack)

            parser.advance()

    def gen(self):
        self.name = self.filename.split(".")[0]
        self.out_file = self.name + ".hack"
        with open(self.out_file, "w+") as out_f:
            out_f.writelines(
                ["\n" + l if p != 0 else l for p, l in enumerate(self._output)]
            )

    def run(self):
        self.first_pass()
        self.second_pass()
        self.gen()


def main():
    """
    Head of execution for the assembler.
    """
    if len(sys.argv) > 1:
        assembler = Assembler(sys.argv[1])
        assembler.run()

    else:
        print("Please provide a filepath! -> $ python assembler.py [filepath]")


def test_until_first_pass():
    """
    Test for the First pass.
    """

    ass = Assembler(sys.argv[1])
    ass.first_pass()
    print(str(ass.parser) + "\n")
    print(ass.sym_table)
    return ass


def test_until_second_pass():
    """
    Test for the second pass.
    """
    ass = test_until_first_pass()
    ass.second_pass()
    print(ass._output)


if __name__ == "__main__":
    main()
