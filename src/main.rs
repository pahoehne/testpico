#![no_std]
#![no_main]

use rp_pico::entry;

use panic_halt as _;

use embedded_hal::digital::v2::OutputPin;

use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let sio = hal::Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();
    let mut gpio1 = pins.gpio1.into_push_pull_output();

    loop {
        gpio1.set_low().unwrap();
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        gpio1.set_high().unwrap();
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
