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

/// Format an i64 rupiah amount into a display string,
/// e.g. 10000 -> "Rp 10000".
fn format_rupiah(amount: i64) -> String {
    // In debug builds, sanity-check we didn’t accidentally pass negative.
    // This is just a sanity check, not a runtime validation.
    // This only runs in debug builds (not release), so it doesn’t affect performance.
    debug_assert!(amount >= 0, "amount should never be negative here");

    format!("Rp {}", amount)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                 KALKULATOR PPh 21 BULANAN                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("\nSekarang kita akan mengubah input ke angka dan format Rupiah.\n");

    let penghasilan_bruto = read_number("💵 Masukkan Penghasilan Bruto Bulanan (contoh: 10000000): ");

    println!("\nAnda memasukkan: {}", penghasilan_bruto);
    println!("Dalam format Rupiah sederhana: {}", format_rupiah(penghasilan_bruto));
}
