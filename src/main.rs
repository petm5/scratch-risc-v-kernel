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
    use core::arch::asm;
    asm!(
        ".option push",
        ".option norelax",
        "la gp, _global_pointer",
        ".option pop",

        "la sp, _init_stack_top",

        "tail {entry}",
        entry = sym entry,
        options(noreturn)
    );
}

extern "C" fn entry() -> ! {
    main();
    loop {}
}

fn main() {

    // let mut color: u8 = 127;

    // draw_line((0, 0), (255, 255));

    display::write_buffer(IMAGE);
    display::set_pixel((0, 0), [255, 0, 0]);
    display::set_pixel((1, 0), [0, 255, 0]);
    display::set_pixel((2, 0), [0, 0, 255]);
    display::wait_for_frame();
}

// fn draw_line() {
//     DISPLAY[]
// }

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
