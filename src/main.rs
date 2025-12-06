// Use File Hierarchy from `https://doc.rust-lang.org/rust-by-example/mod/split.html`

// Notes:
// self::        // current folder
// super::       // parent folder
// crate::       // project root


mod input;
mod tax;
mod formatting;

use input::{read_number, select_calculation_method, select_ptkp};
use tax::{calculate_gross, calculate_gross_up_with_dynamic_ter, get_ter_rate, CalculationMethod};
use formatting::{format_rupiah};

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
