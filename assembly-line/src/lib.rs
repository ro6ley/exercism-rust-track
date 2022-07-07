pub fn production_rate_per_hour(speed: u8) -> f64 {
    // convert to u32 to avoid u8 overflow 
    let total_number: u32 = 221u32 * u32::from(speed); // as u32;
    match speed {
        1..=4 => {
            f64::from(total_number)
        },
        5..=8 => {
            f64::from(total_number) * 0.9_f64
        },
        _ => {
            f64::from(total_number) * 0.77_f64
        }
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / f64::from(60)) as u32
}
