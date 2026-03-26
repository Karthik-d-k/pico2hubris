//! # UART0 Echo Example
//!
//! Configures UART0 at 115200 8N1, sends a greeting, then echoes bytes.

#![no_std]
#![no_main]

use userlib as _;

use rp235x_pac::io_bank0::gpio::gpio_ctrl::FUNCSEL_A;
use rp235x_pac::{IO_BANK0, PADS_BANK0, RESETS, UART0};

/// Write a single byte to UART0, blocking until TX FIFO has space
fn uart_write_byte(uart: &UART0, byte: u8) {
    while uart.uartfr().read().txff().bit_is_set() {}
    uart.uartdr().write(|w| unsafe { w.data().bits(byte) });
}

/// Write a byte slice to UART0
fn uart_write_bytes(uart: &UART0, data: &[u8]) {
    for &b in data {
        uart_write_byte(uart, b);
    }
}

/// Configure a GPIO pin's pad and funcsel
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

#[unsafe(export_name = "main")]
fn main() -> ! {
    let resets = unsafe { RESETS::steal() };
    let io_bank0 = unsafe { IO_BANK0::steal() };
    let pads_bank0 = unsafe { PADS_BANK0::steal() };
    let uart0 = unsafe { UART0::steal() };

    // Assert then deassert reset on all peripherals at once
    resets.reset().modify(|_, w| {
        w.pads_bank0().set_bit();
        w.io_bank0().set_bit();
        w.uart0().set_bit()
    });
    resets.reset().modify(|_, w| {
        w.pads_bank0().clear_bit();
        w.io_bank0().clear_bit();
        w.uart0().clear_bit()
    });
    while !resets.reset_done().read().pads_bank0().bit() {}
    while !resets.reset_done().read().io_bank0().bit() {}
    while !resets.reset_done().read().uart0().bit() {}

    // Configure GPIO0 (TX) and GPIO1 (RX) for UART function
    gpio_configure(&io_bank0, &pads_bank0, 0, FUNCSEL_A::UART);
    gpio_configure(&io_bank0, &pads_bank0, 1, FUNCSEL_A::UART);

    // Configure UART0: 115200 baud, 8N1 (PL011 init sequence)
    uart0.uartcr().write(|w| w); // Disable UART
    while uart0.uartfr().read().busy().bit_is_set() {} // Wait for TX idle
    uart0.uartlcr_h().write(|w| w); // Flush FIFO

    // Baud rate: 115200 @ 12 MHz => IBRD=6, FBRD=33
    uart0
        .uartibrd()
        .write(|w| unsafe { w.baud_divint().bits(6) });
    uart0
        .uartfbrd()
        .write(|w| unsafe { w.baud_divfrac().bits(33) });

    // 8 data bits, no parity, 1 stop bit, FIFO enabled (latches baud divisors)
    uart0.uartlcr_h().write(|w| unsafe { w.wlen().bits(0b11) });

    // Enable UART, TX, and RX
    uart0.uartcr().write(|w| {
        w.uarten().set_bit();
        w.txe().set_bit();
        w.rxe().set_bit()
    });

    // Send initial message
    uart_write_bytes(&uart0, b"PAC-only UART echo on UART0\r\n");
    uart_write_bytes(&uart0, b"Type something and it will be echoed back:\r\n");

    loop {
        // Check if RX FIFO has data (RXFE = 0 means not empty)
        if !uart0.uartfr().read().rxfe().bit_is_set() {
            let data = uart0.uartdr().read().data().bits();
            // Echo the byte back
            uart_write_byte(&uart0, data);
        }
    }
}
