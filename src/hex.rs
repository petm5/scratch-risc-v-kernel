pub mod hex {

    pub fn from_byte(byte: u8) -> [u8; 2] {
        [single((byte >> 4) & 0xf), single(byte & 0xf)]
    }

    fn single(half: u8) -> u8 {
        if half < 10 {
            half + 48
        } else {
            half - 10 + 97
        }
    }

}
