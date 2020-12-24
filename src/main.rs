#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        let count = i as isize * 2;

        unsafe {
            *vga_buffer.offset(count) = byte;
            *vga_buffer.offset(count + 1) = 0xb;
        }
    }

    loop {}
}
