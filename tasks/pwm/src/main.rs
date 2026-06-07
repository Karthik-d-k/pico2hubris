//! # PWM Task
//!
//! Produces a fading brightness pattern on the on-board LED (GPIO25):
//! Uses kernel timer syscalls for delay's.

#![no_std]
#![no_main]

mod pwm;

use hubris_notifications::TIMER;
use userlib::{sys_get_timer, sys_recv_notification, sys_set_timer};

use userlib as _;

use crate::pwm::{PWM_CH, reset_bringup, setup_pwm};

/// Sleep until the given absolute deadline (in kernel ticks / ms).
fn sleep_until(deadline: u64) {
    sys_set_timer(Some(deadline), TIMER);
    sys_recv_notification(TIMER);
}

#[unsafe(export_name = "main")]
fn main() -> ! {
    let p = unsafe { rp235x_pac::Peripherals::steal() };

    reset_bringup(&p.RESETS);
    setup_pwm(&p.IO_BANK0, &p.PADS_BANK0, &p.PWM);

    // --- Brightness fade loop using absolute deadlines ---
    let mut deadline = sys_get_timer().now;
    const PWM_LOW: u16 = 0x0000;
    const PWM_HIGH: u16 = 0xFFFF;

    loop {
        // Ramp brightness up ~2sec
        for v in (PWM_LOW..=PWM_HIGH).step_by(32) {
            p.PWM.ch(PWM_CH).cc().write(|w| unsafe { w.b().bits(v) });
            deadline += 1;
            sleep_until(deadline);
        }

        // Ramp brightness down ~2sec
        for v in (PWM_LOW..=PWM_HIGH).rev().step_by(32) {
            p.PWM.ch(PWM_CH).cc().write(|w| unsafe { w.b().bits(v) });
            deadline += 1;
            sleep_until(deadline);
        }
    }
}
