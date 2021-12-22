pub struct Register {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    ps: ProcessorStatusRegister,
}

impl Register {
    pub fn new() -> Register {
        return Register {
            a: 0,
            x: 0,
            y: 0,
            pc: 0x8000,
            sp: 0,
            ps: ProcessorStatusRegister::new(),
        };
    }

    pub fn get_a(&mut self) -> u8 {
        return self.a;
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn get_x(&self) -> u8 {
        return self.x;
    }

    pub fn set_x(&mut self, value: u8) {
        self.x = value;
    }

    pub fn incriment_x(&mut self) {
        self.x += 1;
    }

    pub fn get_y(&mut self) -> u8 {
        return self.y;
    }

    pub fn set_y(&mut self, value: u8) {
        self.y = value;
    }

    pub fn decriment_y(&mut self) {
        self.y -= 1;
    }

    pub fn get_pc(&self) -> usize {
        return self.pc as usize;
    }

    pub fn set_pc(&mut self, addr: usize) {
        self.pc = addr as u16;
    }

    pub fn increment_pc(&mut self, value: usize) {
        self.pc += value as u16;
    }

    pub fn add_pc_signed(&mut self, value: i8) {
        self.pc = (self.pc as i32 + value as i32) as u16;
    }

    pub fn get_sp(&mut self) -> u8 {
        return self.sp;
    }

    pub fn set_n(&mut self, value: bool) {
        self.ps.set_n(value);
    }

    pub fn set_i(&mut self, value: bool) {
        self.ps.set_i(value);
    }

    pub fn set_z(&mut self, value: bool) {
        self.ps.set_z(value);
    }

    pub fn get_z(&self) -> bool {
        return self.ps.get_z();
    }
}

#[allow(dead_code)]
struct ProcessorStatusRegister {
    n: bool,
    v: bool,
    b: bool,
    d: bool,
    i: bool,
    z: bool,
    c: bool,
}

impl ProcessorStatusRegister {
    fn new() -> ProcessorStatusRegister {
        return ProcessorStatusRegister {
            n: false,
            v: false,
            b: false,
            d: false,
            i: false,
            z: false,
            c: false,
        };
    }

    fn set_i(&mut self, value: bool) {
        self.i = value;
    }

    fn set_n(&mut self, value: bool) {
        self.z = value;
    }

    fn set_z(&mut self, value: bool) {
        self.z = value;
    }

    fn get_z(&self) -> bool {
        return self.z;
    }
}
