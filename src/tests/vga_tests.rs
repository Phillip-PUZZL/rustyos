use crate::drivers::vga::print_vga::{SCREEN_BUFFER_HEIGHT, get_screen_char};
use crate::println;

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = get_screen_char(SCREEN_BUFFER_HEIGHT - 2, i);
        assert_eq!(char::from(screen_char), c);
    }
}