#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;
mod serial;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {

    //println!("Hello World{}", "!");
    //panic!("Some panic message");
    serial_println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //println!("{}", info);
    serial_println!("{}", info);
    loop {}
}