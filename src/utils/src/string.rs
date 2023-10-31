pub fn multiple(item: &str, count: i64) -> String {
    format!("{item}{}", if count == 1 { "" } else { "s" })
}
