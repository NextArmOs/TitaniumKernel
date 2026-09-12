pub struct Realtek8153 {
    pub mac_address: [u8; 6],
    pub is_connected: bool,
}

impl Realtek8153 {
    pub fn new() -> Self {
        Self {
            mac_address: [0x00, 0xE0, 0x4C, 0x68, 0x01, 0x11],
            is_connected: false,
        }
    }

    pub fn init(&mut self) {
        self.is_connected = true;
    }

    pub fn get_mac(&self) -> [u8; 6] {
        self.mac_address
    }
}
