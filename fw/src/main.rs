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
use rtfm::{app, Threshold};

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

extern "C" {
    static mut _sheap: u32;
    static mut _eheap: u32;
}

app! {
    device: efm32hg309,

    resources: {
        static GPIO: efm32hg309::GPIO;
    },

    idle: {
        resources: [GPIO],
    },
}

fn init(p: init::Peripherals) -> init::LateResources {
    // Initialize the allocator
    let start = unsafe { &mut _sheap as *mut u32 as usize };
    let end = unsafe { &mut _eheap as *mut u32 as usize };
    unsafe { ALLOCATOR.init(start, end - start) }

    // Clean up from bootloader
    unsafe {
        p.core.NVIC.icer[0].write(0xFFFFFFFF);
        p.core.NVIC.icpr[0].write(0xFFFFFFFF);
    }

    p.device.CMU.hfperclken0.modify(|_, w| w.gpio().bit(true));

    p.device.GPIO.pa_model.modify(|_, w| w.mode0().wiredand());
    p.device.GPIO.pb_model.modify(|_, w| w.mode7().wiredand());

    p.device.GPIO.pa_doutclr.write(|w| unsafe {w.bits(0b00000001) });
    p.device.GPIO.pb_doutclr.write(|w| unsafe {w.bits(0b10000000) });

    init::LateResources {
        GPIO: p.device.GPIO,
    }
}

fn idle(_t: &mut Threshold, r: idle::Resources) -> ! {
    loop {
        r.GPIO.pa_doutset.write(|w| unsafe {w.bits(0b00000001) });
        r.GPIO.pb_doutset.write(|w| unsafe {w.bits(0b10000000) });

        r.GPIO.pa_doutclr.write(|w| unsafe {w.bits(0b00000001) });
        r.GPIO.pb_doutclr.write(|w| unsafe {w.bits(0b10000000) });
    }
}
