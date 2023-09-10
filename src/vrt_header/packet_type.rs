//! The Packet Type enumeration is utilized for defining the
//! seven supported VITA 49.2 packet types.

use std::convert::TryFrom;

/// The Packet Type enumeration is utilized for definining the
/// seven supported VITA 49.2 packet types.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PacketType {
    DataNoSid,
    DataSid,
    ExtDataNoSid,
    ExtDataSid,
    Context,
    ExtContext,
    Control,
    ExtControl,
}

impl TryFrom<u8> for PacketType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == PacketType::DataNoSid as u8 => Ok(PacketType::DataNoSid),
            x if x == PacketType::DataSid as u8 => Ok(PacketType::DataSid),
            x if x == PacketType::ExtDataNoSid as u8 => Ok(PacketType::ExtDataNoSid),
            x if x == PacketType::ExtDataSid as u8 => Ok(PacketType::ExtDataSid),
            x if x == PacketType::Context as u8 => Ok(PacketType::Context),
            x if x == PacketType::ExtContext as u8 => Ok(PacketType::ExtContext),
            x if x == PacketType::Control as u8 => Ok(PacketType::Control),
            x if x == PacketType::ExtControl as u8 => Ok(PacketType::ExtControl),
            _ => Err(()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_type_to_u8() {
        assert_eq!(PacketType::DataNoSid as u8, 0);
        assert_eq!(PacketType::DataSid as u8, 1);
        assert_eq!(PacketType::ExtDataNoSid as u8, 2);
        assert_eq!(PacketType::ExtDataSid as u8, 3);
        assert_eq!(PacketType::Context as u8, 4);
        assert_eq!(PacketType::ExtContext as u8, 5);
        assert_eq!(PacketType::Control as u8, 6);
        assert_eq!(PacketType::ExtControl as u8, 7);
    }

    #[test]
    fn packet_type_from_u8() {
        assert_eq!(0.try_into(), Ok(PacketType::DataNoSid));
        assert_eq!(1.try_into(), Ok(PacketType::DataSid));
        assert_eq!(2.try_into(), Ok(PacketType::ExtDataNoSid));
        assert_eq!(3.try_into(), Ok(PacketType::ExtDataSid));
        assert_eq!(4.try_into(), Ok(PacketType::Context));
        assert_eq!(5.try_into(), Ok(PacketType::ExtContext));
        assert_eq!(6.try_into(), Ok(PacketType::Control));
        assert_eq!(7.try_into(), Ok(PacketType::ExtControl));
    }
}
