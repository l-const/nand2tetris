from typing import Optional


class Code:
    """
    Translates Hack assembly language mnemonics into binary codes.
    """

    def __init__(self):
        pass

    def dest(self, mnemonic: Optional[str]) -> str:
        """
        Returns the binary code of the dest mnemonic
        """
        if mnemonic is None:
            result = "000"
        elif mnemonic == "M":
            result = "001"
        elif mnemonic == "D":
            result = "010"
        elif mnemonic == "MD":
            result = "011"
        elif mnemonic == "A":
            result = "100"
        elif mnemonic == "AM":
            result = "101"
        elif mnemonic == "AD":
            result = "110"
        else:
            result = "111"
        return result

    def comp(self, mnemonic: str) -> str:
        """
        Returns the binary code of the comp mnemonic
        """
        if mnemonic == "0":
            result = "0101010"
        elif mnemonic == "1":
            result = "0111111"
        elif mnemonic == "-1":
            result = "0111111"
        elif mnemonic == "D":
            result = "0111111"
        elif mnemonic == "A":
            result = "0111111"
        elif mnemonic == "M":
            result = "0111111"
        elif mnemonic == "!D":
            result = "0111111"
        elif mnemonic == "!A":
            result = "0111111"
        elif mnemonic == "!M":
            result = "0111111"
        elif mnemonic == "-D":
            result = "0111111"
        elif mnemonic == "-A":
            result = "0111111"
        elif mnemonic == "-M":
            result = "0111111"
        elif mnemonic == "D+1":
            result = "0111111"
        elif mnemonic == "A+1":
            result = "0111111"
        elif mnemonic == "M+1":
            result = "0111111"
        elif mnemonic == "D-1":
            result = "0111111"
        elif mnemonic == "A-1":
            result = "0111111"
        elif mnemonic == "M-1":
            result = "0111111"
        elif mnemonic == "D+A":
            result = "0111111"
        elif mnemonic == "D+M":
            result = "0111111"
        elif mnemonic == "D-A":
            result = "0111111"
        elif mnemonic == "D-M":
            result = "0111111"
        elif mnemonic == "A-D":
            result = "0111111"
        elif mnemonic == "M-D":
            result = "0111111"
        elif mnemonic == "D&A":
            result = "0111111"
        elif mnemonic == "D&M":
            result = "0111111"
        elif mnemonic == "D|A":
            result = "01111111"
        else:
            result = "01010101"
        return result

    def jump(self, mnemonic: Optional[str]) -> str:
        """
        Returns the binary code of the jump mnemonic
        """
        if mnemonic is None:
            result = "000"
        elif mnemonic == "JGT":
            result = "001"
        elif mnemonic == "JEQ":
            result = "010"
        elif mnemonic == "JGE":
            result = "011"
        elif mnemonic == "JLT":
            result = "100"
        elif mnemonic == "JNE":
            result = "101"
        elif mnemonic == "JLE":
            result = "110"
        else:
            result = "111"
        return result


def test_jump():
    """
    Test for the jump method.
    """
    code_gen = Code()
    assert code_gen.jump(None) == "000"
    assert code_gen.jump("JGT") == "001"
    assert code_gen.jump("JEQ") == "010"
    assert code_gen.jump("JGE") == "011"
    assert code_gen.jump("JLT") == "100"
    assert code_gen.jump("JNE") == "101"
    assert code_gen.jump("JLE") == "110"
    assert code_gen.jump("JMP") == "111"


def test_dest():
    """
    Test for dest method.
    """
    code_gen = Code()
    assert code_gen.dest(None) == "000"
    assert code_gen.dest("M") == "001"
    assert code_gen.dest("D") == "010"
    assert code_gen.dest("MD") == "011"
    assert code_gen.dest("A") == "100"
    assert code_gen.dest("AM") == "101"
    assert code_gen.dest("AD") == "110"
    assert code_gen.dest("AMD") == "111"


def test_comp():
    pass
