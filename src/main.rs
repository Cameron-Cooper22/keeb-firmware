#![no_std]
#![no_main]

use core::panic::PanicInfo;
use nrf52840_hal as hal;

pub extern "C" fn _start() -> ! {
}

fn panic(_info: &PanicInfo) -> ! {
    reset_to_bootloader();
}


fn reset_to_bootloader() -> ! {
    unsafe {
        (*hal::pac::POWER::PTR).gpregret.write(|w| w.bits(0xb1))
    };

    hal::pac::SCB::sys_reset();
}

#[embassy_executor::main]
async fn main() -> ! {
    let p = embassy_nrf::init(Default::default());
}
