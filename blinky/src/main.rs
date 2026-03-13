//! # GPIO 'Blinky' Example
//!
//! This application blinks an on-board led present in Pico 2 board.

#![no_std]
#![no_main]

use rp235x_pac::io_bank0::gpio::gpio_ctrl::FUNCSEL_A;

/// Simple busy-wait delay (not cycle-accurate, but sufficient for blinking)
fn delay(count: u32) {
    for _ in 0..count {
        core::hint::black_box(());
    }
}

// Ensure we halt the program on panic (if we don't mention this crate it won't be linked)
use userlib as _;

#[unsafe(export_name = "main")]
fn main() -> ! {
    // `on-board LED` is on GPIO25 on the Pico 2
    const LED_PIN: usize = 25;
    let mask = 1u32 << LED_PIN;

    // Get peripheral access using the PAC
    // This is the same pattern used throughout Hubris
    let resets = unsafe { rp235x_pac::RESETS::steal() };
    let io_bank0 = unsafe { rp235x_pac::IO_BANK0::steal() };
    let pads_bank0 = unsafe { rp235x_pac::PADS_BANK0::steal() };
    let sio = unsafe { rp235x_pac::SIO::steal() };

    // Step 1: Take IO_BANK0 and PADS_BANK0 out of reset
    resets.reset().modify(|_, w| {
        w.io_bank0().clear_bit();
        w.pads_bank0().clear_bit()
    });

    // Wait for peripherals to come out of reset
    while !resets.reset_done().read().io_bank0().bit() {}
    while !resets.reset_done().read().pads_bank0().bit() {}

    // Step 2: Configure the pad for GPIO25
    // - Clear isolation (ISO) - RP2350 specific
    // - Enable output (clear OD)
    // - Enable input (set IE)
    pads_bank0.gpio(LED_PIN).modify(|_, w| {
        w.iso().clear_bit();
        w.od().clear_bit();
        w.ie().set_bit()
    });

    // Step 3: Set GPIO function to SIO (software control)
    unsafe {
        io_bank0
            .gpio(LED_PIN)
            .gpio_ctrl()
            .write_with_zero(|w| w.funcsel().variant(FUNCSEL_A::SIO));
    }

    // Step 4: Enable GPIO25 as output
    sio.gpio_oe_set().write(|w| unsafe { w.bits(mask) });

    // Step 5: Blink forever!
    loop {
        // Turn LED ON
        sio.gpio_out_set().write(|w| unsafe { w.bits(mask) });
        delay(50_000);

        // Turn LED OFF
        sio.gpio_out_clr().write(|w| unsafe { w.bits(mask) });
        delay(50_000);
    }
}
