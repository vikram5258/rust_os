#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod vga_buffer;



use core::panic::PanicInfo;

#[no_mangle]
static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// #[cfg(test)]
// pub fn test_runner(tests: &[&dyn Fn()]) {
//     println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//     }
// }


// #[test_case]
// fn trivial_assertion() {
//     print!("trivial assertion... ");
//     assert_eq!(1, 1);
//     println!("[ok]");
// }
