#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::panic::PanicInfo;
use core::ptr;

use talc::*;

mod pen;
use crate::pen::canvas;

mod hex;
// use hex::hex;

// use micromath::F32Ext;

#[macro_use]
extern crate alloc;

static mut ARENA: [u8; 1024] = [0; 1024];

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ErrOnOom> = Talc::new(ErrOnOom).lock();

fn uart_write(message: &[u8]) {
   const UART: *mut u8 = 0x10000000 as *mut u8;

   for b in message {
       unsafe {
       	      ptr::write_volatile(UART, *b);
       }
   }
}

fn uart_print(message: &str) {
    uart_write(message.as_bytes())
}

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
    main().unwrap();
    loop {}
}

fn main() -> Option<()> {

    uart_print("Hello from Rust!\n");

    unsafe { ALLOCATOR.lock().claim(ARENA.as_mut().into()).unwrap(); }

    canvas::clear();
    canvas::set_size(6);

    let mut cb: f32 = 0.0;

    loop {
        let mut c = cb.clone();
        for x in 0..128 {
            canvas::move_to(x*2, 0);
            canvas::begin_line((c % 256.0) as u8);
            canvas::move_to(x*2, 255);
            canvas::end_line();
            c += 1.0;
        }
        cb += 3.0;
        canvas::sync();
    }

    // Some(())
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uart_print("Panic!\n");
    if let Some(location) = info.location() {
        uart_print(location.file());
        uart_print("\nLine 0x");
        uart_write(&hex::hex::from_byte(location.line() as u8));
        uart_print(", Col 0x");
        uart_write(&hex::hex::from_byte(location.column() as u8));
        uart_print("\n");
        if let Some(p) = info.payload().downcast_ref::<&str>() {
            uart_print(p);
            uart_print("\n");
        } else {
            uart_print("No info available\n");
        }
    }
    uart_print(format!("{}\n", info).as_str());
    loop {}
}
