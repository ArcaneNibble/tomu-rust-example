#![feature(alloc)]
#![feature(global_allocator)]
#![feature(lang_items)]
#![feature(proc_macro)]
#![no_std]

extern crate alloc_cortex_m;
#[macro_use]
extern crate alloc;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_rtfm as rtfm;
extern crate efm32hg309;

use alloc_cortex_m::CortexMHeap;
use rtfm::{app};

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

extern "C" {
    static mut _sheap: u32;
    static mut _eheap: u32;
}

app! {
    device: efm32hg309,
}

fn init(p: init::Peripherals) {
    // Initialize the allocator
    let start = unsafe { &mut _sheap as *mut u32 as usize };
    let end = unsafe { &mut _eheap as *mut u32 as usize };
    unsafe { ALLOCATOR.init(start, end - start) }
}

fn idle() -> ! {
    loop { }
}
