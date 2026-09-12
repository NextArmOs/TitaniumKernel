pub struct PciScanner;

impl PciScanner {
    pub unsafe fn read_config(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let address = ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | ((offset as u32) & 0xfc)
            | 0x80000000;

        x86_64::instructions::port::PortWrite::write_to_port(0xCF8_u16, address);
        x86_64::instructions::port::PortRead::read_from_port(0xCFC_u16)
    }

    pub unsafe fn find_device_type(class_target: u8) -> bool {
        for bus in 0..255 {
            for slot in 0..32 {
                let vendor_id = (Self::read_config(bus, slot, 0, 0) & 0xFFFF) as u16;
                if vendor_id != 0xFFFF {
                    let class_info = Self::read_config(bus, slot, 0, 0x08);
                    let class_code = ((class_info >> 24) & 0xFF) as u8;
                    if class_code == class_target {
                        return true;
                    }
                }
            }
        }
        false
    }
}
