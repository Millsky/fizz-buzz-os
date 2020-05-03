#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

// Allocate beforehand for max performance
static FIZZ:&'static str = "FIZZ";
static BUZZ:&'static str = "BUZZ";
static FIZZ_BUZZ:&'static str = "FIZZBUZZ";

// This function is the entry point, since the linker looks for a function
// named `_start` by default we don't want to mangle this.
// It uses a diverging function to signal this will never terminate
#[no_mangle]
pub extern "C" fn _start() -> ! {
    for n in 1..100 {
        if n % 15 == 0 {
            println!("{}", FIZZ_BUZZ);
        } else if n % 3 == 0 {
            println!("{}", FIZZ);
        } else if n % 5 == 0 {
            println!("{}", BUZZ);
        }
    }

    loop {}
}

// This function is called on panic.
// It uses a diverging function to signal this will never terminate
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
