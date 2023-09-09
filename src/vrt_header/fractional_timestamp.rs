use crate::{EnumType};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FractionalTimestamps {
    Zero,
    One,
    Two,
    Three,
}

impl EnumType for FractionalTimestamps {
    type Item = FractionalTimestamps;

    fn to_u8(&self) -> u8 {
        match self {
            FractionalTimestamps::Zero => 0,
            FractionalTimestamps::One => 1,
            FractionalTimestamps::Two => 2,
            FractionalTimestamps::Three => 3,
        }
    }

    fn from_u8(value: u8) -> FractionalTimestamps {
        match value {
            0 => FractionalTimestamps::Zero,
            1 => FractionalTimestamps::One,
            2 => FractionalTimestamps::Two,
            3 => FractionalTimestamps::Three,
            _ => panic!("Unsupported u8 value for FractionalTimestamps")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn tsf_to_u8() {
        let tsf: FractionalTimestamps = FractionalTimestamps::Zero;
        assert_eq!(tsf.to_u8(), 0);

        let tsf: FractionalTimestamps = FractionalTimestamps::One;
        assert_eq!(tsf.to_u8(), 1);

        let tsf: FractionalTimestamps = FractionalTimestamps::Two;
        assert_eq!(tsf.to_u8(), 2);

        let tsf: FractionalTimestamps = FractionalTimestamps::Three;
        assert_eq!(tsf.to_u8(), 3);
    }

    #[test]
    fn tsf_from_u8() {
        assert_eq!(FractionalTimestamps::from_u8(0), FractionalTimestamps::Zero);
        assert_eq!(FractionalTimestamps::from_u8(1), FractionalTimestamps::One);
        assert_eq!(FractionalTimestamps::from_u8(2), FractionalTimestamps::Two);
        assert_eq!(FractionalTimestamps::from_u8(3), FractionalTimestamps::Three);
    }
}
