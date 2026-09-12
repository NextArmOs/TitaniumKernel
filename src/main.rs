#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

#[path = "../drivers/net/realtek/rtl8153.rs"]
mod rtl8153;

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut net_card = rtl8153::Realtek8153::new();
    net_card.init();
    let _mac = net_card.get_mac();

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().first() {
            let fb_ptr = framebuffer.address() as *mut u32;
            let width = framebuffer.width as usize;
            let height = framebuffer.height as usize;
            let pitch = framebuffer.pitch as usize / 4; 

            let titanium_bg: u32 = 0x00_1A_1C_23;
            let success_green: u32 = 0x00_00_FF_00;

            for y in 0..height {
                for x in 0..width {
                    unsafe {
                        if x < 50 && y < 50 {
                            fb_ptr.add(y * pitch + x).write_volatile(success_green);
                        } else {
                            fb_ptr.add(y * pitch + x).write_volatile(titanium_bg);
                        }
                    }
                }
            }
        }
    }

    x86_64::instructions::interrupts::disable();
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
