#![no_std] // dont link rust std lib
#![no_main] // disable rust entry point
#![feature(custom_test_frameworks)]
#![test_runner(atlas_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use atlas_os::{println, hlt_loop};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    atlas_os::init();

    #[cfg(test)]
    test_main();
    println!("End main");
    hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    atlas_os::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
