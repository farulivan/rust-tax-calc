use std::io::{self, Write};

use super::read_line;
use crate::tax::CalculationMethod;

pub fn select_calculation_method() -> CalculationMethod {
    println!("\n📋 Pilih Metode Perhitungan:");
    println!("   1. Gross    - Pajak ditanggung karyawan (dipotong dari gaji)");
    println!("   2. Gross Up - Pajak ditunjang perusahaan (dapat tunjangan pajak)");

    loop {
        print!("\nPilihan Anda (1-2): ");
        io::stdout().flush().unwrap();

        match read_line().as_str() {
            "1" => return CalculationMethod::Gross,
            "2" => return CalculationMethod::GrossUp,
            _ => println!("❌ Pilihan tidak valid."),
        }
    }
}