#![no_std]

pub mod error;
mod registers;

use embedded_hal::{
    digital::blocking::{InputPin, OutputPin},
    i2c::{blocking::I2c, SevenBitAddress},
};
use error::RegisterError;
use registers::*;

const TPS_ADDRESS: u8 = 0b100_1000;

pub struct TPS65720<I2C, I2CE, LDO, LDOE, DCDC, DCDCE, RST, RSTE, INT, INTE>
where
    I2C: I2c<SevenBitAddress, Error = I2CE>,
    LDO: OutputPin<Error = LDOE>,
    DCDC: OutputPin<Error = DCDCE>,
    RST: InputPin<Error = RSTE>,
    INT: InputPin<Error = INTE>,
{
    i2c: I2C,
    hold_ldo: LDO,
    hold_dcdc: DCDC,
    reset: RST,
    interrupt: INT,
}

impl<I2C, I2CE, LDO, LDOE, DCDC, DCDCE, RST, RSTE, INT, INTE>
    TPS65720<I2C, I2CE, LDO, LDOE, DCDC, DCDCE, RST, RSTE, INT, INTE>
where
    I2C: I2c<SevenBitAddress, Error = I2CE>,
    LDO: OutputPin<Error = LDOE>,
    DCDC: OutputPin<Error = DCDCE>,
    RST: InputPin<Error = RSTE>,
    INT: InputPin<Error = INTE>,
{
    pub fn new(i2c: I2C, hold_ldo: LDO, hold_dcdc: DCDC, reset: RST, interrupt: INT) -> Self {
        Self {
            i2c,
            hold_ldo,
            hold_dcdc,
            reset,
            interrupt,
        }
    }
    pub fn release(self) -> I2C {
        self.i2c
    }

    pub fn write_register<R>(&mut self, register: R) -> Result<(), RegisterError<I2CE>>
    where
        R: WritableRegister,
        u8: From<R>,
    {
        self.i2c
            .write(TPS_ADDRESS, &[R::ADDRESS as u8])
            .map_err(RegisterError::I2c)?;
        self.i2c
            .write(TPS_ADDRESS, &[register.into()])
            .map_err(RegisterError::I2c)?;
        Ok(())
    }

    pub fn read_register<R>(&mut self) -> Result<R, RegisterError<I2CE>>
    where
        R: Register + From<u8>,
    {
        let mut val: [u8; 1] = [0; 1];
        self.i2c
            .write(TPS_ADDRESS, &[R::ADDRESS as u8])
            .map_err(RegisterError::I2c)?;
        self.i2c
            .write(TPS_ADDRESS, &mut val)
            .map_err(RegisterError::I2c)?;
        Ok(val[0].into())
    }

    pub fn edit_register<R, F>(&mut self, f: F) -> Result<(), RegisterError<I2CE>>
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
