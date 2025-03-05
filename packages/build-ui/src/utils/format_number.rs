pub fn format_number(num: i64) -> String {
    if num >= 1_000_000_000_000 {
        format!("{:.1}T", num as f64 / 1_000_000_000_000.0)
    } else if num >= 1_000_000_000 {
        format!("{:.1}B", num as f64 / 1_000_000_000.0)
    } else if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        format!("{}", num)
    }
}
