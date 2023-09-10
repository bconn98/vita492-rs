//! The integer timestamp enumeration is utilized for defining the
//! four supported VITA 49.2 integer timestamp types.

use std::convert::TryFrom;

/// The integer timestamp enumeration is utilized for definining the
/// four supported VITA 49.2 integer timestamp types.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IntegerTimestamps {
    Zero,
    One,
    Two,
    Three,
}

impl TryFrom<u8> for IntegerTimestamps {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == IntegerTimestamps::Zero as u8 => Ok(IntegerTimestamps::Zero),
            x if x == IntegerTimestamps::One as u8 => Ok(IntegerTimestamps::One),
            x if x == IntegerTimestamps::Two as u8 => Ok(IntegerTimestamps::Two),
            x if x == IntegerTimestamps::Three as u8 => Ok(IntegerTimestamps::Three),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn tsf_to_u8() {
        assert_eq!(IntegerTimestamps::Zero as u8, 0);
        assert_eq!(IntegerTimestamps::One as u8, 1);
        assert_eq!(IntegerTimestamps::Two as u8, 2);
        assert_eq!(IntegerTimestamps::Three as u8, 3);
    }

    #[test]
    fn tsf_from_u8() {
        assert_eq!(0.try_into(), Ok(IntegerTimestamps::Zero));
        assert_eq!(1.try_into(), Ok(IntegerTimestamps::One));
        assert_eq!(2.try_into(), Ok(IntegerTimestamps::Two));
        assert_eq!(3.try_into(), Ok(IntegerTimestamps::Three));
    }
}
