//! # Heartbeat Task
//!
//! Produces a heartbeat pattern on the on-board LED (GPIO25):
//! two quick blinks (250ms on, 250ms off), then a 1.5s pause.
//! Uses kernel timer syscalls for deadline-based scheduling.

#![no_std]
#![no_main]

use hubris_notifications::TIMER;
use rp235x_pac::io_bank0::gpio::gpio_ctrl::FUNCSEL_A;
use userlib::{sys_get_timer, sys_recv_notification, sys_set_timer};

use userlib as _;

const LED_PIN: usize = 25;

/// Sleep until the given absolute deadline (in kernel ticks / ms).
fn sleep_until(deadline: u64) {
    sys_set_timer(Some(deadline), TIMER);
    sys_recv_notification(TIMER);
}

#[unsafe(export_name = "main")]
fn main() -> ! {
    let mask = 1u32 << LED_PIN;

    // --- GPIO25 init ---
    let resets = unsafe { rp235x_pac::RESETS::steal() };
    let io_bank0 = unsafe { rp235x_pac::IO_BANK0::steal() };
    let pads_bank0 = unsafe { rp235x_pac::PADS_BANK0::steal() };
    let sio = unsafe { rp235x_pac::SIO::steal() };

    resets.reset().modify(|_, w| {
        w.io_bank0().clear_bit();
        w.pads_bank0().clear_bit()
    });
    while !resets.reset_done().read().io_bank0().bit() {}
    while !resets.reset_done().read().pads_bank0().bit() {}

    pads_bank0.gpio(LED_PIN).modify(|_, w| {
        w.iso().clear_bit();
        w.od().clear_bit();
        w.ie().set_bit()
    });

    unsafe {
        io_bank0
            .gpio(LED_PIN)
            .gpio_ctrl()
            .write_with_zero(|w| w.funcsel().variant(FUNCSEL_A::SIO));
    }

    sio.gpio_oe_set().write(|w| unsafe { w.bits(mask) });
    sio.gpio_out_clr().write(|w| unsafe { w.bits(mask) });

    // --- Heartbeat loop using absolute deadlines ---
    let mut deadline = sys_get_timer().now;

    loop {
        // Two quick blinks
        for _ in 0..2 {
            // LED ON for 250ms
            sio.gpio_out_set().write(|w| unsafe { w.bits(mask) });
            deadline += 250;
            sleep_until(deadline);

            // LED OFF for 250ms
            sio.gpio_out_clr().write(|w| unsafe { w.bits(mask) });
            deadline += 250;
            sleep_until(deadline);
        }

        // Long pause (1s) with LED off
        deadline += 1000;
        sleep_until(deadline);
    }
}
