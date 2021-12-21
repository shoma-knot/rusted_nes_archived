
pub fn is_up_7bit(value: u8) -> bool {
    return (value & 0b1000000) >> 7 == 1;
}

pub fn is_zero(value: u8) -> bool {
    return value == 0;
}
