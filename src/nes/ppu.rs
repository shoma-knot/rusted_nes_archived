use std::{io::Read, mem, ptr};

pub struct Ppu {
    sprite: [Sprite; 256],
}

impl Ppu {
    // TODO: スプライト初期化方法をもう少し考える
    pub fn new(data: &[u8]) -> Ppu {
        let mut sprites: [Sprite; 256];
        unsafe {
            sprites = mem::MaybeUninit::uninit().assume_init();
            for i in 0..data.len() / 16 {
                ptr::write(&mut sprites[i], Sprite::new(&data[i * 16..(i + 1) * 16]));
            }
        }

        return Ppu { sprite: sprites };
    }
}

pub struct Sprite {
    byte: [u8; 16],
}

impl Sprite {
    pub fn new(mut data: &[u8]) -> Sprite {
        if data.len() != 16 {
            panic!("スプライトのサイズが不正です")
        }
        let mut byte: [u8; 16] = [0; 16];
        data.read_exact(&mut byte)
            .expect("スプライト作成に失敗しました");
        return Sprite { byte: byte };
    }
}
