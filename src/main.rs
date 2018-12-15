#![no_std]
#![cfg_attr(not(test), no_main)]

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Linux startup:
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}!", "Bobby");
    // panic!("Some panic message");

    loop {}
}

mod vga_buffer;
