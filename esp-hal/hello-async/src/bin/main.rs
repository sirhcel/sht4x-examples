#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{DriveMode, Level, Output, OutputConfig, Pull};
use esp_hal::timer::systimer::SystemTimer;
use esp_println as _;
use sht4x::{Precision, Sht4xAsync};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // generator version: 0.3.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let i2c_pin_config = OutputConfig::default()
        .with_drive_mode(DriveMode::OpenDrain)
        .with_pull(Pull::Up);
    let scl = Output::new(peripherals.GPIO18, Level::High, i2c_pin_config);
    let sda = Output::new(peripherals.GPIO19, Level::High, i2c_pin_config);

    let i2c =
        esp_hal::i2c::master::I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default())
            .unwrap()
            .with_scl(scl)
            .with_sda(sda)
            .into_async();
    let mut sht4x = Sht4xAsync::new(i2c);
    let mut delay = Delay;

    info!("Hello world!");

    let serial = sht4x.serial_number(&mut delay).await.unwrap();
    info!("sensor serial: {:x}", serial);

    loop {
        let measurement = sht4x.measure(Precision::High, &mut delay).await.unwrap();
        info!("sensor measurement: {}", measurement);

        Timer::after(Duration::from_secs(3)).await;
    }
}
