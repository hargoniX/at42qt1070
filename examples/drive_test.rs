#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;

use stm32l4xx_hal::i2c::I2c;
use stm32l4xx_hal::prelude::*;
use at42qt1070::Driver;

#[entry]
fn main() -> ! {
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);

    let scl = gpiob
        .pb10
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);
    let scl = scl.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let sda = gpiob
        .pb11
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);
    let sda = sda.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let i2c = I2c::i2c2(dp.I2C2, (scl, sda), 400.khz(), clocks, &mut rcc.apb1r1);
    let mut driver = Driver::new(i2c).unwrap();
    driver.calibrate().unwrap();

    loop {
        let status = driver.get_status().unwrap();
        if status.touch() {
            break;
        }
    }

    let key_status = driver.get_key_status().unwrap();
    let key0 = key_status.key0();
    let key1 = key_status.key1();
    let key2 = key_status.key2();
    let key3 = key_status.key3();
    let key4 = key_status.key4();
    let key5 = key_status.key5();
    let key6 = key_status.key6();

    loop {}
}
