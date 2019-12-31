pub(crate) fn to_2_digits(n: i32) -> i32 {
    let n = n as f32;
    let digit_offset = std::cmp::max((n.abs().log10() - 2.0).ceil() as i32, 0);
    let rounded = n / 10f32.powi(digit_offset);
    rounded as i32
}

pub(crate) fn format_coordinate(coordinate_raw: f64) -> String {
    let coordinate = coordinate_raw;
    let degree = coordinate.trunc() as i64;
    let minute = ((coordinate - degree as f64).abs() * 60.0).trunc() as i64;
    let seconds = 3600.0 * (coordinate - degree as f64).abs() - 60.0 * (minute as f64);
    let decimal_part_string = format!("{:.2}", seconds - (seconds.trunc() as f64) as f64);
    let decimal_part_split = decimal_part_string.split('.').collect::<Vec<&str>>();
    let decimal_part = decimal_part_split.last().unwrap();

    format!(
        "{:03}.{:02}.{:02}.{:02}",
        degree,
        minute,
        to_2_digits(seconds as i32),
        decimal_part
    )
}
