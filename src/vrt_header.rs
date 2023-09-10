//! The VITA Radio Transport (VRT) header object with base VITA 49.2 functionality
//! of creation, serialization/deserialization, and sizing.

use crate::Object;

pub mod packet_type;
pub mod integer_timestamp;
pub mod fractional_timestamp;

/// The VRT header is a 32 bit (1 word/4 bytes) header than can be found
/// on all VITA 49.2 packets.
pub struct VrtHeader {
    packet_type: packet_type::PacketType,
    c_bit: bool,
    indicator_0: bool,
    indicator_1: bool,
    indicator_2: bool,
    tsi: integer_timestamp::IntegerTimestamps,
    tsf: fractional_timestamp::FractionalTimestamps,
    packet_count: u8,
    packet_size: u16,
}

impl VrtHeader {

    /// Get the current packet type from the VrtHeader.
    pub fn get_packet_type(&self) -> packet_type::PacketType {
        self.packet_type
    }

    /// Get the 'C Bit' from the VrtHeader.
    pub fn get_c_bit(&self) -> bool {
        self.c_bit
    }

    /// Get the Indicator 0 bit from the VrtHeader.
    pub fn get_ind_0(&self) -> bool {
        self.indicator_0
    }

    /// Get the Indicator 1 bit from the VrtHeader.
    pub fn get_ind_1(&self) -> bool {
        self.indicator_1
    }

    /// Get the Indicator 2 bit from the VrtHeader.
    pub fn get_ind_2(&self) -> bool {
        self.indicator_2
    }

    /// Get the Integer Timestamp setting from the VrtHeader.
    pub fn get_tsi(&self) -> integer_timestamp::IntegerTimestamps {
        self.tsi
    }

    /// Get the Fractional Timestamp setting from the VrtHeader.
    pub fn get_tsf(&self) -> fractional_timestamp::FractionalTimestamps {
        self.tsf
    }

    /// Get the Packet Count from the VrtHeader.
    pub fn get_packet_count(&self) -> u8 {
        self.packet_count
    }

    /// Get the Packet Size from the VrtHeader.
    pub fn get_packet_size(&self) -> u16 {
        self.packet_size
    }

    /// Set the packet type of the VrtHeader.
    pub fn set_packet_type(&mut self, packet_type: packet_type::PacketType) {
        self.packet_type = packet_type;
    }

    /// Set the C bit of the VrtHeader.
    pub fn set_c_bit(&mut self, c_bit: bool) {
        self.c_bit = c_bit;
    }

    /// Set the Indicator 0 bit of the VrtHeader.
    pub fn set_ind_0(&mut self, indicator_0: bool) {
        self.indicator_0 = indicator_0;
    }

    /// Set the Indicator 1 bit of the VrtHeader.
    pub fn set_ind_1(&mut self, indicator_1: bool) {
        self.indicator_1 = indicator_1;
    }

    /// Set the Indicator 2 bit of the VrtHeader.
    pub fn set_ind_2(&mut self, indicator_2: bool) {
        self.indicator_2 = indicator_2;
    }

    /// Set the Integer Timestamp setting of the VrtHeader.
    pub fn set_tsi(&mut self, tsi: integer_timestamp::IntegerTimestamps) {
        self.tsi = tsi;
    }

    /// Set the Fractional Timestamp setting of the VrtHeader.
    pub fn set_tsf(&mut self, tsf: fractional_timestamp::FractionalTimestamps) {
        self.tsf = tsf;
    }

    /// Set the packet count setting of the VrtHeader.
    pub fn set_packet_count(&mut self, packet_count: u8) {
        self.packet_count = packet_count;
    }

    /// Set the packet size setting of the VrtHeader.
    pub fn set_packet_size(&mut self, packet_size: u16) {
        self.packet_size = packet_size;
    }

    /// Helper to convert a byte to a boolean.
    fn bit_to_bool(value: u8) -> bool {
        value == 1
    }
}

impl Object for VrtHeader {
    /// The Object is of type VrtHeader.
    type Item = VrtHeader;

    /// Create a default VrtHeader.
    fn new() -> VrtHeader {
        VrtHeader {
            packet_type: packet_type::PacketType::DataNoSid,
            c_bit: false,
            indicator_0: false,
            indicator_1: false,
            indicator_2: false,
            tsi: integer_timestamp::IntegerTimestamps::Zero,
            tsf: fractional_timestamp::FractionalTimestamps::SampleCount,
            packet_count: 0,
            packet_size: 0,
        }
    }

    /// Serialize VrtHeader into a vector of bytes.
    fn serialize(&self) -> Vec<u8> {
        let mut packet_vec: Vec<u8> = Vec::with_capacity(self.get_num_bytes());

        let mut first_byte: u8 = 0;
        first_byte |= (self.packet_type as u8) << 4;
        first_byte |= (self.c_bit as u8) << 3;
        first_byte |= (self.indicator_0 as u8) << 2;
        first_byte |= (self.indicator_1 as u8) << 1;
        first_byte |= self.indicator_2 as u8;

        packet_vec.push(first_byte);

        let mut second_byte: u8 = 0;
        second_byte |= ((self.tsi as u8) << 6) & 0b11000000;
        second_byte |= ((self.tsf as u8) << 4) & 0b00110000;
        second_byte |= self.packet_count & 0b00001111;

        packet_vec.push(second_byte);

        let [third_byte, forth_byte] = self.packet_size.to_be_bytes();
        packet_vec.push(third_byte);
        packet_vec.push(forth_byte);

        packet_vec
    }

    /// Deserialize a vector of bytes into VrtHeader
    fn deserialize(buffer: Vec<u8>) -> VrtHeader {
        let mut vrt: VrtHeader = VrtHeader::new();
        assert!(buffer.len() >= vrt.get_num_bytes());

        let first_byte: u8 = buffer[0];
        vrt.packet_type = ((first_byte >> 4) & 0b00001111).try_into().expect("Not a valid packet type setting");
        vrt.c_bit = VrtHeader::bit_to_bool((first_byte >> 3) & 0b00000001);
        vrt.indicator_0 = VrtHeader::bit_to_bool((first_byte >> 2) & 0b00000001);
        vrt.indicator_1 = VrtHeader::bit_to_bool((first_byte >> 1) & 0b00000001);
        vrt.indicator_2 = VrtHeader::bit_to_bool(first_byte & 0b00000001);

        let second_byte: u8 = buffer[1];
        vrt.tsi = ((second_byte >> 6) & 0b00000011).try_into().expect("Not a valid integer timestamp setting.");
        vrt.tsf = ((second_byte >> 4) & 0b00000011).try_into().expect("Not a valid fractional timestamp setting.");
        vrt.packet_count = second_byte & 0b00001111;

        let third_byte: u8 = buffer[2];
        let forth_byte: u8 = buffer[3];

        vrt.packet_size = ((third_byte as u16) << 8) & 0b1111111100000000;
        vrt.packet_size |= (forth_byte as u16) & 0b0000000011111111;

        vrt
    }

    /// Get the size of the VrtHeader in Bytes, this will
    /// be the constant value of 4.
    fn get_num_bytes(&self) -> usize {
        let num_bytes = 4;
        num_bytes
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_creation() {
        let vrt: VrtHeader = VrtHeader::new();

        assert_eq!(vrt.get_packet_type(), packet_type::PacketType::DataNoSid);
        assert_eq!(vrt.get_c_bit(), false);
        assert_eq!(vrt.get_ind_0(), false);
        assert_eq!(vrt.get_ind_1(), false);
        assert_eq!(vrt.get_ind_2(), false);
        assert_eq!(vrt.get_tsi(), integer_timestamp::IntegerTimestamps::Zero);
        assert_eq!(vrt.get_tsf(), fractional_timestamp::FractionalTimestamps::SampleCount);
        assert_eq!(vrt.get_packet_count(), 0);
        assert_eq!(vrt.get_packet_size(), 0);
    }

    #[test]
    fn empty_serialize() {
        let vrt: VrtHeader = VrtHeader::new();
        let empty_header: Vec<u8> = vrt.serialize();

        assert_ne!(empty_header.is_empty(), true);
        assert_eq!(empty_header.len(), 4);

        for byte in empty_header {
            assert_eq!(byte, 0);
        }
    }

    #[test]
    fn serialize() {
        let mut vrt: VrtHeader = VrtHeader::new();
        vrt.set_ind_1(true);
        vrt.set_tsi(integer_timestamp::IntegerTimestamps::One);
        vrt.set_packet_size(8);

        let header: Vec<u8> = vrt.serialize();

        assert_ne!(header.is_empty(), true);
        assert_eq!(header.len(), 4);

        assert_eq!(header[0], 2);
        assert_eq!(header[1], 64); // 2^6 == 64
        assert_eq!(header[2], 0);
        assert_eq!(header[3], 8);
    }

    #[test]
    fn get_num_bytes() {
        let vrt: VrtHeader = VrtHeader::new();

        assert_eq!(vrt.get_num_bytes(), 4);
    }

    #[test]
    fn empty_deserialize() {
        let mut buffer: Vec<u8> = Vec::with_capacity(4);
        buffer.push(0);
        buffer.push(0);
        buffer.push(0);
        buffer.push(0);

        let vrt: VrtHeader = VrtHeader::deserialize(buffer);

        assert_eq!(vrt.get_packet_type(), packet_type::PacketType::DataNoSid);
        assert_eq!(vrt.get_c_bit(), false);
        assert_eq!(vrt.get_ind_0(), false);
        assert_eq!(vrt.get_ind_1(), false);
        assert_eq!(vrt.get_ind_2(), false);
        assert_eq!(vrt.get_tsi(), integer_timestamp::IntegerTimestamps::Zero);
        assert_eq!(vrt.get_tsf(), fractional_timestamp::FractionalTimestamps::SampleCount);
        assert_eq!(vrt.get_packet_count(), 0);
        assert_eq!(vrt.get_packet_size(), 0);       
    }

    #[test]
    fn deserialize() {
        let mut buffer: Vec<u8> = Vec::with_capacity(4);
        buffer.push(0x1A);
        buffer.push(0x70);
        buffer.push(0x08);
        buffer.push(0x0);

        let vrt: VrtHeader = VrtHeader::deserialize(buffer);

        assert_eq!(vrt.get_packet_type(), packet_type::PacketType::DataSid);
        assert_eq!(vrt.get_c_bit(), true);
        assert_eq!(vrt.get_ind_0(), false);
        assert_eq!(vrt.get_ind_1(), true);
        assert_eq!(vrt.get_ind_2(), false);
        assert_eq!(vrt.get_tsi(), integer_timestamp::IntegerTimestamps::One);
        assert_eq!(vrt.get_tsf(), fractional_timestamp::FractionalTimestamps::Other);
        assert_eq!(vrt.get_packet_count(), 0);
        assert_eq!(vrt.get_packet_size(), 2048);   
    }

    #[test]
    fn packet_type() {
        let mut vrt: VrtHeader = VrtHeader::new();
        assert_eq!(vrt.get_packet_type(), packet_type::PacketType::DataNoSid);

        vrt.set_packet_type(packet_type::PacketType::Context);
        assert_eq!(vrt.get_packet_type(), packet_type::PacketType::Context);
    }

    #[test]
    fn c_bit() {
        let mut vrt: VrtHeader = VrtHeader::new();
        assert_eq!(vrt.get_c_bit(), false);

        vrt.set_c_bit(true);
        assert_eq!(vrt.get_c_bit(), true);
    }

    #[test]
    fn indicator_0() {
        let mut vrt: VrtHeader = VrtHeader::new();
        assert_eq!(vrt.get_ind_0(), false);

        vrt.set_ind_0(true);
        assert_eq!(vrt.get_ind_0(), true);
    }

    #[test]
    fn indicator_1() {
        let mut vrt: VrtHeader = VrtHeader::new();
        assert_eq!(vrt.get_ind_1(), false);

        vrt.set_ind_1(true);
        assert_eq!(vrt.get_ind_1(), true);
    }

    #[test]
    fn indicator_2() {
        let mut vrt: VrtHeader = VrtHeader::new();
        assert_eq!(vrt.get_ind_2(), false);

        vrt.set_ind_2(true);
        assert_eq!(vrt.get_ind_2(), true);
    }

    #[test]
    fn tsi() {
        let mut vrt: VrtHeader = VrtHeader::new();
        assert_eq!(vrt.get_tsi(), integer_timestamp::IntegerTimestamps::Zero);

        vrt.set_tsi(integer_timestamp::IntegerTimestamps::Two);
        assert_eq!(vrt.get_tsi(), integer_timestamp::IntegerTimestamps::Two);
    }

    #[test]
    fn tsf() {
        let mut vrt: VrtHeader = VrtHeader::new();
        assert_eq!(vrt.get_tsf(), fractional_timestamp::FractionalTimestamps::SampleCount);

        vrt.set_tsf(fractional_timestamp::FractionalTimestamps::RealTimePico);
        assert_eq!(vrt.get_tsf(), fractional_timestamp::FractionalTimestamps::RealTimePico);
    }

    #[test]
    fn packet_count() {
        let mut vrt: VrtHeader = VrtHeader::new();
        assert_eq!(vrt.get_packet_count(), 0);

        vrt.set_packet_count(5);
        assert_eq!(vrt.get_packet_count(), 5);
    }

    #[test]
    fn packet_size() {
        let mut vrt: VrtHeader = VrtHeader::new();
        assert_eq!(vrt.get_packet_size(), 0);

        vrt.set_packet_size(35000);
        assert_eq!(vrt.get_packet_size(), 35000);
    }
}
