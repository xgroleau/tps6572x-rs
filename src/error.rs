/// All possible errors emitted by the driver
#[derive(Debug, Clone, Copy)]
pub enum Error<I2cError> {
    /// Internal i2c error
    I2c(I2cError),

    /// Cannot decode value
    UnexpectedValue,
}
