/// Possible error when reading an register error
#[derive(Debug, Clone, Copy)]
pub enum Error<I2CError> {
    /// Internal i2c error
    I2c(I2CError),
}
