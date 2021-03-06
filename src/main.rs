#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry pointsstatus
#![feature(custom_test_frameworks)]
#![test_runner(djanka::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use djanka::{hlt_loop, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Czesc, jestem Djanka");

    djanka::init();

    hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    djanka::test_panic_handler(info)
}
