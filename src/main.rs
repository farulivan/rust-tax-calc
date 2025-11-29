// 'use' is like import, to bring functionality from other modules into scope
// 'std::io' is a module in the standard library for input/output
// 'self' here brings the io *module itself* into scope as `io`
//   → without `self`, we would have to write `std::io::stdin()` instead of `io::stdin()`
// 'Write' is a trait that provides the 'flush' method; in Rust we must explicitly import
//   traits to be able to call their methods in method-call syntax (like `.flush()`).
use std::io::{self, Write};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                 KALKULATOR PPh 21 BULANAN                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("\n📌 Untuk sekarang, kita hanya akan input penghasilan bruto.\n");

    // 'print!' is similar to 'println!' but it does NOT print a newline at the end.
    // This means the cursor stays on the same line after the text.
    print!("💵 Masukkan Penghasilan Bruto Bulanan (contoh: 10_000_000): ");

    // We flush the stdout buffer so the prompt text is definitely shown
    // before the program waits for user input.
    // 'flush' returns a Result; 'unwrap' means:
    //   - if flush succeeds → continue
    //   - if it fails       → panic (crash the program)
    // NOTE: in production code, it's usually better to handle the error
    //       instead of calling 'unwrap'.
    io::stdout().flush().unwrap(); 

    // 'mut' means this variable is mutable; we need to be able to change
    // the String contents when we read input into it.
    // 'String::new' creates a new, empty String buffer on the heap.
    let mut input = String::new();

    // 'read_line' reads a line of text from standard input and APPENDS it
    // to the given String buffer.
    // It takes '&mut input' because it only BORROWS the String mutably;
    // it does not take ownership of it.
    // 'expect' is like 'unwrap' but with a custom panic message if an error occurs.
    // Again, in real applications you'd normally handle the error instead of panicking.
    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input"); 

    // 'trim' returns a &str slice that points into the existing String;
    // it does NOT allocate a new String, it just gives a view without
    // leading/trailing whitespace (including the newline from read_line).
    // Here we "shadow" the previous `input` variable: now `input` is a &str, not a String.
    let input = input.trim(); 

    println!("\nPenghasilan Bruto Bulanan Anda: \"{}\"", input);
}
