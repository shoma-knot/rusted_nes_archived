mod ppu;
mod ram;
mod register;

use ram::Memory;
use register::Register;

use std::{fs::File, io::Read};

pub struct Nes {
    ram: Memory,
    register: Register,
}

impl Nes {
    pub fn new() -> Nes {
        return Nes {
            ram: Memory::new(),
            register: Register::new(),
        };
    }

    pub fn fetch(self, path: &String) {
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

        self.ram
            .readPRGROM(&buf[0x10..(0x10 + units_prg * 0x4000) as usize])
    }
}
