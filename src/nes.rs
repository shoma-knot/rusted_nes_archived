mod opecode_dict;
mod ppu;
mod ram;
mod register;
mod utility;

use opecode_dict::{AddressingMode, OpecodeDict, OpecodeName};
use ppu::Ppu;
use ram::Memory;
use register::Register;

use std::{fs::File, io::Read};

pub struct Nes {
    ram: Memory,
    register: Register,
    ppu: Ppu,
    opecode_dict: OpecodeDict,
}

impl Nes {
    pub fn new() -> Nes {
        return Nes {
            ram: Memory::new(),
            register: Register::new(),
            ppu: Ppu::new(),
            opecode_dict: OpecodeDict::new(),
        };
    }

    pub fn fetch(&mut self, path: &str) {
        let mut buf = Vec::new();

        // nesファイル読み込み
        match File::open(path) {
            Ok(mut f) => match f.read_to_end(&mut buf) {
                Ok(_) => match &buf[0..4] {
                    [0x4e, 0x45, 0x53, 0x1a] => {}
                    _ => panic!("nesファイルではありません"),
                },
                Err(_) => panic!("バッファ書き込みエラー"),
            },
            Err(_) => panic!("ファイルオープンに失敗しました"),
        }

        let units_prg = buf[4] as u16;
        let units_chr = buf[5] as u16;

        let prg_start = 0x10 as usize;
        let prg_end = prg_start + units_prg as usize * 0x4000;
        let chr_start = prg_end;
        let chr_end = chr_start + units_chr as usize * 0x2000;

        self.ram.readPRGROM(&buf[prg_start..prg_end]);

        self.ppu.read_chr_rom(&buf[chr_start..chr_end]);
    }

    pub fn read_pc_data(&self, offset: usize) -> u8 {
        return self.ram.getU8Data(self.register.get_pc() + offset);
    }

    /** 命令を読み出し、実行します */
    pub fn exec(&mut self) {
        let opecode = self.read_pc_data(0);
        let operand = self.opecode_dict.searchOpecode(&opecode);

        match operand.name {
            OpecodeName::JMP => match operand.mode {
                // JMP命令 PCを飛ばします
                AddressingMode::ABS => {
                    let addr: usize = ((self.read_pc_data(2) as u16) << 8) as usize
                        + self.read_pc_data(1) as usize;
                    self.register.set_pc(addr);
                }
                _ => panic!("未実装の関数です"),
            },
            OpecodeName::SEI => match operand.mode {
                // SET命令 割り込みを無効化します
                AddressingMode::IMP => {
                    self.register.set_i(true);
                    self.register.increment_pc(operand.mode.getOperandNum());
                }
                _ => panic!("未実装の関数です"),
            },
            OpecodeName::DEY => match operand.mode {
                // DEY命令 Yレジスタをデクリメントします
                AddressingMode::IMP => {
                    self.register.decriment_y();
                    self.register.increment_pc(operand.mode.getOperandNum());
                    let value = self.register.get_y();
                    self.register.set_n(utility::is_up_7bit(value));
                    self.register.set_z(utility::is_zero(value));
                }
                _ => panic!("未実装の関数です"),
            },
            OpecodeName::TSX => match operand.mode {
                // TSX命令 SPの値をXレジスタへ格納
                AddressingMode::IMP => {
                    let value = self.register.get_sp();
                    self.register.set_x(value);
                    self.register.increment_pc(operand.mode.getOperandNum());
                    let value = self.register.get_x();
                    self.register.set_n(utility::is_up_7bit(value));
                    self.register.set_z(utility::is_zero(value));
                }
                _ => panic!("未実装の関数です"),
            },
            OpecodeName::LDY => match operand.mode {
                // LDY命令 Yレジスタに値を格納
                AddressingMode::IMD => {
                    self.register.set_y(self.read_pc_data(1));
                    self.register.increment_pc(operand.mode.getOperandNum());
                    let value = self.register.get_y();
                    self.register.set_n(utility::is_up_7bit(value));
                    self.register.set_z(utility::is_zero(value));
                }
                _ => panic!("未実装の関数です"),
            },
            OpecodeName::LDX => match operand.mode {
                // LDX命令 Xレジスタに値を格納
                AddressingMode::IMD => {
                    self.register.set_x(self.read_pc_data(1));
                    self.register.increment_pc(operand.mode.getOperandNum());
                    let value = self.register.get_x();
                    self.register.set_n(utility::is_up_7bit(value));
                    self.register.set_z(utility::is_zero(value));
                }
                _ => panic!("未実装の関数です"),
            },
            OpecodeName::LDA => match operand.mode {
                // LDA命令 Aレジスタに値を格納
                AddressingMode::IMD => {
                    self.register.set_a(self.read_pc_data(1));
                    self.register.increment_pc(operand.mode.getOperandNum());
                    let value = self.register.get_a();
                    self.register.set_n(utility::is_up_7bit(value));
                    self.register.set_z(utility::is_zero(value));
                }
                AddressingMode::ABX => {
                    let addr: usize = ((self.read_pc_data(2) as u16) << 8) as usize
                        + self.read_pc_data(1) as usize
                        + self.register.get_x() as usize;
                    self.register.set_a(self.ram.getU8Data(addr));

                    self.register.increment_pc(operand.mode.getOperandNum());
                    let value = self.register.get_a();
                    self.register.set_n(utility::is_up_7bit(value));
                    self.register.set_z(utility::is_zero(value));
                }
                _ => panic!("未実装の関数です"),
            },
            OpecodeName::STA => match operand.mode {
                // STA命令 Aレジスタの内容を指定アドレスに代入します
                AddressingMode::ABS => {
                    let addr: usize = ((self.read_pc_data(2) as u16) << 8) as usize
                        + self.read_pc_data(1) as usize;
                    self.ram.setU8Data(addr, self.register.get_a());
                    self.register.increment_pc(operand.mode.getOperandNum());
                }
                _ => panic!("未実装の関数です"),
            },
            OpecodeName::INX => match operand.mode {
                // INX命令 Xレジスタの値をインクリメントします
                AddressingMode::IMP => {
                    self.register.incriment_x();
                    self.register.increment_pc(operand.mode.getOperandNum());
                    let value = self.register.get_x();
                    self.register.set_n(utility::is_up_7bit(value));
                    self.register.set_z(utility::is_zero(value));
                }
                _ => panic!("未実装の関数です"),
            },
            OpecodeName::BNE => match operand.mode {
                // BNE命令 Zフラグがセットされていない時に分岐します
                AddressingMode::REL => {
                    if !self.register.get_z() {
                        let offset = self.read_pc_data(1) as i8;
                        self.register.increment_pc(operand.mode.getOperandNum());
                        self.register.add_pc_signed(offset);
                    } else {
                        self.register.increment_pc(operand.mode.getOperandNum())
                    }
                }
                _ => panic!("未実装の関数です"),
            },
        }
    }

    fn get_ppu_io(&self) -> [u8; 8] {
        return self.ram.get_ppu_io();
    }

    fn ppu_io_reset(&mut self) {
        for i in 0..8 {
            self.ram.setU8Data(0x2000 + i, 0);
        }
    }

    pub fn run(&mut self) {
        loop {
            self.exec();
            self.ppu.update(&self.get_ppu_io());
            self.ppu_io_reset();
            self.ppu.draw();
        }

        println!("{:?}", self.ppu.getMemory(0x21C9, 0x21C9 + 13));
    }
}
