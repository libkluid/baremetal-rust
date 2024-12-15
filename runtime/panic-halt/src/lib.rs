#![no_std]

use core::panic::PanicInfo;

#[inline(never)]
#[cfg_attr(not(test), panic_handler)]
pub fn panic_halt(_info: &PanicInfo) -> ! {
    loop {}
}
