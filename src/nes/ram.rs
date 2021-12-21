pub struct Memory {
    ram: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Memory {
        return Memory { ram: [0; 0x10000] };
    }

    pub fn getU8Data(&self, index: usize) -> u8 {
        return self.ram[index];
    }

    pub fn setU8Data(&mut self, index: usize, value: u8) {
        self.ram[index] = value;
    }

    pub fn get_ppu_io(&self) -> [u8; 8] {
        let mut res: [u8; 8] = [0; 8];
        for i in 0..8 {
            res[i] = self.getU8Data(0x2000 + i);
        }
        return res;
    }

    pub fn reset_ppu_io(&mut self) {
        for i in 0..8 {
            self.setU8Data(0x2000 + i, 0);
        }
    }

    pub fn readPRGROM(&mut self, prg: &[u8]) {
        if prg.len() > 0x8000 {
            panic!("プログラムの長さが長すぎます")
        }
        let mut ptr: usize = 0;
        for byte in prg.iter() {
            // プログラムデータは0x8000から
            self.ram[0x8000 + ptr] = *byte;
            ptr += 1;
        }
    }
}
