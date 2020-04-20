#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;

use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::i2c::I2c;

#[entry]
fn main() -> ! {
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);

    let mut scl = gpiob.pb10.into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);
    let scl = scl.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let mut sda = gpiob.pb11.into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);
    let sda = sda.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let mut i2c = I2c::i2c2(dp.I2C2, (scl, sda), 400.khz(), clocks, &mut rcc.apb1r1);
    let mut buffer = [0u8; 1];
    i2c.write_read(0x1B<<1, &[0x00], &mut buffer).unwrap();

    loop {}
}
