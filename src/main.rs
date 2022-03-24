#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]

mod vga_buffer;
mod serial;
mod interrupts;
mod gdt;
use x86_64;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {

    //println!("Hello World{}", "!");
    //panic!("Some panic message");
    serial_println!("Hello World{}", "!");

    interrupts::init_idt();
    gdt::init_gdt();
    interrupts::init_pic8259();

    //enable interrupts
    x86_64::instructions::interrupts::enable();
    
    //x86_64::instructions::interrupts::int3();

    // // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // // trigger a stack overflow
    // stack_overflow();    

    serial_println!("It did not crash!");
    interrupts::hlt_loop();
}

