//! # Ping Task (IPC Client)
//!
//! Sends an incrementing counter (1..=10) to the pong task via IPC.
//! Pong blinks the LED that many times. After each round, waits 3 seconds.

#![no_std]
#![no_main]

use cortex_m::asm;
use hubris_task_slots::SLOTS;
use userlib::sys_send;

// Ensure we halt the program on panic
use userlib as _;

const PAUSE_DELAY: u32 = 24_000_000; // 2s at 12MHz

#[unsafe(export_name = "main")]
fn main() -> ! {
    let mut pong = SLOTS.pong;
    let mut counter: u32 = 1;

    loop {
        let bytes = counter.to_le_bytes();

        match sys_send(pong, 1, &bytes, &mut [], &mut []) {
            Ok(_) => {}
            Err(dead) => {
                pong = pong.with_generation(dead.new_generation());
                continue;
            }
        }

        // Wait 3 seconds before sending next count
        asm::delay(PAUSE_DELAY);

        // Increment counter, wrap back to 1 after 10
        counter = if counter >= 10 { 1 } else { counter + 1 };
    }
}
