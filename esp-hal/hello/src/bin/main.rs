#![no_std]
#![no_main]

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{DriveMode, Level, Output, Pull};
use esp_hal::main;
use esp_println as _;
use sht4x::{Precision, Sht4x};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[main]
fn main() -> ! {
    // generator version: 0.3.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    info!("Hello world!");

    let i2c_pin_config = esp_hal::gpio::OutputConfig::default()
        .with_drive_mode(DriveMode::OpenDrain)
        .with_pull(Pull::Up);
    let scl = Output::new(peripherals.GPIO18, Level::High, i2c_pin_config);
    let sda = Output::new(peripherals.GPIO19, Level::High, i2c_pin_config);

    let i2c =
        esp_hal::i2c::master::I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default())
            .unwrap()
            .with_scl(scl)
            .with_sda(sda);
    let delay = Delay::new();
    let mut sht4x = Sht4x::new(i2c, delay);

    let serial = sht4x.serial_number().unwrap();
    info!("sensor serial: {:x}", serial);

    loop {
        let measurement = sht4x.measure(Precision::High).unwrap();
        info!("sensor measurement: {}", measurement);

        delay.delay_millis(3000);
    }
}
