"""
Keeps a correspondence between symbolic labels and numeric
addresses.
"""


from typing import Dict, Optional


class SymbolTable:
    """
    Creates a new empty symbol
    table.
    """

    def __init__(self):
        self.table: Dict[str, int] = {
            "R0": 0,
            "R1": 1,
            "R2": 2,
            "R3": 3,
            "R5": 5,
            "R6": 6,
            "R7": 7,
            "R8": 8,
            "R9": 9,
            "R10": 10,
            "R11": 11,
            "R12": 12,
            "R13": 13,
            "R14": 14,
            "R15": 15,
            "SCREEN": 16834,
            "KBD": 24576,
            "SP": 0,
            "LCL": 1,
            "ARG": 2,
            "THIS": 3,
            "THAT": 4,
        }

    def add_entry(self, symbol: str, address: int):
        """
        Adds the pair (symbol, address) to the table
        """
        self.table[symbol] = address

    def contains(self, symbol: str) -> bool:
        """
        Does the symbol table contain
        the given symbol?
        """
        return self.table.get(symbol) is not None

    def get_address(self, symbol: str) -> Optional[int]:
        """
        Returns the address associated
        with the symbol.
        """
        return self.table.get(symbol)


def test_symbol_table():
    """
    Test for the SymbolTable class.
    """
    sy_table = SymbolTable()
    assert len(sy_table.table) == 22
    sy_table.add_entry("test", 100)
    assert len(sy_table.table) == 23
    assert sy_table.contains("test")
    assert not sy_table.contains("not_found")
    assert sy_table.get_address("SCREEN") == 16834
    assert sy_table.get_address("test") == 100
    assert sy_table.get_address("unkown") is None
    