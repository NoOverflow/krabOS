#![no_std]
#![no_main]

pub mod drivers;

use core::panic::PanicInfo;
use drivers::vgatty::VGATty;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut tty: VGATty = VGATty::bind();

    tty.set_background_color(drivers::vgatty::colors::Color::Cyan);
    tty.set_foreground_color(drivers::vgatty::colors::Color::LightGray);
    tty.clear_screen();

    tty.write(b"Monkey is happy.");
    loop {}
}

#[panic_handler]
fn __panic(_info: &PanicInfo) -> ! {
    loop {}
}
