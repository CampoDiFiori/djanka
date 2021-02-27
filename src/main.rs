#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry pointsstatus
#![feature(custom_test_frameworks)]
#![test_runner(djanka::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use djanka::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
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
