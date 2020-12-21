use std::collections::HashMap;
#[derive(Debug)]
pub(crate) struct SymbolTable {
    table: HashMap<String, usize>,
}

impl SymbolTable {
    pub(crate) fn new() -> Self {
        SymbolTable {
            table: [
                ("R0".to_string(), 0),
                ("R1".to_string(), 1),
                ("R2".to_string(), 2),
                ("R3".to_string(), 3),
                ("R5".to_string(), 5),
                ("R6".to_string(), 6),
                ("R7".to_string(), 7),
                ("R8".to_string(), 8),
                ("R9".to_string(), 9),
                ("R10".to_string(), 10),
                ("R11".to_string(), 11),
                ("R12".to_string(), 12),
                ("R13".to_string(), 13),
                ("R14".to_string(), 14),
                ("R15".to_string(), 15),
                ("SCREEN".to_string(), 16384),
                ("KBD".to_string(), 24576),
                ("SP".to_string(), 0),
                ("LCL".to_string(), 1),
                ("ARG".to_string(), 2),
                ("THIS".to_string(), 3),
                ("THAT".to_string(), 4),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub(crate) fn add_entry(&mut self, mut symbol: String, address: usize) {
        if symbol.contains("(") {
            symbol = symbol.split("(").nth(1).unwrap().to_owned();
            if symbol.contains(")") {
                symbol = symbol.split(")").nth(0).unwrap().to_owned();
            }
        }
        self.table.insert(symbol, address);
    }

    pub(crate) fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub(crate) fn get_address(&self, symbol: &str) -> Option<usize> {
        match self.table.get(symbol) {
            Some(&v) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut s_table = SymbolTable::new();
        assert_eq!(s_table.table.len(), 22);
        println!("{:?}", s_table);
        s_table.add_entry("test".to_string(), 100);
        assert_eq!(s_table.table.len(), 23);
        assert_eq!(s_table.contains("SCREEN"), true);
        assert_eq!(s_table.contains("not found"), false);
        assert_eq!(s_table.get_address("SCREEN"), Some(16384));
        assert_eq!(s_table.get_address("test"), Some(100));
    }
}
