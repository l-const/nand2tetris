"""
Module that deals with I/O.
"""
import sys
from typing import List
from .parser import Command, Parser
from .codegen import Code
from .symbol_table import SymbolTable
from pprint import pprint


class Assembler:
    def __init__(self, filename) -> None:
        self.parser = Parser(filename)
        self.code_gen = Code()
        self.sym_table = SymbolTable()
        self._output: List = []
        self._base_symbol_addr = 16

    def first_pass(self):
        parser = self.parser
        s_table = self.sym_table
        while parser.has_more_commands():
            if parser._cur()._is_label():
                # if is label then:
                # the label instruction must be removed
                # and its position will  represent the new position
                # the position that the next instruction will hold once removed
                # the counter does not need to be increased
                s_table.add_entry(parser._cur(), parser.counter)
                parser.remove(parser._cur())

            else:
                parser.advance()
        # Re-initialize the parser
        parser.counter = 0

    def process_address(self, addr: int):
        pass


    def second_pass(self):
        parser = self.parser
        s_table = self.sym_table
        while parser.has_more_commands():
            if parser.command_type() == Command.A_COMMAND and  not parser.symbol().isnumeric():
                sym = parser.symbol()
                if not s_table.contains(sym):
                    s_table.add_entry(sym, self._base_symbol_addr)
                    self._base_symbol_addr += 1
                else:
                    addr = s_table.get_address(sym)
                    self._output[parser.counter] = f"{addr:016b}"

            else:
                # C command
                pass        
            parser.advance()

    def code_gen(self):
        self.name = self.filename.split(".")[0]
        self.out_file = self.name + ".hack"
        with open(self.out_file, "w+") as out_f:
            out_f.writelines(self._output)

    def run(self):
        self.first_pass()
        self.second_pass()
        self.code_gen()


def main():
    """
    Head of execution for the assembler.
    """
    if len(sys.argv) > 1:
        assembler = Assembler(sys.argv[1])
        # assembler.run()
    else:
        print("Please provide a filepath! -> $ python assembler.py [filepath]")

if __name__ == "__main__":
    main()
