#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

#[path = "../drivers/net/realtek/rtl8153.rs"]
mod rtl8153;
#[path = "../drivers/sound/speaker.rs"]
mod speaker;
mod pci;

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub struct TitaniumKernel {
    pub version: &'static str,
}

impl TitaniumKernel {
    pub fn new() -> Self {
        Self { version: "0.0.5" }
    }

    pub fn boot(&mut self) {
        unsafe {
            speaker::PcSpeaker::play_tone(1000);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut kernel = TitaniumKernel::new();
    kernel.boot();

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
