#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

mod ui;

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    if let Some(fb_res) = FRAMEBUFFER_REQUEST.response() {
        if let Some(fb) = fb_res.framebuffers().first() {
            let fb_ptr = fb.address() as *mut u32;
            let width = fb.width as usize;
            let height = fb.height as usize;
            let pitch = fb.pitch as usize / 4;
            for i in 0..(height * pitch) {
                unsafe { fb_ptr.add(i).write_volatile(0x00_1A_1C_23); }
            }

            unsafe {
                ui::CopperUI::draw_rect(fb_ptr, pitch, 20, 20, 200, 40, 0x00_B8_73_33);

                ui::CopperUI::draw_rect(fb_ptr, pitch, 22, 22, 196, 36, 0x00_11_13_17);

                let mut char_x = 30;
                let char_y = 37;

                for _ in 0..14 {
                    ui::CopperUI::draw_block_char(fb_ptr, pitch, char_x, char_y, 0x00_FF_D7_00);
                    char_x += 12;
                }
            }
        }
    }

    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
