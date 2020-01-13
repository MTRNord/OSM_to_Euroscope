pub(crate) fn to_2_digits(n: i32) -> i32 {
    let n = n as f32;
    let digit_offset = std::cmp::max((n.abs().log10() - 2.0).ceil() as i32, 0);
    let rounded = n / 10f32.powi(digit_offset);
    rounded as i32
}
