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
            pc: 0,
            sp: 0,
            ps: ProcessorStatusRegister::new(),
        };
    }
}

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
}
