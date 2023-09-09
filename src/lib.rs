pub mod vrt_header;

trait Object {
    type Item;

    fn new() -> Self::Item;
    fn packetize(&self) -> Vec<u8>;
    fn parse(buffer: Vec<u8>) -> Self::Item;
    fn get_num_bytes(&self) -> usize;
}

trait EnumType {
    type Item;
    
    fn to_u8(&self) -> u8;
    fn from_u8(value: u8) -> Self::Item;
}

trait Packet<T: Object = Self>{
    fn num_words(&self) -> usize;
}
