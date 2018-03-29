#![feature(alloc)]
#![feature(global_allocator)]
#![feature(lang_items)]
#![feature(used)]
#![no_std]

extern crate alloc_cortex_m;
#[macro_use]
extern crate alloc;
extern crate cortex_m;
extern crate cortex_m_rt;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

extern "C" {
    static mut _sheap: u32;
    static mut _eheap: u32;
}

fn main() {
    // Initialize the allocator
    let start = unsafe { &mut _sheap as *mut u32 as usize };
    let end = unsafe { &mut _eheap as *mut u32 as usize };
    unsafe { ALLOCATOR.init(start, end - start) }

    // Test allocator
    let xs = vec![0, 1, 2];

    loop {}
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
