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

// ===============================================================
// SELECT PTKP
// ===============================================================

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

// ===============================================================
// TER CATEGORY TABLES — FULL 44(A)/44(B)/42(C)
// ===============================================================

const TER_CATEGORY_A: [(f64, f64); 44] = [
    (5_400_000.0, 0.0), (5_650_000.0, 0.25), (5_950_000.0, 0.5), (6_300_000.0, 0.75),
    (6_750_000.0, 1.0), (7_500_000.0, 1.25), (8_550_000.0, 1.5), (9_650_000.0, 1.75),
    (10_050_000.0, 2.0), (10_350_000.0, 2.25), (10_700_000.0, 2.5), (11_050_000.0, 3.0),
    (11_600_000.0, 3.5), (12_500_000.0, 4.0), (13_750_000.0, 5.0), (15_100_000.0, 6.0),
    (16_950_000.0, 7.0), (19_750_000.0, 8.0), (24_150_000.0, 9.0), (26_450_000.0, 10.0),
    (28_000_000.0, 11.0), (30_050_000.0, 12.0), (32_400_000.0, 13.0), (35_400_000.0, 14.0),
    (39_100_000.0, 15.0), (43_850_000.0, 16.0), (47_800_000.0, 17.0), (51_400_000.0, 18.0),
    (56_300_000.0, 19.0), (62_200_000.0, 20.0), (68_600_000.0, 21.0), (77_500_000.0, 22.0),
    (89_000_000.0, 23.0), (103_000_000.0, 24.0), (125_000_000.0, 25.0), (157_000_000.0, 26.0),
    (206_000_000.0, 27.0), (337_000_000.0, 28.0), (454_000_000.0, 29.0), (550_000_000.0, 30.0),
    (695_000_000.0, 31.0), (910_000_000.0, 32.0), (1_400_000_000.0, 33.0), (f64::MAX, 34.0),
];

const TER_CATEGORY_B: [(f64, f64); 44] = [
    (6_200_000.0, 0.0), (6_500_000.0, 0.25), (6_850_000.0, 0.5), (7_300_000.0, 0.75),
    (7_700_000.0, 1.0), (8_300_000.0, 1.25), (9_200_000.0, 1.5), (10_750_000.0, 1.75),
    (11_250_000.0, 2.0), (11_600_000.0, 2.25), (12_050_000.0, 2.5), (12_500_000.0, 3.0),
    (13_050_000.0, 3.5), (14_000_000.0, 4.0), (15_550_000.0, 5.0), (17_050_000.0, 6.0),
    (19_500_000.0, 7.0), (22_700_000.0, 8.0), (26_600_000.0, 9.0), (28_100_000.0, 10.0),
    (30_100_000.0, 11.0), (32_600_000.0, 12.0), (35_400_000.0, 13.0), (38_900_000.0, 14.0),
    (43_000_000.0, 15.0), (47_400_000.0, 16.0), (51_200_000.0, 17.0), (55_800_000.0, 18.0),
    (61_400_000.0, 19.0), (68_000_000.0, 20.0), (74_500_000.0, 21.0), (83_200_000.0, 22.0),
    (95_000_000.0, 23.0), (110_000_000.0, 24.0), (134_000_000.0, 25.0), (169_000_000.0, 26.0),
    (221_000_000.0, 27.0), (390_000_000.0, 28.0), (463_000_000.0, 29.0), (561_000_000.0, 30.0),
    (709_000_000.0, 31.0), (965_000_000.0, 32.0), (1_405_000_000.0, 33.0), (f64::MAX, 34.0),
];

const TER_CATEGORY_C: [(f64, f64); 42] = [
    (6_600_000.0, 0.0), (6_950_000.0, 0.25), (7_350_000.0, 0.5), (7_800_000.0, 0.75),
    (8_250_000.0, 1.0), (8_850_000.0, 1.25), (9_800_000.0, 1.5), (10_950_000.0, 1.75),
    (11_200_000.0, 2.0), (12_050_000.0, 2.25), (12_950_000.0, 2.5), (14_150_000.0, 3.0),
    (15_550_000.0, 3.5), (17_050_000.0, 4.0), (19_000_000.0, 5.0), (21_100_000.0, 6.0),
    (24_000_000.0, 7.0), (26_400_000.0, 8.0), (28_850_000.0, 9.0), (31_450_000.0, 10.0),
    (34_050_000.0, 11.0), (36_700_000.0, 12.0), (39_650_000.0, 13.0), (43_100_000.0, 14.0),
    (47_100_000.0, 15.0), (51_300_000.0, 16.0), (55_700_000.0, 17.0), (60_600_000.0, 18.0),
    (66_700_000.0, 19.0), (74_500_000.0, 20.0), (83_200_000.0, 21.0), (93_000_000.0, 22.0),
    (109_000_000.0, 23.0), (129_000_000.0, 24.0), (163_000_000.0, 25.0), (211_000_000.0, 26.0),
    (374_000_000.0, 27.0), (459_000_000.0, 28.0), (555_000_000.0, 29.0), (704_000_000.0, 30.0),
    (957_000_000.0, 31.0), (f64::MAX, 32.0),
];

// ===============================================================
// TER LOOKUP FUNCTION
// ===============================================================

fn get_ter_rate(category: TerCategory, bruto: f64) -> f64 {
    let table = match category {
        TerCategory::A => TER_CATEGORY_A.as_slice(),
        TerCategory::B => TER_CATEGORY_B.as_slice(),
        TerCategory::C => TER_CATEGORY_C.as_slice(),
    };

    for &(max_income, rate) in table {
        if bruto <= max_income {
            return rate;
        }
    }

    // Return the last rate in the table (already handles f64::MAX)
    // This line should never execute due to f64::MAX sentinel
    table.last().map(|&(_, rate)| rate).unwrap_or(0.0)
}

// ===============================================================
// CALCULATION METHOD
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
// TAX FORMULAS
// ===============================================================

fn calculate_gross(bruto: i64, rate: f64) -> f64 {
    (bruto as f64 * (rate / 100.0)).floor()
}

/// Official PP58 Gross-Up — dynamic TER re-evaluation loop.
/// 
/// Why this needs a loop:
/// - Gross-Up increases total penghasilan (bruto + tunjangan pajak)
/// - Higher total penghasilan may move to a higher TER bracket
/// - New TER changes the tax → which changes total penghasilan again
/// - So TER and total must be recalculated until TER becomes stable
///
/// Loop steps:
/// 1. Hitung total penghasilan memakai TER saat ini
/// 2. Cek TER baru berdasarkan total penghasilan tersebut
/// 3. Jika TER sama → selesai (stabil)
/// 4. Jika TER berubah → ulangi perhitungan dengan TER baru
///
/// Convergence is very fast (1–3 loops), but we cap max iterations for safety.
fn calculate_gross_up_with_dynamic_ter(bruto: i64, category: TerCategory) -> (f64, f64, f64) {
    let mut rate = get_ter_rate(category, bruto as f64);
    const MAX_ITERATIONS: u8 = 10; // TER tables have finite brackets

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

    // Fallback: use last computed values
    let total = (bruto as f64 * (100.0 / (100.0 - rate))).floor();
    let pph21 = ((total * rate) / 100.0).floor();
    let tunjangan = total - bruto as f64;
    (tunjangan, pph21, rate)
}

// ===============================================================
// MAIN
// ===============================================================
fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                 KALKULATOR PPh 21 BULANAN                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    let bruto = read_number("💵 Masukkan Penghasilan Bruto Bulanan (contoh: 10000000): ");
    let ptkp = select_ptkp();
    let method = select_calculation_method();

    let category = ptkp.get_ter_category();
    let initial_rate = get_ter_rate(category, bruto as f64);

    println!("\n===== RINGKASAN INPUT =====");
    println!("Penghasilan Bruto : {}", format_rupiah(bruto));
    println!("Status PTKP       : {}", ptkp.display_name());
    println!("Kategori TER      : {:?}", category);
    println!("Tarif TER Awal    : {}%", initial_rate);
    println!("Metode            : {:?}\n", method);

    match method {
        CalculationMethod::Gross => {
            let pph21 = calculate_gross(bruto, initial_rate); // already floored
            let pph21_int = pph21 as i64; // safe to convert to i64
            println!("PPh21 (Gross)     : {}", format_rupiah(pph21_int));
        }

        CalculationMethod::GrossUp => {
            let (tunjangan, pph21, final_rate) =
                calculate_gross_up_with_dynamic_ter(bruto, category);
            
            let tunjangan_int = tunjangan as i64;
            let pph21_int = pph21 as i64;

            println!("Tarif TER Akhir   : {}%", final_rate);
            println!("Tunjangan PPh21   : {}", format_rupiah(tunjangan_int));
            println!("Total Penghasilan : {}", format_rupiah(bruto + tunjangan_int));
            println!("PPh21 (Gross-Up)  : {}", format_rupiah(pph21_int));
        }
    }
}
