use crate::{EnumType};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IntegerTimestamps {
    Zero,
    One,
    Two,
    Three,
}

impl EnumType for IntegerTimestamps {
    type Item = IntegerTimestamps;

    fn to_u8(&self) -> u8 {
        match self {
            IntegerTimestamps::Zero => 0,
            IntegerTimestamps::One => 1,
            IntegerTimestamps::Two => 2,
            IntegerTimestamps::Three => 3,
        }
    }

    fn from_u8(value: u8) -> IntegerTimestamps {
        match value {
            0 => IntegerTimestamps::Zero,
            1 => IntegerTimestamps::One,
            2 => IntegerTimestamps::Two,
            3 => IntegerTimestamps::Three,
            _ => panic!("Unsupported u8 value for IntegerTimestamps")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn tsi_to_u8() {
        let tsi: IntegerTimestamps = IntegerTimestamps::Zero;
        assert_eq!(tsi.to_u8(), 0);

        let tsi: IntegerTimestamps = IntegerTimestamps::One;
        assert_eq!(tsi.to_u8(), 1);

        let tsi: IntegerTimestamps = IntegerTimestamps::Two;
        assert_eq!(tsi.to_u8(), 2);

        let tsi: IntegerTimestamps = IntegerTimestamps::Three;
        assert_eq!(tsi.to_u8(), 3);
    }

    #[test]
    fn tsi_from_u8() {
        assert_eq!(IntegerTimestamps::from_u8(0), IntegerTimestamps::Zero);
        assert_eq!(IntegerTimestamps::from_u8(1), IntegerTimestamps::One);
        assert_eq!(IntegerTimestamps::from_u8(2), IntegerTimestamps::Two);
        assert_eq!(IntegerTimestamps::from_u8(3), IntegerTimestamps::Three);
    }
}
