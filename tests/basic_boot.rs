#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod regular;

use core::panic::PanicInfo;

//noinspection ALL,RsUnresolvedReference
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main(); // Don't worry about this

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustyos::test_panic_handler(info)
}