use crate::{EnumType};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PacketType {
    DataNoSid,
    DataSid,
    ExtDataNoSid,
    ExtDataSid,
    Context,
}

impl EnumType for PacketType {
    type Item = PacketType;

    fn to_u8(&self) -> u8 {
        match self {
            PacketType::DataNoSid => 0,
            PacketType::DataSid => 1,
            PacketType::ExtDataNoSid => 2,
            PacketType::ExtDataSid => 3,
            PacketType::Context => 4,
        }
    }

    fn from_u8(value: u8) -> PacketType {
        match value {
            0 => PacketType::DataNoSid,
            1 => PacketType::DataSid,
            2 => PacketType::ExtDataNoSid,
            3 => PacketType::ExtDataSid,
            4 => PacketType::Context,
            _ => panic!("Unsupported u8 value for PacketType")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_type_to_u8() {
        let packet_type: PacketType = PacketType::DataNoSid;
        assert_eq!(packet_type.to_u8(), 0);
        
        let packet_type: PacketType = PacketType::DataSid;
        assert_eq!(packet_type.to_u8(), 1);

        let packet_type: PacketType = PacketType::ExtDataNoSid;
        assert_eq!(packet_type.to_u8(), 2);

        let packet_type: PacketType = PacketType::ExtDataSid;
        assert_eq!(packet_type.to_u8(), 3);

        let packet_type: PacketType = PacketType::Context;
        assert_eq!(packet_type.to_u8(), 4);
    }

    #[test]
    fn packet_type_from_u8() {
        assert_eq!(PacketType::from_u8(0), PacketType::DataNoSid);
        assert_eq!(PacketType::from_u8(1), PacketType::DataSid);
        assert_eq!(PacketType::from_u8(2), PacketType::ExtDataNoSid);
        assert_eq!(PacketType::from_u8(3), PacketType::ExtDataSid);
        assert_eq!(PacketType::from_u8(4), PacketType::Context);
    }
}
