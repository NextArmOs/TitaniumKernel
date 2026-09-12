pub struct PcSpeaker;

impl PcSpeaker {
    pub unsafe fn play_tone(frequency: u32) {
        if frequency == 0 {
            let mut port_val = x86_64::instructions::port::PortRead::read_from_port(0x61_u16);
            port_val &= 0xFC;
            x86_64::instructions::port::PortWrite::write_to_port(0x61_u16, port_val);
            return;
        }

        let divider = 1193180 / frequency;

        x86_64::instructions::port::PortWrite::write_to_port(0x43_u16, 0xB6_u8);
        x86_64::instructions::port::PortWrite::write_to_port(0x42_u16, (divider & 0xFF) as u8);
        x86_64::instructions::port::PortWrite::write_to_port(0x42_u16, ((divider >> 8) & 0xFF) as u8);

        let mut port_val = x86_64::instructions::port::PortRead::read_from_port(0x61_u16);
        if (port_val & 3) != 3 {
            x86_64::instructions::port::PortWrite::write_to_port(0x61_u16, port_val | 3);
        }
    }

    pub unsafe fn mute() {
        Self::play_tone(0);
    }
}

