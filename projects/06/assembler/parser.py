"""
Parser for the Hack Assembly language.
"""
import sys
from typing import List


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
        with open(self.filename + ".ir", "w+") as f:
            f.writelines(["\n" + l if p != 0 else l for p, l in enumerate(self.lines)])

    def _loc(self) -> int:
        """
        Returns loc.
        lines of code.
        """
        return len(self.lines)

    def is_label(self) -< bool:
        """
        Checks if instruction is Label (END)
        """
        return self.lines[self.counter].startswith("(") and self.lines[
            self.counter
        ].endswith(")")

    def is_A(self):
        """
        Checks if is A-instruction: eg. @100 , @label
        """
        return self.lines[self.counter].startswith("@")

    def is_C(self):
        return not self.is_A() or self.is_label()

    def advance(self):
        """
        Reads the next command from the input and makes it the current
        command. Should be called only if hasMoreCommands() is true.
        Initially there is no current command.
        """
        pass

    def command_type(self):
        """
        Returns the type of the current command: C_COMMAND, L_COMMAND
        or A_COMMAND.
        """
        pass

    def symbol(self):
        """
        Returns the symbol or decimal Xxx of the current command
        @Xxx or (Xxx) . Should be called only when commandType() is
        A_COMMAND or L_COMMAND.
        """
        pass

    def dest(self):
        """
        Returns the dest mnemonic in the current C-command (8 possi-bilities).
         Should be called only when commandType() is C_COMMAND.
        """
        pass


def test_parser():
    parser = Parser(sys.argv[1])._write_to_file()


if __name__ == "__main__":
    test_parser()
