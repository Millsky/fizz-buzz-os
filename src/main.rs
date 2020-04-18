#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello Github!";

// This function is the entry point, since the linker looks for a function
// named `_start` by default we don't want to mangle this.
// It uses a diverging function to signal this will never terminate
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
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
