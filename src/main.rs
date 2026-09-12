#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[path = "../drivers/net/realtek/rtl8153.rs"]
mod rtl8153;

bootloader::entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static bootloader::BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u16;
    let message = b"TitaniumKernel v0.0.1 is alive!";

    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            let code = (0x0F_u16 << 8) | (byte as u16);
            vga_buffer.add(i).write_volatile(code);
        }
    }

    let mut net_card = rtl8153::Realtek8153::new();
    net_card.init();

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
