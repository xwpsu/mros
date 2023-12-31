#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

extern crate multiboot2;
use multiboot2::{Tag, TagType, FramebufferTag};

mod drivers;
use crate::drivers::console::{multiboot_info, find_fb, STDOUT};

mod mm;
use mm::page_table_entry::PhysAddr;
use mm::phys_page::kernel_heap_init;

#[no_mangle]
pub extern "C" fn rust_main(info: *mut multiboot_info, free_mem_base: *mut u8)
{
    // Initialize FrameBuffer
    let buffer_base: *mut u32 = find_fb(info) as *mut u32;
    STDOUT.set_base(buffer_base);

    // Set kernel heap start
    kernel_heap_init(PhysAddr::from(free_mem_base));

    loop{}
}

#[lang = "eh_personality"] 
#[no_mangle] 
pub extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}