#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

fn clear_screen() {
    let vga_buffer = 0xb8000 as *mut u8;

    for i in 0..25 {
        for j in 0..80 {
            unsafe {
                *vga_buffer.offset((i * 80 + j) as isize * 2) = 0;
                *vga_buffer.offset((i * 80 + j) as isize * 2 + 1) = 0x0F;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    clear_screen();
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0F;
        }
    }

    loop {}
}

#[panic_handler]
fn __panic(_info: &PanicInfo) -> ! {
    loop {}
}
