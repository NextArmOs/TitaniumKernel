#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

#[path = "../drivers/net/realtek/rtl8153.rs"]
mod rtl8153;
mod pci;

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub struct TitaniumKernel {
    pub version: &'static str,
    pub net_driver: rtl8153::Realtek8153,
}

impl TitaniumKernel {
    pub fn new() -> Self {
        Self {
            version: "0.0.4",
            net_driver: rtl8153::Realtek8153::new(),
        }
    }

    pub fn boot(&mut self) {
        self.net_driver.init();
        
        let has_gpu = unsafe { pci::PciScanner::find_device_type(0x03) };
        let has_net = unsafe { pci::PciScanner::find_device_type(0x02) };

        if has_gpu {
            self.draw_screen(has_net);
        }
    }

    fn draw_screen(&self, net_success: bool) {
        if let Some(fb_res) = FRAMEBUFFER_REQUEST.response() {
            if let Some(fb) = fb_res.framebuffers().first() {
                let fb_ptr = fb.address() as *mut u32;
                let width = fb.width as usize;
                let height = fb.height as usize;
                let pitch = fb.pitch as usize / 4;

                let bg_color = 0x00_1A_1C_23;
                let net_color = if net_success { 0x00_00_FF_00 } else { 0x00_FF_00_00 };

                for y in 0..height {
                    for x in 0..width {
                        unsafe {
                            if x < 40 && y < 40 {
                                fb_ptr.add(y * pitch + x).write_volatile(net_color);
                            } else {
                                fb_ptr.add(y * pitch + x).write_volatile(bg_color);
                            }
                        }
                    }
                }
            }
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
