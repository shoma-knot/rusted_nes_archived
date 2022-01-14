extern crate image;

use core::panic;
use std::convert::TryInto;

use image::RgbaImage;
use piston_window::{Texture, TextureSettings};

pub struct Ppu {
    memory: [u8; 0x4000],
    ptr: usize,
    window: piston_window::PistonWindow,
    images: [NesImage; 512],
}

impl Ppu {
    pub fn new() -> Self {
        return Ppu {
            memory: [0; 0x4000],
            ptr: 0x0000,
            window: piston_window::WindowSettings::new("Hello, World!", [256, 240])
                .exit_on_esc(true)
                .resizable(false)
                .build()
                .unwrap(),
            images: [NesImage::empty(); 512],
        };
    }

    pub fn read_chr_rom(&mut self, chr: &[u8]) {
        self.images = Ppu::create_image_array(chr);
        println!("H: {:?}", self.images[0x48].dot)
    }

    fn create_image_array(data: &[u8]) -> [NesImage; 512] {
        let mut images = [NesImage::empty(); 512];
        if data.len() > 512 * 16 {
            panic!("キャラクタデータが大きすぎます");
        }

        for i in 0..(data.len() / 16) {
            images[i] = NesImage::new(data[i * 16..(i + 1) * 16].try_into().unwrap());
        }

        return images;
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

    pub fn get_memory(&self, start: usize, end: usize) -> &[u8] {
        return &self.memory[start..end];
    }

    fn get_nes_image(&self, index: usize) -> &NesImage {
        return &self.images[index];
    }

    pub fn draw(&mut self) {
        let name_table: [u8; 0x03C0] = self.get_memory(0x2000, 0x23C0).try_into().unwrap();
        let color: [[u8; 4]; 4] = [
            [0x00, 0x00, 0x00, 0xff],
            [0x55, 0x55, 0x55, 0xff],
            [0xaa, 0xaa, 0xaa, 0xff],
            [0xff, 0xff, 0xff, 0xff],
        ];

        let mut tex = RgbaImage::new(256, 240);
        for i in 0..0x03C0 {
            let x = i % 32;
            let y = i / 32;

            let image = self.get_nes_image(name_table[i] as usize);

            for px in 0..8 {
                for py in 0..8 {
                    /*
                    img.put_pixel(
                        (x * 8 + px) as u32,
                        (y * 8 + py) as u32,
                        Rgba(color[image.dot[px][py] as usize]),
                    );
                    */
                    tex.get_pixel_mut((x * 8 + px) as u32, (y * 8 + py) as u32)
                        .0 = color[image.dot[px][py] as usize];
                }
            }
        }

        let img = Texture::from_image(
            &mut self.window.create_texture_context(),
            &tex,
            &TextureSettings::new(),
        )
        .unwrap();

        let ev = match self.window.next() {
            Some(v) => v,
            None => {
                panic!("ウィンドウが閉じられました");
            }
        };
        self.window.draw_2d(&ev, |context, graphics, _devices| {
            piston_window::clear([0.0; 4], graphics);
            piston_window::image(&img, context.transform, graphics);
        });
    }
}

struct NesImage {
    dot: [[u8; 8]; 8],
}

impl NesImage {
    pub fn new(data: [u8; 16]) -> Self {
        let mut dot: [[u8; 8]; 8] = [[0; 8]; 8];

        for i in 0..16 {
            let mut byte = data[i];
            for j in 0..8 {
                dot[7 - j][i % 8] += byte & 0x1 << i / 8;
                byte >>= 1;
            }
        }
        return NesImage { dot: dot };
    }

    pub fn empty() -> Self {
        return NesImage { dot: [[0; 8]; 8] };
    }
}

impl Clone for NesImage {
    fn clone(&self) -> Self {
        Self {
            dot: self.dot.clone(),
        }
    }
}

impl Copy for NesImage {}
