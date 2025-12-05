use std::io::{self, Write};

// ===============================================================
// CONSTANTS
// ===============================================================

// Upper limit for any rupiah amount we accept.
// This avoids unrealistic values and keeps us safely within i64 range.
const MAX_SAFE_RUPIAH_I64: i64 = 1_000_000_000_000_000; // 1e15

// ===============================================================
// FUNCTIONS
// ===============================================================

/// Read a single line of user input from stdin and return it as an owned `String`.
///
/// How it works:
/// 1. Create an empty String.
/// 2. Fill it using `read_line`.
/// 3. Trim whitespace and newline.
/// 4. Convert slice → owned String (so we can safely return it).
fn read_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");

    input.trim().to_string()
}

/// Read a whole-number rupiah amount as i64.
///
/// Accepts:
/// - "10000"
/// - "2500000"
///
/// Rejects:
/// - ".", ","
/// - decimals
/// - letters
/// - empty input
/// - numbers > MAX_SAFE_RUPIAH_I64
fn read_number(prompt: &str) -> i64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let raw = read_line();

        if raw.contains('.') || raw.contains(',') {
            println!("❌ Tidak menerima tanda pemisah atau desimal.");
            println!("   Contoh benar: 10000\n");
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
    /// Return a human-readable label for this PTKP status.
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

    /// Map PTKP → TER Category (Rule: A for TK0/TK1/K0, etc.)
    fn get_ter_category(&self) -> TerCategory {
        match self {
            PtkpStatus::TK0 | PtkpStatus::TK1 | PtkpStatus::K0 => TerCategory::A,
            PtkpStatus::TK2 | PtkpStatus::TK3 | PtkpStatus::K1 | PtkpStatus::K2 => TerCategory::B,
            PtkpStatus::K3 => TerCategory::C,
        }
    }
}

/// Show PTKP menu and return a PtkpStatus.
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
// SIMPLE TER RATE + SIMPLE TAX
// ===============================================================

/// Temporary simplified TER rates.
/// Real government tables will be added later.
fn get_simple_ter_rate(category: TerCategory) -> f64 {
    match category {
        TerCategory::A => 5.0,
        TerCategory::B => 10.0,
        TerCategory::C => 15.0,
    }
}

/// Simple Gross-method PPh21:
///       bruto × (ter_rate / 100)
fn calculate_pph21_gross(bruto: i64, ter_rate_percent: f64) -> f64 {
    bruto as f64 * (ter_rate_percent / 100.0)
}

// ===============================================================
// MAIN
// ===============================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                 KALKULATOR PPh 21 BULANAN                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("\n📌 Step 5: Hitung PPh21 sederhana (Gross Method).\n");

    let penghasilan_bruto = read_number("💵 Masukkan Penghasilan Bruto Bulanan: ");
    let ptkp_status = select_ptkp();

    // Determine special category (A/B/C)
    let ter_category = ptkp_status.get_ter_category();

    // Get simple TER rate
    let ter_rate = get_simple_ter_rate(ter_category);

    // Calculate simple PPh21
    let pph21 = calculate_pph21_gross(penghasilan_bruto, ter_rate);

    println!("\n===== RINGKASAN INPUT =====");
    println!("Penghasilan Bruto : {}", format_rupiah(penghasilan_bruto));
    println!("Status PTKP       : {}", ptkp_status.display_name());
    println!("Kategori TER      : {:?}", ter_category);
    println!("Tarif TER         : {}%", ter_rate);
    println!("PPh21 (Sederhana) : Rp {:.0}", pph21);
}
