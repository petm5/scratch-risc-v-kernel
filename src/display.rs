const DISPLAY_BASE: usize = 0x80040000;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 48;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT * 4;

pub fn set_pixel(pos: (u8, u8), color: [u8; 3]) {
    let (x, y) = pos;
    let ptr = DISPLAY_BASE as *mut u8;
    let index: usize = (y as usize * DISPLAY_HEIGHT + x as usize) * 4;
    unsafe {
        let slice: &mut [u8] = core::slice::from_raw_parts_mut(ptr, DISPLAY_SIZE);
        slice[index] = color[0];
        slice[index+1] = color[1];
        slice[index+2] = color[2];
    }
}

pub fn write_buffer(buffer: [u8; DISPLAY_SIZE]) {
    let ptr = DISPLAY_BASE as *mut u8;
    unsafe {
        let slice: &mut [u8] = core::slice::from_raw_parts_mut(ptr, DISPLAY_SIZE);
        for i in 0..DISPLAY_SIZE {
            slice[i] = buffer[i];
        }
    }
}

pub fn wait_for_frame() {
    use core::arch::asm;
    unsafe {
        asm!(
            "scall"
        );
    }
}
