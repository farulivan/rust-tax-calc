use super::ptkp::TerCategory;
use super::ter_tables::{TER_CATEGORY_A, TER_CATEGORY_B, TER_CATEGORY_C};

pub fn get_ter_rate(category: TerCategory, bruto: f64) -> f64 {
    let table: &[(f64, f64)] = match category {
        TerCategory::A => &TER_CATEGORY_A,
        TerCategory::B => &TER_CATEGORY_B,
        TerCategory::C => &TER_CATEGORY_C,
    };

    for &(max_income, rate) in table {
        if bruto <= max_income {
            return rate;
        }
    }

    table.last().map(|&(_, rate)| rate).unwrap_or(0.0)
}