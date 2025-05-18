use core::mem;
use core::ffi::CStr;
use core::ptr::addr_of;

use core::sync::atomic::{compiler_fence, Ordering};

const DISPLAY_BASE: usize = 0x80040000;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 48;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT * 4;

const FW_CFG_DMA_CTL_READ:   u32 = 0x02;
const FW_CFG_DMA_CTL_SELECT: u32 = 0x08;
const FW_CFG_DMA_CTL_WRITE:  u32 = 0x10;

#[repr(C)]
struct FWCfgFile {
    size: u32,
    select: u16,
    reserved: u16,
    name: [u8; 56]
}

#[repr(C, packed)]
struct FWCfgDmaAccess {
    control: u32,
    len: u32,
    addr: u64
}

#[repr(C, packed)]
struct RamFBCfg {
    addr: u64,
    fmt: u32,
    flags: u32,
    w: u32,
    h: u32,
    st: u32
}

unsafe fn fw_cfg_dma_transfer(control: u32, len: u32, addr: u64) {
    const DMA: *mut u32 = (0x10100000 + 16) as *mut u32;

    let dma_access = FWCfgDmaAccess {
        control: control.to_be(),
        len: len.to_be(),
        addr: addr.to_be()
    };

    unsafe {
        let address = &dma_access as *const _ as u64;
        DMA.write_volatile(((address >> 32) as u32).to_be());
        compiler_fence(Ordering::AcqRel);
        DMA.add(1).write_volatile((address as u32).to_be());
    }

    while (u32::from_be(dma_access.control) != 0) {}
}

pub fn init_ramfb() {
    let mut num_entries: u32 = 0;
    let fw_cfg_file_directory = 0x19;
    unsafe {
        fw_cfg_dma_transfer((fw_cfg_file_directory << 16
            | FW_CFG_DMA_CTL_SELECT 
            | FW_CFG_DMA_CTL_READ) as u32,
            mem::size_of::<u32>() as u32,
            addr_of!(num_entries) as u64);
    }

    num_entries = num_entries.to_be();

    let ramfb = FWCfgFile {
        size: 0,
        select: 0,
        reserved: 0,
        name: [0; 56]
    };

    for _ in 0..num_entries {
        unsafe {
            fw_cfg_dma_transfer(FW_CFG_DMA_CTL_READ,
                mem::size_of::<FWCfgFile>() as u32,
                addr_of!(ramfb) as u64);
        }
        let entry = CStr::from_bytes_until_nul(&ramfb.name).unwrap();
        let entry = entry.to_str().unwrap();
        if entry == "etc/ramfb" {
            break;
        }
    }

    let pixel_format = ('A' as u32) | (('B' as u32) << 8) | 
    (('2' as u32) << 16) | (('4' as u32) << 24);

    let ramfb_cfg = RamFBCfg {
        addr: (DISPLAY_BASE as u64).to_be(),
        fmt: (pixel_format).to_be(),
        flags: (0 as u32).to_be(),
        w: (DISPLAY_WIDTH as u32).to_be(),
        h: (DISPLAY_HEIGHT as u32).to_be(),
        st: (4*DISPLAY_WIDTH as u32).to_be()
    };

    unsafe {
        fw_cfg_dma_transfer((ramfb.select.to_be() as u32) << 16 
            | FW_CFG_DMA_CTL_SELECT 
            | FW_CFG_DMA_CTL_WRITE, mem::size_of::<RamFBCfg>() as u32,
            addr_of!(ramfb_cfg) as u64);
    }
}

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

pub fn write_buffer(buffer: &[u8; DISPLAY_SIZE]) {
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
