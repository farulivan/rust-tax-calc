pub fn format_rupiah(amount: i64) -> String {
    debug_assert!(amount >= 0, "amount should never be negative here");

    if amount == 0 {
        return "Rp 0".to_string();
    }

    let s = amount.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);

    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push('.');
        }
        result.push(c);
    }

    format!("Rp {}", result.chars().rev().collect::<String>())
}