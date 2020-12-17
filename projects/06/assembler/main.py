"""
Module that deals with I/O.
"""
import sys
from typing import List
from parser import Parser 
from codegen import Code
from symbol_table import SymbolTable 
from pprint import pprint

def parse_file(filename: str) -> int:
    """
    Parsing the asm file.

    Args:
        filename : str
    """
    contents: List[str] = list(map(str.strip, open(filename, "r").readlines()))
    print(f"file.length = {len(contents)}")
    pprint(contents)
    
    with open(filename+ ".ir", "w+") as f:
        f.writelines(["\n" + l if p !=0 else l  for p, l in enumerate(contents)])
    return len(contents)

def main():
    """
    Head of execution for the assembler.
    """
    #parser = Parser()
    code_gen = Code()
    
    if len(sys.argv) > 1:
        parse_file(sys.argv[1])


def test_parse_file():
    """
    Test for parsing file contents.
    """
    filename = "../max/Max.asm"
    assert parse_file(filename) == 26
    filename = "../max/MaxL.asm"
    assert parse_file(filename) == 23
    filename = "../pong/Pong.asm"
    assert parse_file(filename) == 28375
    filename = "../pong/PongL.asm"
    assert parse_file(filename) == 27490
    filename = "../rect/RectL.asm"
    assert parse_file(filename) == 32
    filename = "../rect/Rect.asm"
    assert parse_file(filename) == 35

if __name__ == "__main__":
    main()
