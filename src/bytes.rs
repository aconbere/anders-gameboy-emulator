pub fn combine_little(a:u8, b:u8) -> u16 {
    let a1 = a as u16;
    let b1 = b as u16;

    match b1.checked_shl(8) {
        Some(s) =>  s | a1,
        None => panic!("Invalid shift results")
    }
}

pub fn combine(a:u8, b:u8) -> u16 {
    combine_little(b,a)
}

pub fn split_u16(a:u16) -> (u8, u8) {
    let high = (a >> 8) as u8;
    let low = a as u8;
    (high, low)
}

pub fn get_bit(input: u8, n: u8) -> u8 {
    input & (1 << n)
}

pub fn check_bit(input: u8, n: u8) -> bool {
    if n < 8 {
        get_bit(input, n) != 0
    } else {
        false
    }
}

pub fn set_bit(input: u8, n: u8) -> u8 {
    input | (1 << n)
}

pub fn clear_bit(input: u8, n: u8) -> u8 {
    input & !(1 << n)
}

pub fn add_unsigned_signed(unsigned:u16, signed:i8) -> u16 {
    if signed < 0 {
        unsigned - (-signed as u16)
    } else {
        unsigned + (signed as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_bit() {
        assert_eq!(check_bit(0x99, 7), true);
        assert_eq!(check_bit(0x7F, 7), false);
    }

    #[test]
    fn test_add_unsigned_signed() {
        assert_eq!(add_unsigned_signed(50 as u16, -13 as i8), 37);
    }
}
