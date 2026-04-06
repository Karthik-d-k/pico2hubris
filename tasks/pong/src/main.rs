//! # Pong Task (LED Server)
//!
//! Receives an IPC message containing a blink count (u32 LE),
//! blinks the on-board LED that many times, then replies.

#![no_std]
#![no_main]

use core::mem::MaybeUninit;
#[cfg(target_arch = "arm")]
use cortex_m::asm;
#[cfg(target_arch = "riscv32")]
use riscv::asm;
use rp235x_pac::io_bank0::gpio::gpio_ctrl::FUNCSEL_A;
use userlib::{ResponseCode, sys_recv_msg_open, sys_reply};

// Ensure we halt the program on panic
use userlib as _;

const LED_PIN: usize = 25;
const BLINK_DELAY: u32 = 6_000_000; // 0.5s at 12MHz

#[unsafe(export_name = "main")]
fn main() -> ! {
    let mask = 1u32 << LED_PIN;

    // --- GPIO25 init ---
    let resets = unsafe { rp235x_pac::RESETS::steal() };
    let io_bank0 = unsafe { rp235x_pac::IO_BANK0::steal() };
    let pads_bank0 = unsafe { rp235x_pac::PADS_BANK0::steal() };
    let sio = unsafe { rp235x_pac::SIO::steal() };

    // Take IO_BANK0 and PADS_BANK0 out of reset
    resets.reset().modify(|_, w| {
        w.io_bank0().clear_bit();
        w.pads_bank0().clear_bit()
    });
    while !resets.reset_done().read().io_bank0().bit() {}
    while !resets.reset_done().read().pads_bank0().bit() {}

    // Configure pad for GPIO25
    pads_bank0.gpio(LED_PIN).modify(|_, w| {
        w.iso().clear_bit();
        w.od().clear_bit();
        w.ie().set_bit()
    });

    // Set GPIO function to SIO
    unsafe {
        io_bank0
            .gpio(LED_PIN)
            .gpio_ctrl()
            .write_with_zero(|w| w.funcsel().variant(FUNCSEL_A::SIO));
    }

    // Enable GPIO25 as output
    sio.gpio_oe_set().write(|w| unsafe { w.bits(mask) });

    // Ensure LED starts off
    sio.gpio_out_clr().write(|w| unsafe { w.bits(mask) });

    // --- Server loop ---
    let mut buffer = [MaybeUninit::uninit(); 4];
    loop {
        let msg = sys_recv_msg_open(&mut buffer);

        // Parse blink count from LE bytes
        let count = if let Ok(data) = msg.data {
            if data.len() >= 4 {
                u32::from_le_bytes([data[0], data[1], data[2], data[3]])
            } else {
                0
            }
        } else {
            0
        };

        // Blink LED `count` times
        for _ in 0..count {
            sio.gpio_out_set().write(|w| unsafe { w.bits(mask) });
            asm::delay(BLINK_DELAY);
            sio.gpio_out_clr().write(|w| unsafe { w.bits(mask) });
            asm::delay(BLINK_DELAY);
        }

        sys_reply(msg.sender, ResponseCode::SUCCESS, &[]);
    }
}
