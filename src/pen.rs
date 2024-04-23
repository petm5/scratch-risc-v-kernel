pub mod canvas {

    use core::ptr;

    pub fn move_to(x: u8, y: u8) {
        const PEN_X: *mut u8 = 0x10002000 as *mut u8;
        const PEN_Y: *mut u8 = 0x10002001 as *mut u8;
        unsafe {
            ptr::write_volatile(PEN_X, x);
            ptr::write_volatile(PEN_Y, y);
        }
    }

    pub fn begin_line(color: u8) {
        const PEN_COL: *mut u8 = 0x10002002 as *mut u8;
        unsafe {
            ptr::write_volatile(PEN_COL, color);
        }
    }

    pub fn set_size(size: u8) {
        const PEN_SIZE: *mut u8 = 0x10002003 as *mut u8;
        unsafe {
            ptr::write_volatile(PEN_SIZE, size);
        }
    }

    pub fn end_line() {
        const PEN_SIZE: *mut u8 = 0x10002003 as *mut u8;
        unsafe {
            ptr::write_volatile(PEN_SIZE, 0);
        }
    }

    pub fn clear() {
        const PEN_CLEAR: *mut u8 = 0x10002004 as *mut u8;
        unsafe {
            ptr::write_volatile(PEN_CLEAR, 1);
        }
    }

    pub fn sync() {
        use core::arch::asm;
        unsafe {
            asm!(
                "scall"
            );
        }
    }

}
