use super::get_ter_rate::get_ter_rate;
use super::ptkp::TerCategory;

pub fn calculate_gross_up_with_dynamic_ter(bruto: i64, category: TerCategory) -> (f64, f64, f64) {
    let mut rate = get_ter_rate(category, bruto as f64);
    const MAX_ITERATIONS: u8 = 10;

    for _ in 0..MAX_ITERATIONS {
        let total = (bruto as f64 * (100.0 / (100.0 - rate))).floor();
        let new_rate = get_ter_rate(category, total);

        if (new_rate - rate).abs() < 0.0001 {
            let pph21 = ((total * rate) / 100.0).floor();
            let tunjangan = total - bruto as f64;
            return (tunjangan, pph21, rate);
        }
        rate = new_rate;
    }

    let total = (bruto as f64 * (100.0 / (100.0 - rate))).floor();
    let pph21 = ((total * rate) / 100.0).floor();
    let tunjangan = total - bruto as f64;
    (tunjangan, pph21, rate)
}