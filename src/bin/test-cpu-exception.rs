#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;
use a_new_os::{exit_qemu, serial_println};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    a_new_os::interrupts::init_idt();

    // invoke a breakpoint exception
    x86_64::instructions::int3();

    serial_println!("failed");

    unsafe { exit_qemu(); }
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("ok");

    unsafe { exit_qemu(); }
    loop {}
}
