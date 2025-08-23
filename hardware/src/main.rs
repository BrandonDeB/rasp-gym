#![no_std]
#![no_main]

use bsp::entry;
use defmt_rtt as _;
use panic_probe as _;
use bsp::hal::fugit::RateExtU32;
use rp_pico as bsp;
use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
    gpio::{FunctionI2C, Pin},
};
use defmt::info;
mod lcd_driver;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio26.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio27.reconfigure();

    let mut i2c = bsp::hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin, // Try `not_an_scl_pin` here
        400_u32.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );
    // Send to I2C device at address 0x27
    let mut lcd = lcd_driver::I2cLcd::new(i2c, 0x27); // 0x27 is common I2C LCD address

    lcd.write_str("Hello, world!").unwrap();
    info!("Scan Complete");

    loop {
        cortex_m::asm::wfi();       
    }
}

