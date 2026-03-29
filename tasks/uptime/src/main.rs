//! # Uptime Task
//!
//! Every 5 seconds, reads the kernel timer to get elapsed ms since boot
//! and sends it over UART0 as human-readable text.

#![no_std]
#![no_main]

use hubris_notifications::TIMER;
use rp235x_pac::io_bank0::gpio::gpio_ctrl::FUNCSEL_A;
use rp235x_pac::{IO_BANK0, PADS_BANK0, RESETS, UART0};
use userlib::{sys_get_timer, sys_recv_notification, sys_set_timer};

use userlib as _;

/// Sleep until the given absolute deadline (in kernel ticks / ms).
fn sleep_until(deadline: u64) {
    sys_set_timer(Some(deadline), TIMER);
    sys_recv_notification(TIMER);
}

/// Write a single byte to UART0, blocking until TX FIFO has space.
fn uart_write_byte(uart: &UART0, byte: u8) {
    while uart.uartfr().read().txff().bit_is_set() {}
    uart.uartdr().write(|w| unsafe { w.data().bits(byte) });
}

/// Write a byte slice to UART0.
fn uart_write_bytes(uart: &UART0, data: &[u8]) {
    for &b in data {
        uart_write_byte(uart, b);
    }
}

/// Configure a GPIO pin's pad and funcsel.
fn gpio_configure(io_bank0: &IO_BANK0, pads_bank0: &PADS_BANK0, pin: usize, funcsel: FUNCSEL_A) {
    pads_bank0.gpio(pin).modify(|_, w| {
        w.ie().set_bit();
        w.od().clear_bit()
    });

    unsafe {
        io_bank0
            .gpio(pin)
            .gpio_ctrl()
            .write_with_zero(|w| w.funcsel().variant(funcsel));
    };

    pads_bank0.gpio(pin).modify(|_, w| w.iso().clear_bit());
}

/// Convert u64 to decimal ASCII into a stack buffer, return the slice.
fn u64_to_dec(mut val: u64, buf: &mut [u8; 20]) -> &[u8] {
    if val == 0 {
        buf[19] = b'0';
        return &buf[19..];
    }
    let mut pos = 20;
    while val > 0 {
        pos -= 1;
        buf[pos] = b'0' + (val % 10) as u8;
        val /= 10;
    }
    &buf[pos..]
}

#[unsafe(export_name = "main")]
fn main() -> ! {
    let resets = unsafe { RESETS::steal() };
    let io_bank0 = unsafe { IO_BANK0::steal() };
    let pads_bank0 = unsafe { PADS_BANK0::steal() };
    let uart0 = unsafe { UART0::steal() };

    // Reset only UART0; io_bank0/pads_bank0 is configured by heartbeat task
    resets.reset().modify(|_, w| w.uart0().set_bit());
    resets.reset().modify(|_, w| w.uart0().clear_bit());
    while !resets.reset_done().read().uart0().bit() {}

    // Wait for io_bank0 and pads_bank0 (already out of reset by heartbeat task)
    while !resets.reset_done().read().pads_bank0().bit() {}
    while !resets.reset_done().read().io_bank0().bit() {}

    // Configure GPIO0 (TX) and GPIO1 (RX) for UART
    gpio_configure(&io_bank0, &pads_bank0, 0, FUNCSEL_A::UART);
    gpio_configure(&io_bank0, &pads_bank0, 1, FUNCSEL_A::UART);

    // Configure UART0: 115200 baud, 8N1
    uart0.uartcr().write(|w| w);
    while uart0.uartfr().read().busy().bit_is_set() {}
    uart0.uartlcr_h().write(|w| w);

    // Baud rate: 115200 @ 12 MHz => IBRD=6, FBRD=33
    uart0
        .uartibrd()
        .write(|w| unsafe { w.baud_divint().bits(6) });
    uart0
        .uartfbrd()
        .write(|w| unsafe { w.baud_divfrac().bits(33) });

    // 8 data bits, no parity, 1 stop bit, FIFO enabled
    uart0.uartlcr_h().write(|w| unsafe { w.wlen().bits(0b11) });

    // Enable UART, TX, and RX
    uart0.uartcr().write(|w| {
        w.uarten().set_bit();
        w.txe().set_bit();
        w.rxe().set_bit()
    });

    // --- Uptime loop using absolute deadlines ---
    let mut deadline = sys_get_timer().now;

    loop {
        deadline += 2000; // 2s at 1ms ticks
        sleep_until(deadline);

        let now = sys_get_timer().now;
        let mut buf = [0u8; 20];
        let digits = u64_to_dec(now / 1000, &mut buf); // convert ms to seconds

        uart_write_bytes(&uart0, b"uptime: ");
        uart_write_bytes(&uart0, digits);
        uart_write_bytes(&uart0, b"s\r\n");
    }
}
