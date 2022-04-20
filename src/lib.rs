#![no_std]

pub mod error;
mod register;

use embedded_hal::i2c::{blocking::I2c, SevenBitAddress};
use error::Error;
use register::Register;

const TPS_ADDRESS: u8 = 0b100_1000;

struct TPS6572x<T, E>
where
    T: I2c<SevenBitAddress, Error = E>,
{
    i2c: T,
}

impl<T, E> TPS6572x<T, E>
where
    T: I2c<SevenBitAddress, Error = E>,
{
    pub fn new(i2c: T) -> Self {
        Self { i2c }
    }

    pub fn write_register(&mut self, register: Register, val: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(TPS_ADDRESS, &[register as u8])
            .map_err(Error::I2c)?;
        self.i2c
            .write(TPS_ADDRESS, &[val as u8])
            .map_err(Error::I2c)?;
        Ok(())
    }

    pub fn read_register(&mut self, register: Register) -> Result<u8, Error<E>> {
        let mut val: [u8; 1] = [0; 1];
        self.i2c
            .write(TPS_ADDRESS, &[register as u8])
            .map_err(Error::I2c)?;
        self.i2c.write(TPS_ADDRESS, &mut val).map_err(Error::I2c)?;
        Ok(val[0])
    }
}
