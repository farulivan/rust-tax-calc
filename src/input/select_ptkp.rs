use std::io::{self, Write};

use super::read_line;
use crate::tax::PtkpStatus;

pub fn select_ptkp() -> PtkpStatus {
    println!("\n📋 Pilih Status PTKP:");
    println!("   1. TK/0 - Tidak Kawin, Tanpa Tanggungan");
    println!("   2. TK/1 - Tidak Kawin, 1 Tanggungan");
    println!("   3. TK/2 - Tidak Kawin, 2 Tanggungan");
    println!("   4. TK/3 - Tidak Kawin, 3 Tanggungan");
    println!("   5. K/0  - Kawin, Tanpa Tanggungan");
    println!("   6. K/1  - Kawin, 1 Tanggungan");
    println!("   7. K/2  - Kawin, 2 Tanggungan");
    println!("   8. K/3  - Kawin, 3 Tanggungan");

    loop {
        print!("\nPilihan Anda (1-8): ");
        io::stdout().flush().unwrap();

        match read_line().as_str() {
            "1" => return PtkpStatus::TK0,
            "2" => return PtkpStatus::TK1,
            "3" => return PtkpStatus::TK2,
            "4" => return PtkpStatus::TK3,
            "5" => return PtkpStatus::K0,
            "6" => return PtkpStatus::K1,
            "7" => return PtkpStatus::K2,
            "8" => return PtkpStatus::K3,
            _ => println!("❌ Pilihan tidak valid. Silakan pilih 1-8."),
        }
    }
}