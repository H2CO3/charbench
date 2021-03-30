use std::ops::BitOr;

#[inline(always)]
pub fn is_reserved_char_1(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

#[inline(always)]
fn bit_mask_for_chars(chars: &[u8]) -> (u64, u32, u32) {
    let min = chars.iter().copied().min().unwrap();
    let max = chars.iter().copied().max().unwrap();
    let mask = chars.iter().map(|&c| 1_u64 << (c - min)).fold(0, BitOr::bitor);
    (mask, min.into(), max.into())
}

#[inline(always)]
fn is_one_of(c: char, chars: &[u8]) -> bool {
    let c: u32 = c.into();
    let (mask, min, max) = bit_mask_for_chars(chars);
    let in_range = (min..=max).contains(&c);
    let matches = mask & 1 << (c - min);
    (matches != 0) & in_range
}

#[inline(always)]
pub fn is_reserved_char_6(c: char) -> bool {
    is_one_of(c, b"aeiou")
}
