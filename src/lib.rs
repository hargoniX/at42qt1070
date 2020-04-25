#![no_std]
use bitfield::bitfield;
use embedded_hal::blocking::i2c::{Write, WriteRead};

#[derive(Clone, Copy, Debug)]
pub enum Error<I2cError> {
    I2cError(I2cError),
    IdMismatch(u8),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
}

bitfield! {
    pub struct Status(u8);
    impl Debug;
    pub calibrate, _: 7;
    pub overflow, _: 6;
    pub touch, _: 0;
}

bitfield! {
    pub struct KeyStatus(u8);
    impl Debug;
    pub key6, _: 6;
    pub key5, _: 5;
    pub key4, _: 4;
    pub key3, _: 3;
    pub key2, _: 2;
    pub key1, _: 1;
    pub key0, _: 0;
}

pub struct Driver<I2C> {
    i2c: I2C,
}

impl<I2C, I2cError> Driver<I2C>
where
    I2C: WriteRead<Error = I2cError> + Write<Error = I2cError>,
{
    pub fn new(i2c: I2C) -> Result<Driver<I2C>, Error<I2cError>> {
        let mut driver = Driver { i2c: i2c };

        let id = driver.get_id()?;
        if id != Chip::ID {
            return Err(Error::IdMismatch(id));
        }

        Ok(driver)
    }

    pub fn calibrate(&mut self) -> Result<(), Error<I2cError>> {
        self.i2c.write(Chip::I2C, &[Chip::CALIBRATE_ADDR, 0xFF])?;

        loop {
            let status = self.get_status()?;
            if !status.calibrate() {
                break;
            }
        }

        Ok(())
    }

    pub fn get_status(&mut self) -> Result<Status, Error<I2cError>> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(Chip::I2C, &[Chip::STATUS_ADDR], &mut buffer)?;
        Ok(Status(buffer[0]))
    }

    pub fn get_key_status(&mut self) -> Result<KeyStatus, Error<I2cError>> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(Chip::I2C, &[Chip::KEY_STATUS_ADDR], &mut buffer)?;
        Ok(KeyStatus(buffer[0]))
    }

    fn get_id(&mut self) -> Result<u8, Error<I2cError>> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(Chip::I2C, &[Chip::ID_ADDR], &mut buffer)?;
        Ok(buffer[0])
    }
}

mod Chip {
    pub const I2C: u8 = 0x1B << 1;
    pub const ID: u8 = 0x2E;
    pub const ID_ADDR: u8 = 0;
    pub const STATUS_ADDR: u8 = 2;
    pub const KEY_STATUS_ADDR: u8 = 3;
    pub const CALIBRATE_ADDR: u8 = 56;
}
