#![no_std]

pub mod error;
mod register;

use bit::BitIndex;
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
        Ok(ChargerStatus {
            temp_high: val.bit(7),
            temp_low: val.bit(6),
            over_voltage_protection: val.bit(5),
            charger_active: val.bit(3),
            power_source_ok: val.bit(2),
            thermal_loop_active: val.bit(1),
        })
    }

    pub fn get_charger_config_0(&mut self) -> Result<ChargerConfig0, Error<E>> {
        let val = self.read_register(RegisterAddress::BatteryChargeState)?;
        let output_voltage =
            OutputVoltage::try_from(val.bit_range(6..8)).map_err(|_| Error::UnexpectedValue)?;
        let ac_input_current =
            AcInputCurrent::try_from(val.bit_range(4..6)).map_err(|_| Error::UnexpectedValue)?;

        Ok(ChargerConfig0 {
            output_voltage,
            ac_input_current,
            thermal_loop: val.bit(3),
            dynamic_timer: val.bit(2),
            termination_enabled: val.bit(1),
            charger_enabled: val.bit(0),
        })
    }
}
