use std::io::{self, Write};

use super::read_line;

const MAX_SAFE_RUPIAH_I64: i64 = 1_000_000_000_000_000;

pub fn read_number(prompt: &str) -> i64 {
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
            }
            Err(_) => {
                println!("❌ Angka terlalu besar untuk diproses oleh sistem.\n");
            }
        }
    }
}