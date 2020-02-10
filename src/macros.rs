macro_rules! format_coordinate_part {
    ($coordinate:expr) => {{
        use crate::utils::to_2_digits;

        let degree = $coordinate.trunc() as i64;
        let minute = (($coordinate - degree as f64).abs() * 60.0).trunc() as i64;
        let seconds = 3600.0 * ($coordinate - degree as f64).abs() - 60.0 * (minute as f64);
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
    }};
}

macro_rules! format_coordinate {
    ($lat:expr, $lon:expr) => {{
        let lat_str = format_coordinate_part!($lat);
        let lon_str = format_coordinate_part!($lon);
        format!("COORD:N{}:E{}\n", lat_str, lon_str).as_bytes()
    }};
}
