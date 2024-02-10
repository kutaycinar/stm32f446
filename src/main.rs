#![no_main]
#![no_std]

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    init_gpio();

    let led = 0x14;
    
    // blink led
    loop {
        set_register(0x40020000, led, 0b100000);
        cortex_m::asm::delay(5000000);

        set_register(0x40020000, led, 0);
        cortex_m::asm::delay(5000000);
    }
}

// ----------------------------------------------------------------------------

use core::ptr::{read_volatile, write_volatile};

// Function to set a register value
fn set_register(address: u32, offset: u32, value: u32) {
    let register = (address + offset) as *mut u32;
    unsafe {
        write_volatile(register, value);
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

const RCC_BASE: u32 = 0x4002_3800;
const GPIOA_BASE: u32 = 0x4002_0000;
const RCC_AHB1ENR_OFFSET: u32 = 0x30;
const GPIOA_MODER_OFFSET: u32 = 0x00;

fn init_gpio() {
    // Enable GPIOA clock
    let rcc_ahb1enr = (RCC_BASE + RCC_AHB1ENR_OFFSET) as *mut u32;
    unsafe {
        let mut tmp = read_volatile(rcc_ahb1enr);
        tmp |= 1 << 0; // Enable GPIOA clock
        write_volatile(rcc_ahb1enr, tmp);
    }

    // Configure PA5 as an output
    let gpioa_moder = (GPIOA_BASE + GPIOA_MODER_OFFSET) as *mut u32;
    unsafe {
        let mut tmp = read_volatile(gpioa_moder);
        tmp &= !(0b11 << (5 * 2)); // Clear mode bits for PA5
        tmp |= 0b01 << (5 * 2); // Set mode to General Purpose Output (01)
        write_volatile(gpioa_moder, tmp);
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
