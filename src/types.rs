use config::Config;

/// PCA9685 PWM/Servo/LED controller.
#[derive(Debug, Default)]
pub struct Pca9685<I2C> {
    /// The concrete I²C device implementation.
    pub(crate) i2c: I2C,
    /// The I²C device address.
    pub(crate) address: u8,
    /// Current device configuration.
    pub(crate) config: Config,
}

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data provided
    InvalidInputData,
}

/// Output channel selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Channel {
    /// Channel 0
    C0,
    /// Channel 1
    C1,
    /// Channel 2
    C2,
    /// Channel 3
    C3,
    /// Channel 4
    C4,
    /// Channel 5
    C5,
    /// Channel 6
    C6,
    /// Channel 7
    C7,
    /// Channel 8
    C8,
    /// Channel 9
    C9,
    /// Channel 10
    C10,
    /// Channel 11
    C11,
    /// Channel 12
    C12,
    /// Channel 13
    C13,
    /// Channel 14
    C14,
    /// Channel 15
    C15,
    /// All channels
    All,
}

/// Output logic state inversion
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputLogicState {
    /// Output logic state is not inverted.
    ///
    /// Value to set when external driver is used. Applicable when `OE = 0`.
    Direct,
    /// Output logic state is inverted.
    ///
    /// Value to set when no external driver is used. Applicable when `OE = 0`.
    Inverted,
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A5, A4, A3, A2, A1 and A0
    Alternative(bool, bool, bool, bool, bool, bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    pub(crate) fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a5, a4, a3, a2, a1, a0) => {
                default
                    | ((a5 as u8) << 5)
                    | ((a4 as u8) << 4)
                    | ((a3 as u8) << 3)
                    | ((a2 as u8) << 2)
                    | ((a1 as u8) << 1)
                    | a0 as u8
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DEVICE_BASE_ADDRESS as DEV_ADDR;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(DEV_ADDR, addr.addr(DEV_ADDR));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(
            0b100_0000,
            SlaveAddr::Alternative(false, false, false, false, false, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b100_0001,
            SlaveAddr::Alternative(false, false, false, false, false, true).addr(DEV_ADDR)
        );
        assert_eq!(
            0b100_0010,
            SlaveAddr::Alternative(false, false, false, false, true, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b100_0100,
            SlaveAddr::Alternative(false, false, false, true, false, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b100_1000,
            SlaveAddr::Alternative(false, false, true, false, false, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b101_0000,
            SlaveAddr::Alternative(false, true, false, false, false, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b110_0000,
            SlaveAddr::Alternative(true, false, false, false, false, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b111_1111,
            SlaveAddr::Alternative(true, true, true, true, true, true).addr(DEV_ADDR)
        );
    }
}
