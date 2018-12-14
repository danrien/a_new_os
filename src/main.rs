#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Linux startup:
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}!", "Bobby");
    // panic!("Some panic message");

    loop {}
}

mod vga_buffer;
