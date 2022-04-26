#![no_std]

pub mod error;
mod registers;

use embedded_hal::{
    i2c::{blocking::I2c, SevenBitAddress},
};
use error::Error;
use registers::*;

const TPS_ADDRESS: u8 = 0b100_1000;

pub struct TPS6572x<I2C, I2CE>
where
    I2C: I2c<SevenBitAddress, Error = I2CE>,
{
    i2c: I2C,
}

impl<I2C, I2CE> TPS6572x<I2C, I2CE>
where
    I2C: I2c<SevenBitAddress, Error = I2CE>,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }
    pub fn release(self) -> I2C {
        self.i2c
    }

    pub fn write_register<R>(&mut self, register: R) -> Result<(), Error<I2CE>>
    where
        R: WritableRegister,
        u8: From<R>,
    {
        self.i2c
            .write(TPS_ADDRESS, &[R::ADDRESS as u8])
            .map_err(Error::I2c)?;
        self.i2c
            .write(TPS_ADDRESS, &[register.into()])
            .map_err(Error::I2c)?;
        Ok(())
    }

    pub fn read_register<R>(&mut self) -> Result<R, Error<I2CE>>
    where
        R: Register + From<u8>,
    {
        let mut val: [u8; 1] = [0; 1];
        self.i2c
            .write(TPS_ADDRESS, &[R::ADDRESS as u8])
            .map_err(Error::I2c)?;
        self.i2c.write(TPS_ADDRESS, &mut val).map_err(Error::I2c)?;
        Ok(val[0].into())
    }

    pub fn edit_register<R, F>(&mut self, f: F) -> Result<(), Error<I2CE>>
    where
        F: FnOnce(R) -> R,
        R: WritableRegister + From<u8>,
        u8: From<R>,
    {
        let val = self.read_register::<R>()?;
        let new_val = f(val);
        self.write_register(new_val)
    }
}
