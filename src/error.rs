/// Possible error when reading an register error
#[derive(Debug, Clone, Copy)]
pub enum RegisterError<I2CError> {
    /// Internal i2c error
    I2c(I2CError),
}

#[derive(Debug, Clone, Copy)]
pub enum StuffError<LDOError, DCDCError, RSTError, INTError> {
    /// Internal HOLD_LDO pin error
    LDO(LDOError),

    /// Internal HOLD_DCDC pin error
    DCDC(DCDCError),

    /// Internal reset pin error
    Reset(RSTError),

    /// Internal interrupt pin error
    Interrupt(INTError),
}
