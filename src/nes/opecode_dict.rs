use std::collections;
pub struct OpecodeDict {
    table: collections::HashMap<u8, Opecode>,
}

impl OpecodeDict {
    pub fn new() -> Self {
        let mut table: collections::HashMap<u8, Opecode> = collections::HashMap::new();
        table.insert(0x4C, Opecode::new(OpecodeName::JMP, AddressingMode::ABS));
        table.insert(0x78, Opecode::new(OpecodeName::SEI, AddressingMode::IMP));
        table.insert(0x88, Opecode::new(OpecodeName::DEY, AddressingMode::IMP));
        table.insert(0x9A, Opecode::new(OpecodeName::TSX, AddressingMode::IMP));
        table.insert(0xA0, Opecode::new(OpecodeName::LDY, AddressingMode::IMD));
        table.insert(0xA2, Opecode::new(OpecodeName::LDX, AddressingMode::IMD));
        table.insert(0xA9, Opecode::new(OpecodeName::LDA, AddressingMode::IMD));
        table.insert(0x8D, Opecode::new(OpecodeName::STA, AddressingMode::ABS));
        table.insert(0xBD, Opecode::new(OpecodeName::LDA, AddressingMode::ABX));
        table.insert(0xE8, Opecode::new(OpecodeName::INX, AddressingMode::IMP));
        table.insert(0xD0, Opecode::new(OpecodeName::BNE, AddressingMode::REL));
        return OpecodeDict { table: table };
    }

    pub fn searchOpecode(&self, opecode: &u8) -> Opecode {
        let v: Opecode = match self.table.get(opecode) {
            Some(v) => v.clone(),
            None => {
                panic!("不明なオペコードが渡されました (0x{:02X})", opecode);
            }
        };
        return v;
    }
}

pub struct Opecode {
    pub name: OpecodeName,
    pub mode: AddressingMode,
}

impl Opecode {
    pub fn new(name: OpecodeName, mode: AddressingMode) -> Self {
        return Opecode {
            name: name,
            mode: mode,
        };
    }
}

impl Clone for Opecode {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            mode: self.mode.clone(),
        }
    }
    fn clone_from(&mut self, source: &Self) {
        self.name = source.name.clone();
        self.mode = source.mode.clone();
    }
}

pub enum OpecodeName {
    JMP,
    SEI,
    DEY,
    TSX,
    LDY,
    LDX,
    LDA,
    STA,
    INX,
    BNE,
}

impl Clone for OpecodeName {
    fn clone(&self) -> Self {
        match self {
            Self::JMP => Self::JMP,
            Self::SEI => Self::SEI,
            Self::DEY => Self::DEY,
            Self::TSX => Self::TSX,
            Self::LDY => Self::LDY,
            Self::LDX => Self::LDX,
            Self::LDA => Self::LDA,
            Self::STA => Self::STA,
            Self::INX => Self::INX,
            Self::BNE => Self::BNE,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match self {
            Self::JMP => Self::JMP,
            Self::SEI => Self::SEI,
            Self::DEY => Self::DEY,
            Self::TSX => Self::TSX,
            Self::LDY => Self::LDY,
            Self::LDX => Self::LDX,
            Self::LDA => Self::LDA,
            Self::STA => Self::STA,
            Self::INX => Self::INX,
            Self::BNE => Self::BNE,
        };
    }
}

pub enum AddressingMode {
    IMP,
    IMD,
    REL,
    ABS,
    ABX,
}

impl AddressingMode {
    pub fn getOperandNum(&self) -> usize {
        return match self {
            AddressingMode::IMP => 1,
            AddressingMode::IMD => 2,
            AddressingMode::REL => 2,
            AddressingMode::ABX => 3,
            AddressingMode::ABS => 3,
        };
    }
}

impl Clone for AddressingMode {
    fn clone(&self) -> Self {
        match self {
            Self::IMP => Self::IMP,
            Self::IMD => Self::IMD,
            Self::REL => Self::REL,
            Self::ABS => Self::ABS,
            Self::ABX => Self::ABX,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match self {
            Self::IMP => Self::IMP,
            Self::IMD => Self::IMD,
            Self::REL => Self::REL,
            Self::ABS => Self::ABS,
            Self::ABX => Self::ABX,
        };
    }
}
