pub fn combine_big(a:u8, b:u8) -> u16 {
    combine_little(b, a)
}

pub fn combine_little(a:u8, b:u8) -> u16 {
    let a1 = a as u16;
    let b1 = b as u16;

    match b1.checked_shl(8) {
        Some(s) =>  s | a1,
        None => panic!("Invalid shift results")
    }
}

pub fn split_u16(a:u16) -> (u8, u8) {
    let high = (a >> 8) as u8;
    let low = a as u8;
    (high, low)
}
