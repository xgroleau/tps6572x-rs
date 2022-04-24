#![no_std]

pub mod error;
mod registers;

use embedded_hal::i2c::{blocking::I2c, SevenBitAddress};
use error::Error;
use registers::*;

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
    pub fn release(self) -> T {
        self.i2c
    }

    fn write_register<R>(&mut self, register: R) -> Result<(), Error<E>>
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

    fn read_register<R>(&mut self) -> Result<R, Error<E>>
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
}
