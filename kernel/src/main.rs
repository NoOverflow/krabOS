#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

pub mod libs;

use core::arch::asm;
use core::fmt::Write;

use limine::BaseRevision;
use limine::framebuffer::Framebuffer;
use limine::request::{
    BootloaderInfoRequest, FramebufferRequest, RequestsEndMarker, RequestsStartMarker,
    StackSizeRequest,
};
use limine::response::BootloaderInfoResponse;

use crate::libs::drivers;
use crate::libs::utils::logger::Logger;

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static BOOTLOADERINFO_REQUEST: BootloaderInfoRequest = BootloaderInfoRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(0xF00000);

/// Define the stand and end markers for Limine requests.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

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

fn get_limine_framebuffer(framebuffer: &mut Option<Framebuffer>) {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(fb) = framebuffer_response.framebuffers().next() {
            *framebuffer = Some(fb);
        } else {
            panic!("No framebuffer found");
        }
    } else {
        panic!("Framebuffer request failed");
    }
}

fn get_limine_bootloader_info(
    bootloader_info_response: &mut Option<&BootloaderInfoResponse>,
    logger: &mut Logger,
) {
    match BOOTLOADERINFO_REQUEST.get_response() {
        Some(response) => {
            writeln!(
                logger,
                "[krabos] Bootloader info: {}, {} REV {}",
                response.name(),
                response.version(),
                response.revision()
            )
            .unwrap();
            *bootloader_info_response = Some(response)
        }
        None => {
            panic!("Bootloader info request failed");
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());
    let mut fb_request: Option<Framebuffer<'_>> = None;

    get_limine_framebuffer(&mut fb_request);

    let framebuffer = fb_request.unwrap();
    let mut vga = drivers::logs::sinks::vga::Vga::new(&framebuffer);
    let mut logger = Logger::new(&mut vga);

    writeln!(logger, "[krabos] Kernel started successfully!").unwrap();
    writeln!(
        logger,
        "[krabos] Limine Base Revision: {}",
        BASE_REVISION.loaded_revision().unwrap_or(0)
    )
    .unwrap();
    writeln!(
        logger,
        "[krabos] Framebuffer: {}x{} @ {}bpp",
        framebuffer.width(),
        framebuffer.height(),
        framebuffer.bpp()
    )
    .unwrap();

    let mut bootloader_info_response: Option<&BootloaderInfoResponse> = None;

    get_limine_bootloader_info(&mut bootloader_info_response, &mut logger);
    for i in 0..100 {
        writeln!(logger, "[krabos] Test log line {}", i).unwrap();
    }
    hcf();
}
