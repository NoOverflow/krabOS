#![feature(cfg_select)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(static_mut_refs)]
#![allow(unused_unsafe)]
#![allow(unconditional_panic)]

pub mod context;
pub mod libs;

use core::arch::asm;

use crate::context::{BootInfo, KernelContext};
use crate::libs::generic::logging::logger::Logger;
use crate::libs::generic::memory;
use crate::libs::{arch, drivers};
use limine::BaseRevision;
use limine::framebuffer::Framebuffer;
use limine::request::{
    BootloaderInfoRequest, DateAtBootRequest, ExecutableAddressRequest, FramebufferRequest,
    HhdmRequest, MemoryMapRequest, MpRequest, RequestsEndMarker, RequestsStartMarker,
    StackSizeRequest,
};

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
#[unsafe(link_section = ".requests")]
static KA_REQUEST: ExecutableAddressRequest = ExecutableAddressRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static KMMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

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
    kpanic!(
        "Message: {}\nLocation: {}",
        _info.message(),
        _info.location().unwrap_or(&core::panic::Location::caller())
    );
    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

fn populate_boot_info(boot_info: &mut BootInfo) {
    boot_info.limine_base_revision = BASE_REVISION.loaded_revision();
    boot_info.kernel_phys_address = KA_REQUEST
        .get_response()
        .expect("Incomplete bootloader request for response address")
        .physical_base();
    boot_info.kernel_virt_address = KA_REQUEST
        .get_response()
        .expect("Incomplete bootloader request for response address")
        .virtual_base();
    boot_info.hhdm = HHDM_REQUEST
        .get_response()
        .expect("Incomplete bootloader response for HHDM")
        .offset();
    boot_info.rtc_boot = DATE_AT_BOOT_REQUEST.get_response().map(|r| r.timestamp());
}

fn print_boot_info(boot_info: &BootInfo) {
    match BOOTLOADERINFO_REQUEST.get_response() {
        Some(response) => {
            info!(
                "Bootloader info: {}, {} REV {}",
                response.name(),
                response.version(),
                response.revision()
            );
        }
        None => {
            panic!("Bootloader info request failed");
        }
    }
    info!(
        "Kernel loaded at physical address {:#x} (virtual {:#x} - HHDM {:#x})",
        boot_info.kernel_phys_address, boot_info.kernel_virt_address, boot_info.hhdm
    );
    match boot_info.rtc_boot {
        Some(rtc) => {
            info!("Booted at {:#?}", rtc);
        }
        None => warning!("No RTC found, set date and time manually !"),
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

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());
    let mut fb_request: Option<Framebuffer<'_>> = None;
    let mut boot_info: BootInfo = BootInfo::default();

    get_limine_framebuffer(&mut fb_request);

    unsafe {
        // Fuck you for coding this, straight up.
        KERNEL_CONTEXT.framebuffer = Some(fb_request.unwrap());
        KERNEL_CONTEXT.vga = Some(drivers::logs::sinks::vga::Vga::new(
            KERNEL_CONTEXT.framebuffer.as_ref().unwrap(),
        ));
        KERNEL_CONTEXT.logger = Some(Logger::new(KERNEL_CONTEXT.vga.as_mut().unwrap()));
    };

    info!("Kernel started successully !");
    populate_boot_info(&mut boot_info);
    print_boot_info(&boot_info);
    arch::init();
    info!("Many maaan");
    hcf();
}
