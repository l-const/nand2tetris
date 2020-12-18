"""
Module that deals with I/O.
"""
import sys
from typing import List
from parser import Parser
from codegen import Code
from symbol_table import SymbolTable
from pprint import pprint


class Assembler:
    def __init__(self, filename) -> None:
        self.parser = Parser(filename)
        self.code_gen = Code()
        self.sym_table = SymbolTable()

    def run(self):
        pass

    def first_pass():
        pass

    def second_pass():
        pass

def main():
    """
    Head of execution for the assembler.
    """
    # assembler = Assembler

    if len(sys.argv) > 1:
        pass


if __name__ == "__main__":
    main()
