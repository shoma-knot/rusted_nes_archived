pub struct Memory {
    ram: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Memory {
        return Memory { ram: [0; 0x10000] };
    }

    pub fn getU8Data(self, index: usize) -> u8 {
        return self.ram[index];
    }

    pub fn readPRGROM(mut self, prg: &[u8]) {
        if prg.len() > 0x8000 {
            panic!("プログラムの長さが長すぎます")
        }
        let ptr: usize = 0;
        for byte in prg.iter() {
            // プログラムデータは0x8000から
            self.ram[0x8000 + ptr] = *byte;
        }
    }
}
