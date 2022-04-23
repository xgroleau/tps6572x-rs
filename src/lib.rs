#![no_std]

pub mod error;
mod register;

use embedded_hal::i2c::{blocking::I2c, SevenBitAddress};
use error::Error;
use register::*;

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

    fn write_register(&mut self, register: RegisterAddress, val: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(TPS_ADDRESS, &[register as u8])
            .map_err(Error::I2c)?;
        self.i2c
            .write(TPS_ADDRESS, &[val as u8])
            .map_err(Error::I2c)?;
        Ok(())
    }

    fn read_register(&mut self, register: RegisterAddress) -> Result<u8, Error<E>> {
        let mut val: [u8; 1] = [0; 1];
        self.i2c
            .write(TPS_ADDRESS, &[register as u8])
            .map_err(Error::I2c)?;
        self.i2c.write(TPS_ADDRESS, &mut val).map_err(Error::I2c)?;
        Ok(val[0])
    }

    pub fn get_charger_status(&mut self) -> Result<ChargerStatus, Error<E>> {
        let val = self.read_register(RegisterAddress::BatteryChargeState)?;
        Ok(val.into())
    }

    pub fn get_charger_config_0(&mut self) -> Result<ChargerConfig0, Error<E>> {
        let val = self.read_register(RegisterAddress::BatteryChargerConfigControl0)?;
        Ok(val.into())
    }
}
