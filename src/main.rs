#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
mod vga_buffer;
mod serial;
mod interrupts;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {

    //println!("Hello World{}", "!");
    //panic!("Some panic message");
    serial_println!("Hello World{}", "!");

    interrupts::init_idt();
    
    x86_64::instructions::interrupts::int3();

    serial_println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //println!("{}", info);
    serial_println!("{}", info);
    loop {}
}