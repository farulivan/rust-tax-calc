pub fn calculate_gross(bruto: i64, rate: f64) -> f64 {
    (bruto as f64 * (rate / 100.0)).floor()
}