#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

pub mod libs;

use core::arch::asm;

use limine::BaseRevision;
use limine::framebuffer::Framebuffer;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};

use crate::libs::drivers;

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

/// Define the stand and end markers for Limine requests.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());
    let framebuffer: Option<Framebuffer>;

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(fb) = framebuffer_response.framebuffers().next() {
            framebuffer = Some(fb);
        } else {
            panic!("No framebuffer found");
        }
    } else {
        panic!("Framebuffer request failed");
    }
    let fb = framebuffer.unwrap();
    let mut vga = drivers::vga::Vga::new(&fb);

    vga.init();
    hcf();
}

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
