#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit as _;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    // loop infinito; apenas para não sairmos deste stack frame
    loop {}
}
