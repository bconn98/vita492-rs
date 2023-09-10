pub mod vrt_header;
pub mod prolog;

/// The Object trait consists of 4 methods that all VITA 49.2 objects
/// are expected to contain: default construction, serialization/deserialization,
/// and size determination in bytes.
pub trait Object {
    type Item;

    /// Create a default Object of type Self::Item
    fn new() -> Self::Item;

    /// Serialize Self::Item into a vector of bytes.
    fn serialize(&self) -> Vec<u8>;

    /// Deserialize a vector of bytes into Self::Item.
    fn deserialize(buffer: Vec<u8>) -> Self::Item;

    /// Get the size of Self::Item in bytes as usize.
    fn get_num_bytes(&self) -> usize;
}

trait Packet<T: Object = Self>{
    /// Get the size of the Packet in words as usize.
    fn num_words(&self) -> usize;
}
