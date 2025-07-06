#![feature(cfg_select)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(static_mut_refs)]

pub mod context;
pub mod libs;

use core::arch::asm;
use core::fmt::Write;

use limine::BaseRevision;
use limine::framebuffer::Framebuffer;
use limine::request::{
    BootloaderInfoRequest, DateAtBootRequest, FramebufferRequest, MpRequest, RequestsEndMarker, RequestsStartMarker, StackSizeRequest
};
use limine::response::BootloaderInfoResponse;

use crate::context::KernelContext;
use crate::libs::generic::logging::logger::Logger;
use crate::libs::{arch, drivers};

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

#[used]
#[unsafe(link_section = ".requests")]
static DATE_AT_BOOT_REQUEST: DateAtBootRequest = DateAtBootRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static MP_REQUEST: MpRequest = MpRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

static mut KERNEL_CONTEXT: KernelContext<'static> = KernelContext {
    framebuffer: None,
    vga: None,
    logger: None,
};

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

fn get_limine_bootloader_info(bootloader_info_response: &mut Option<&BootloaderInfoResponse>) {
    match BOOTLOADERINFO_REQUEST.get_response() {
        Some(response) => {
            info!(
                "Bootloader info: {}, {} REV {}",
                response.name(),
                response.version(),
                response.revision()
            );
            *bootloader_info_response = Some(response)
        }
        None => {
            panic!("Bootloader info request failed");
        }
    }
}

fn get_boot_time() {
    match DATE_AT_BOOT_REQUEST.get_response() {
        Some(response) => {
            info!("Booted at {:#?}", response.timestamp());
        }
        None => {
            panic!("DateAtBoot request failed.")
        }
    }
}

fn get_mp() {
    if let Some(mp_response) = MP_REQUEST.get_response() {
        info!("Physical processors count: {:#?}", mp_response.cpus().len());
    } else {
        panic!("MP request failed");
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());
    let mut fb_request: Option<Framebuffer<'_>> = None;

    get_limine_framebuffer(&mut fb_request);

    if fb_request.is_none() {
        hcf();
    }

    unsafe {
        // Fuck you for coding this, straight up.
        KERNEL_CONTEXT.framebuffer = Some(fb_request.unwrap());
        KERNEL_CONTEXT.vga = Some(drivers::logs::sinks::vga::Vga::new(
            KERNEL_CONTEXT.framebuffer.as_ref().unwrap(),
        ));
        KERNEL_CONTEXT.logger = Some(Logger::new(KERNEL_CONTEXT.vga.as_mut().unwrap()));
    };

    info!("Kernel started successully !");
    info!(
        "Limine Base Revision: {}",
        BASE_REVISION.loaded_revision().unwrap_or(0)
    );
    get_mp();
    if let Some(fb) = &KERNEL_CONTEXT.framebuffer {
        info!(
            "Framebuffer: {}x{} @ {}bpp",
            fb.width(),
            fb.height(),
            fb.bpp()
        );
    }

    let mut bootloader_info_response: Option<&BootloaderInfoResponse> = None;

    get_limine_bootloader_info(&mut bootloader_info_response);
    get_boot_time();

    arch::init();
    info!("Many maaan");
    hcf();
}
