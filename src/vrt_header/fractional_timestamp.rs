//! The fractional timestamp enumeration is utilized for defining the
//! four supported VITA 49.2 fractional timestamp types.

use std::convert::TryFrom;

/// The fractional timestamp enumeration is utilized for definining the
/// four supported VITA 49.2 fractional timestamp types.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FractionalTimestamps {
    SampleCount,
    RealTimePico,
    FreeRunning,
    Other,
}

impl TryFrom<u8> for FractionalTimestamps {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == FractionalTimestamps::SampleCount as u8 => Ok(FractionalTimestamps::SampleCount),
            x if x == FractionalTimestamps::RealTimePico as u8 => Ok(FractionalTimestamps::RealTimePico),
            x if x == FractionalTimestamps::FreeRunning as u8 => Ok(FractionalTimestamps::FreeRunning),
            x if x == FractionalTimestamps::Other as u8 => Ok(FractionalTimestamps::Other),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tsf_to_u8() {
        assert_eq!(FractionalTimestamps::SampleCount as u8, 0);
        assert_eq!(FractionalTimestamps::RealTimePico as u8, 1);
        assert_eq!(FractionalTimestamps::FreeRunning as u8, 2);
        assert_eq!(FractionalTimestamps::Other as u8, 3);
    }

    #[test]
    fn tsf_from_u8() {
        assert_eq!(0.try_into(), Ok(FractionalTimestamps::SampleCount));
        assert_eq!(1.try_into(), Ok(FractionalTimestamps::RealTimePico));
        assert_eq!(2.try_into(), Ok(FractionalTimestamps::FreeRunning));
        assert_eq!(3.try_into(), Ok(FractionalTimestamps::Other));
    }
}
