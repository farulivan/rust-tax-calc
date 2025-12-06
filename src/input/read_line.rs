use std::io;

pub fn read_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");

    input.trim().to_string()
}