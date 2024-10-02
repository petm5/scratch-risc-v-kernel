

    pub fn write(byte: u8) {
        const UART: *mut u8 = 0x10000000 as *mut u8;

        unsafe {
            UART.write_volatile(byte);
        }
    }

    pub fn read() -> Option<u8> {
        const UART: *mut u8 = 0x10000000 as *mut u8;

        unsafe {
            if UART.add(5).read_volatile() & 1 == 0 {
                None
            } else {
       	        Some(UART.read_volatile())
            }
        }
    }

    pub fn print(message: &str) {
        for b in message.as_bytes() {
            write(*b)
        }
    }

