"""
Parser for the Hack Assembly language.
"""
import sys
from typing import List, Optional
from enum import Enum
from pprint import pformat


class Parser:
    """
    Encapsulates access to the input code. Reads an assembly language com-
    mand, parses it, and provides convenient access to the command’s components
    (fields and symbols). In addition, removes all white space and comments.
    """

    def __init__(self, filename: str):
        contents: List[str] = list(map(str.strip, open(filename, "r").readlines()))
        self.filename = filename
        self.lines = contents
        self.__clean_line_comments()
        self.__clean_inline_comments()
        self.lines = [s.replace(" ", "") for s in self.lines]
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
        with open(self.filename + ".ir", "w+") as file:
            file.writelines(
                ["\n" + l if p != 0 else l for p, l in enumerate(self.lines)]
            )

    def _loc(self) -> int:
        """
        Returns loc.
        lines of code.
        """
        return len(self.lines)

    def _cur(self):
        return self.lines[self.counter]

    def _is_label(self) -> bool:
        """
        Checks if instruction is Label (END)
        """
        return self.lines[self.counter].startswith("(") and self.lines[
            self.counter
        ].endswith(")")

    def _is_a(self) -> bool:
        """
        Checks if is A-instruction: eg. @100 , @label
        """
        return self.lines[self.counter].startswith("@")

    def _is_c(self) -> bool:
        return not self._is_a() or self._is_label()

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
            # if self._is_label():
            #    self.counter += 1

    def command_type(self) -> Enum:
        """
        Returns the type of the current command: C_COMMAND, L_COMMAND
        or A_COMMAND.
        """
        if self._is_a():
            return Command.A_COMMAND
        if self._is_c():
            return Command.C_COMMAND
        return Command.L_COMMAND

    def symbol(self) -> Optional[str]:
        """
        Returns the symbol or decimal Xxx of the current command
        @Xxx or (Xxx) . Should be called only when commandType() is
        A_COMMAND or L_COMMAND.
        """
        if self._is_a() or self._is_label():
            return self._cur().split("@")[1]

    def dest(self) -> Optional[str]:
        """
        Returns the dest mnemonic in the current C-command (8 possi-bilities).
         Should be called only when commandType() is C_COMMAND.
        """
        if self._is_c():
            inst = self._cur()
            if "=" in inst:
                return inst.split("=")[0]
        return None

    def comp(self) -> Optional[str]:
        """
        Returns the comp mnemonic in the current C-command (28 possi-bilities).
         Should be called only when commandType() is C_COMMAND.
        """
        if self._is_c():
            inst = self._cur()
            if "=" in inst:
                if ";" in inst:
                    return inst.split("=")[1].split(";")[0]
                else:
                    return inst.split("=")[1]
            elif ";" in inst:
                return inst.split(";")[0]

            return inst
        return None

    def jump(self) -> Optional[str]:
        """
        Returns the jump mnemonic in the current C-command (8 possi-bilities).
         Should be called only when commandType() is C_COMMAND.
        """
        if self._is_c():
            inst = self._cur()
            if ";" in inst:
                return inst.split(";")[1]
        return None


class Command(Enum):
    """
    Enumeration with three variants.
    Represisanting the three different  type of instructions
    of Hack assembly.
    """

    A_COMMAND = 0
    C_COMMAND = 1
    L_COMMAND = 2


def parser_demo():
    par = Parser(sys.argv[1])  # ._write_to_file()

    while par.has_more_commands():

        print(
            f"{par}, Current_inst: {par._cur()} dest: {par.dest()},   \
         comp: {par.comp()},  jump: {par.jump()} , A-inst?: {par.symbol()}"
        )
        par.advance()


def test_jump():
    test_arr = [
        None,
        None,
        None,
        None,
        None,
        "JGT",
        None,
        None,
        None,
        "JMP",
        None,
        None,
        None,
        None,
        None,
        "JMP",
    ]
    par = Parser("../max/MaxL.asm")
    i = 0
    while par.has_more_commands():
        assert par.jump() == test_arr[i]
        par.advance()
        i += 1


def test_comp():
    test_arr = [
        None,
        "M",
        None,
        "D-M",
        None,
        "D",
        None,
        "M",
        None,
        "0",
        None,
        "M",
        None,
        "D",
        None,
        "0",
    ]
    par = Parser("../max/MaxL.asm")
    i = 0
    while par.has_more_commands():
        # print(par.comp())
        assert par.comp() == test_arr[i]
        par.advance()
        i += 1


def test_dest():
    test_arr = [
        None,
        "D",
        None,
        "D",
        None,
        None,
        None,
        "D",
        None,
        None,
        None,
        "D",
        None,
        "M",
        None,
        None,
    ]
    par = Parser("../max/MaxL.asm")
    i = 0
    while par.has_more_commands():
        assert par.dest() == test_arr[i]
        par.advance()
        i += 1
