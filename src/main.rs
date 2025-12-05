use std::io::{self, Write};

// ===============================================================
// CONSTANTS
// ===============================================================

const MAX_SAFE_RUPIAH_I64: i64 = 1_000_000_000_000_000; // 1e15

// ===============================================================
// INPUT FUNCTIONS
// ===============================================================

fn read_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");

    input.trim().to_string()
}

fn read_number(prompt: &str) -> i64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let raw = read_line();

        if raw.contains('.') || raw.contains(',') {
            println!("❌ Tidak menerima tanda pemisah atau desimal. Hanya boleh angka tanpa format.");
            println!("   Contoh benar: 10000 (bukan 10.000 atau 10,000)\n");
            continue;
        }

        if raw.is_empty() {
            println!("❌ Input tidak boleh kosong.\n");
            continue;
        }

        if !raw.chars().all(|c| c.is_ascii_digit()) {
            println!("❌ Hanya boleh angka 0-9 tanpa spasi atau simbol.\n");
            continue;
        }

        match raw.parse::<i64>() {
            Ok(value) if value <= MAX_SAFE_RUPIAH_I64 => return value,
            Ok(_) => {
                println!(
                    "❌ Angka terlalu besar. Maksimal adalah {}.\n",
                    MAX_SAFE_RUPIAH_I64
                );
                continue;
            }
            Err(_) => {
                println!("❌ Angka terlalu besar untuk diproses oleh sistem.\n");
                continue;
            }
        }
    }
}

// ===============================================================
// PTKP STATUS ENUM
// ===============================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum PtkpStatus {
    TK0, TK1, TK2, TK3,
    K0, K1, K2, K3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TerCategory {
    A,
    B,
    C,
}

impl PtkpStatus {
    fn display_name(&self) -> &'static str {
        match self {
            PtkpStatus::TK0 => "TK/0 - Tidak Kawin, Tanpa Tanggungan",
            PtkpStatus::TK1 => "TK/1 - Tidak Kawin, 1 Tanggungan",
            PtkpStatus::TK2 => "TK/2 - Tidak Kawin, 2 Tanggungan",
            PtkpStatus::TK3 => "TK/3 - Tidak Kawin, 3 Tanggungan",
            PtkpStatus::K0  => "K/0  - Kawin, Tanpa Tanggungan",
            PtkpStatus::K1  => "K/1  - Kawin, 1 Tanggungan",
            PtkpStatus::K2  => "K/2  - Kawin, 2 Tanggungan",
            PtkpStatus::K3  => "K/3  - Kawin, 3 Tanggungan",
        }
    }

    fn get_ter_category(&self) -> TerCategory {
        match self {
            PtkpStatus::TK0 | PtkpStatus::TK1 | PtkpStatus::K0 => TerCategory::A,
            PtkpStatus::TK2 | PtkpStatus::TK3 | PtkpStatus::K1 | PtkpStatus::K2 => TerCategory::B,
            PtkpStatus::K3 => TerCategory::C,
        }
    }
}

fn select_ptkp() -> PtkpStatus {
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

// ===============================================================
// RUPIAH FORMATTER
// ===============================================================

fn format_rupiah(amount: i64) -> String {
    debug_assert!(amount >= 0, "amount should never be negative here");
    format!("Rp {}", amount)
}

// ===============================================================
// Calculation Method ENUM
// ===============================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum CalculationMethod {
    Gross,
    GrossUp,
}

fn select_calculation_method() -> CalculationMethod {
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

// ===============================================================
// SIMPLE TER + TAX FORMULAS
// ===============================================================

fn get_simple_ter_rate(category: TerCategory) -> f64 {
    match category {
        TerCategory::A => 5.0,
        TerCategory::B => 10.0,
        TerCategory::C => 15.0,
    }
}

fn calculate_pph21_gross(bruto: i64, rate: f64) -> f64 {
    bruto as f64 * (rate / 100.0)
}

// Gross Up Formula
fn calculate_pph21_gross_up(bruto: i64, rate: f64) -> (f64, f64) {
    let tunjangan = bruto as f64 * (rate / (100.0 - rate));
    let total = bruto as f64 + tunjangan;
    let pph21 = total * (rate / 100.0);
    (tunjangan, pph21)
}

// ===============================================================
// MAIN
// ===============================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                 KALKULATOR PPh 21 BULANAN                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("\n📌 Untuk sekarang: input penghasilan bruto + pilih status PTKP.\n");

    let penghasilan_bruto = read_number("💵 Masukkan Penghasilan Bruto Bulanan (contoh: 10000000): ");
    let ptkp_status = select_ptkp();

    // Ask calculation method
    let method = select_calculation_method();

    let ter_category = ptkp_status.get_ter_category();
    let ter_rate = get_simple_ter_rate(ter_category);

    println!("\n===== RINGKASAN INPUT =====");
    println!("Penghasilan Bruto : {}", format_rupiah(penghasilan_bruto));
    println!("Status PTKP       : {}", ptkp_status.display_name());
    println!("Kategori TER      : {:?}", ter_category);
    println!("Tarif TER         : {}%", ter_rate);
    println!("Metode            : {:?}\n", method);

    // Branching based on method
    match method {
        CalculationMethod::Gross => {
            let pph21 = calculate_pph21_gross(penghasilan_bruto, ter_rate);
            println!("PPh21 (Sederhana / Gross) : Rp {:.0}", pph21);
            println!("Penghasilan Bersih        : Rp {:.0}", (penghasilan_bruto as f64) - pph21);
        }

        CalculationMethod::GrossUp => {
            let (tunjangan, pph21) = calculate_pph21_gross_up(penghasilan_bruto, ter_rate);
            println!("Tunjangan PPh21           : Rp {:.0}", tunjangan);
            println!("Total Penghasilan         : Rp {:.0}", tunjangan + penghasilan_bruto as f64);
            println!("PPh21 (Sederhana / GrossUp): Rp {:.0}", pph21);
            println!("Penghasilan Bersih        : Rp {:.0}", penghasilan_bruto);
        }
    }
}
