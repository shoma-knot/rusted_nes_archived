use std::{convert::TryInto, io::Read, mem, ptr};

pub struct Ppu {
    memory: [u8; 0x4000],
    ptr: usize,
    io_mirror: [u8; 0x8],
}

impl Ppu {
    pub fn new() -> Self {
        return Ppu {
            memory: [0; 0x4000],
            ptr: 0x0000,
            io_mirror: [0; 0x8],
        };
    }

    pub fn update(&mut self, io: &[u8; 8]) {
        // 0x2006への書き込みがなされたら
        if io[6] != 0 {
            self.ptr &= 0x00ff;
            self.ptr <<= 8;
            self.ptr += io[6] as usize;
        }
        // 0x2007への書き込みがなされたら
        if io[7] != 0 {
            self.memory[self.ptr] = io[7];
            self.ptr += 1;
        }
    }

    pub fn getMemory(&self, start: usize, end: usize) -> &[u8] {
        return &self.memory[start..end];
    }
}
