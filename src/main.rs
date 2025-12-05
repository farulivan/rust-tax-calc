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
/// This function is extracted from our previous `main` function to avoid repeating
/// the same boilerplate code each time we want to ask the user for input.
///
/// How it works:
/// 1. Create an empty `String` buffer.
/// 2. Call `read_line` to fill that buffer with user input.  
///    - It needs `&mut String` because it borrows our buffer and writes into it.
/// 3. `trim()` removes whitespace and the trailing newline `\n` from the input.
///    - `trim()` returns `&str` (a slice), NOT a new String.
/// 4. `.to_string()` converts the slice into a new owned `String`,  
///    so this function can safely return it.
///
/// We return a String (not &str) because returning a `&str` would require the data
/// to live beyond this function, but our local `String` would be dropped at the end.
/// Therefore, we must return an owned `String` to transfer ownership to the caller.
fn read_line() -> String {
    // Create an empty String buffer that will hold the user input.
    let mut input = String::new();

    // Read user input into the buffer.
    // `read_line` appends the typed text into `input`.
    // We use `expect` to panic if reading fails (fine for small CLI tools).
    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");

    // `trim()` removes surrounding whitespace and the trailing newline.
    // It returns a &str slice, so we call `to_string()` to return a new owned String.
    input.trim().to_string()
}

/// Read a whole-number rupiah amount as i64.
///
/// Accepts only:
///     "10000"
///     "2500000"
///
/// Rejects:
///     - thousand separators: "." or ","
///     - decimals
///     - negative values
///     - scientific notation
///     - letters/symbols
///     - empty input
///     - if the number is > MAX_SAFE_RUPIAH_I64
fn read_number(prompt: &str) -> i64 {
    loop {
        // Print the prompt (no newline).
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        // Read user input (already trimmed).
        let raw = read_line();

        // Reject '.', ',' to avoid thousand/decimal confusion.
        if raw.contains('.') || raw.contains(',') {
            println!("❌ Tidak menerima tanda pemisah atau desimal. Hanya boleh angka tanpa format.");
            println!("   Contoh benar: 10000  (bukan 10.000 atau 10,000)\n");
            continue;
        }

        // Reject empty input.
        if raw.is_empty() {
            println!("❌ Input tidak boleh kosong.\n");
            continue;
        }

        // Ensure the input contains ONLY digits.
        if !raw.chars().all(|c| c.is_ascii_digit()) {
            println!("❌ Hanya boleh angka 0-9 tanpa spasi atau simbol.\n");
            continue;
        }

        // Convert to i64 safely (catch overflow, then domain limit).
        match raw.parse::<i64>() {
            // Parsed OK and within our domain limit → accept.
            Ok(value) if value <= MAX_SAFE_RUPIAH_I64 => {
                return value;
            }
            // Parsed OK but too big for our business rules.
            Ok(_) => {
                println!(
                    "❌ Angka terlalu besar. Maksimal yang diperbolehkan adalah {}.\n",
                    MAX_SAFE_RUPIAH_I64
                );
                continue;
            }
            // Overflow / parse error (should be rare because we validated digits, but still safe).
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
    TK0,
    TK1,
    TK2,
    TK3,
    K0,
    K1,
    K2,
    K3,
}

impl PtkpStatus {
    /// Returns a human-readable label for this PTKP status.
    ///
    /// Why `&self`?
    /// - This is an *instance method*, meaning it is called on a value such as:
    ///     `ptkp.display_name()`
    /// - We borrow `self` instead of taking ownership because:
    ///     - We don’t need to modify it.
    ///     - We don’t want to move it (enums are cheap to pass, but borrowing is idiomatic).
    ///
    /// Why return `&'static str`?
    /// - All returned values are **string literals**, e.g. `"TK/0 - ..."`.
    /// - String literals live for the *entire lifetime of the program*, so they naturally have
    ///   a `'static` lifetime.
    /// - Returning `&'static str` is correct, efficient, and avoids unnecessary allocations.
    ///
    /// Why `match self`?
    /// - We pattern-match the enum variant to select the correct label.
    /// - `self` is borrowed, so we match the reference directly
    ///   because Rust automatically dereferences in match patterns when possible.
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
}

/// Show PTKP menu and keep asking until the user selects 1–8.
/// Returns the chosen `PtkpStatus`.
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

        let input = read_line();

        match input.as_str() {
            "1" => return PtkpStatus::TK0,
            "2" => return PtkpStatus::TK1,
            "3" => return PtkpStatus::TK2,
            "4" => return PtkpStatus::TK3,
            "5" => return PtkpStatus::K0,
            "6" => return PtkpStatus::K1,
            "7" => return PtkpStatus::K2,
            "8" => return PtkpStatus::K3,
            _ => {
                println!("❌ Pilihan tidak valid. Silakan pilih 1-8.");
            }
        }
    }
}

// ===============================================================
// RUPIAH FORMATTER
// ===============================================================

/// Format an `i64` amount into a simple Rupiah string, e.g. 10000 → "Rp 10000".
///
/// At this step, we keep it simple (no thousand separators),
/// just to focus on the flow and types.
fn format_rupiah(amount: i64) -> String {
    // In debug builds, sanity-check we didn’t accidentally pass negative.
    // This is just a sanity check, not a runtime validation.
    // This only runs in debug builds (not release), so it doesn’t affect performance.
    debug_assert!(amount >= 0, "amount should never be negative here");

    format!("Rp {}", amount)
}

// =======================================================
// MAIN
// =======================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                 KALKULATOR PPh 21 BULANAN                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("\n📌 Untuk sekarang: input penghasilan bruto + pilih status PTKP.\n");

    // 1. Ask user for gross monthly income (whole rupiah, as i64).
    let penghasilan_bruto = read_number("💵 Masukkan Penghasilan Bruto Bulanan (contoh: 10000000): ");

    // 2. Ask user to choose PTKP status from the menu.
    let ptkp_status = select_ptkp();

    // 3. Show a simple summary of what the user entered.
    println!("\n===== RINGKASAN INPUT =====");
    println!("Penghasilan Bruto : {}", format_rupiah(penghasilan_bruto));
    println!("Status PTKP       : {}", ptkp_status.display_name());
}
