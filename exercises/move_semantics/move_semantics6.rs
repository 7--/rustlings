// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references


fn main() {
    let mut data = "Rust is great!".to_string();
    get_char(&mut data);
    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &mut String) -> char {
    *data += "add this";
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{}", data);
}
