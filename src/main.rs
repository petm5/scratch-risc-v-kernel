#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::panic::PanicInfo;

// mod hex;
mod uart;
mod display;

const IMAGE: [u8; 64 * 48 * 4] = *include_bytes!("image.raw");

#[naked]
#[no_mangle]
#[link_section = ".text.init"]
unsafe extern "C" fn _start() -> ! {
    use core::arch::naked_asm;
    naked_asm!(
        ".option push",
        ".option norelax",
        "la gp, _global_pointer",
        ".option pop",

        "la sp, _init_stack_top",

        "tail {entry}",
        entry = sym entry,
    );
}

extern "C" fn entry() -> ! {
    main();
    loop {}
}

fn main() {
    display::init_ramfb();
    display::write_buffer(&IMAGE);
    display::set_pixel((0, 0), [255, 0, 0]);
    display::set_pixel((1, 0), [0, 255, 0]);
    display::set_pixel((2, 0), [0, 0, 255]);
    display::wait_for_frame();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
