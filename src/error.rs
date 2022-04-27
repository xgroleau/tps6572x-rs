/// Possible error when reading an register error
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy)]
pub enum Error<I2CError> {
    /// Internal i2c error
    I2c(I2CError),
}
