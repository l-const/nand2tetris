// Translates Hack assembly language mnemonics into binary codes.
#[allow(dead_code)]
pub(crate) struct Code;

#[allow(dead_code)]
impl Code {
    // Returns the binary code of the dest mnemonic

    pub(crate) fn dest(&self, mnemonic: Option<&str>) -> &'static str {
        match mnemonic {
            Some(mne) => match mne {
                "M" => "001",
                "D" => "010",
                "MD" => "011",
                "A" => "100",
                "AM" => "101",
                "AD" => "110",
                _ => "111",
            },
            _ => "000",
        }
    }

    // Returns the binary code of the comp mnemonic
    pub(crate) fn comp(&self, mnemonic: &str) -> &'static str {
        match mnemonic {
            "0" => "0101010",
            "1" => "0111111",
            "-1" => "0111010",
            "D" => "0001100",
            "A" => "0110000",
            "M" => "1110000",
            "!D" => "0001101",
            "!A" => "0110001",
            "!M" => "1110001",
            "-D" => "0001111",
            "-A" => "0110011",
            "-M" => "1110011",
            "D+1" => "0011111",
            "A+1" => "0110111",
            "M+1" => "1110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "M-1" => "1110010",
            "D+A" => "0000010",
            "D+M" => "1000010",
            "D-A" => "0010011",
            "D-M" => "1010011",
            "A-D" => "0000111",
            "M-D" => "1000111",
            "D&A" => "0000000",
            "D&M" => "1000000",
            "D|A" => "0010101",
            _ => "1010101",
        }
    }

    // Returns the binary code of the jump mnemonic
    pub(crate) fn jump(&self, mnemonic: Option<&str>) -> &'static str {
        match mnemonic {
            Some(mne) => match mne {
                "JGT" => "001",
                "JEQ" => "010",
                "JGE" => "011",
                "JLT" => "100",
                "JNE" => "101",
                "JLE" => "110",
                _ => "111",
            },
            _ => "000",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump() {
        let c = Code {};
        assert_eq!(c.jump(Some("JLT")), "100")
    }

    #[test]
    fn test_dest() {
        let c = Code {};
        assert_eq!(c.dest(Some("M")), "001")
    }

    #[test]
    fn test_comp() {
        let c = Code {};
        assert_eq!(c.comp("M-D"), "1000111");
    }
}
