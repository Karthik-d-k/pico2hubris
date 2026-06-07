use rp235x_pac::io_bank0::gpio::gpio_ctrl::FUNCSEL_A;
use rp235x_pac::{IO_BANK0, PADS_BANK0, PWM, RESETS};

pub const PWM_PIN: usize = 25; // PWM 4B -> OnBoard LED
pub const PWM_CH: usize = 4; // PWM channel 4

pub fn reset_bringup(resets: &RESETS) {
    resets.reset().modify(|_, w| {
        w.io_bank0().clear_bit();
        w.pads_bank0().clear_bit();
        w.pwm().clear_bit()
    });

    while !resets.reset_done().read().io_bank0().bit() {}
    while !resets.reset_done().read().pads_bank0().bit() {}
    while !resets.reset_done().read().pwm().bit() {}
}

pub fn setup_pwm(io_bank0: &IO_BANK0, pads_bank0: &PADS_BANK0, pwm: &PWM) {
    pads_bank0.gpio(PWM_PIN).modify(|_, w| {
        w.iso().clear_bit();
        w.od().clear_bit();
        w.ie().set_bit()
    });

    unsafe {
        io_bank0
            .gpio(PWM_PIN)
            .gpio_ctrl()
            .write_with_zero(|w| w.funcsel().variant(FUNCSEL_A::PWM));
    }

    // set counter wrap value to max - 1
    pwm.ch(PWM_CH).top().write(|w| unsafe { w.bits(0xFFFE) });

    // enable pwm channel
    pwm.ch(PWM_CH).csr().write(|w| w.en().set_bit());
}
