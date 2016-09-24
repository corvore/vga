extern crate vga;
extern crate core;

use core::fmt::Write;
use vga::Vga;

#[test]
fn create() {
    let mut mock_memory = vec![0u8; 25 * 80];

    unsafe { Vga::new(&mut mock_memory[0] as *mut u8) };
}

fn check_write<T: Write>(_: T) { }

#[test]
fn write() {
    let mut mock_memory = vec![0u8; 25 * 80];
    let vga = unsafe { Vga::new(&mut mock_memory[0] as *mut u8) };
    check_write(vga);
}

#[test]
fn flush() {
    let mut mock_memory = vec![0u8; 25 * 80];

    let mut vga = unsafe { Vga::new(&mut mock_memory[0] as *mut u8) };

    vga.write_str("hello").unwrap();

    vga.flush();

    assert_eq!(mock_memory[0], 'h' as u8);
    assert_eq!(mock_memory[1], 'e' as u8);
    assert_eq!(mock_memory[2], 'l' as u8);
    assert_eq!(mock_memory[3], 'l' as u8);
    assert_eq!(mock_memory[4], 'o' as u8);
}
