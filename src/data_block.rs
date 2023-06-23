use crate::object::Object;

/// Enum in MB, Value in Bytes
#[derive(Debug, Copy, Clone)]
pub enum DataBlockSize{
    MBx8  = 8388608,
    MBx16 = 16777216,
    MBx32 = 33554432,
    MBx64 = 67108864,
}

/// Summary
/// - **block_hash:** sha256 digest of all the fields(except payload) concatenated by "."
/// - **object_uuid:** the object is like the whole virtual file, this field put together all data blocks
/// - **file_name:** the real filename, can includes the extensions
/// - **segment_number:** used to remount the file
/// - **block_size:** used to calculate billing and distribute the file across the oceans
/// - **payload:** the data block itself. it's a part of the file
/// - **payload_digest_256:** sha256 digest of the payload
pub struct DataBlock {
    pub block_hash: String,
    pub file_name: String,
    pub segment_number: u32,
    pub block_size: DataBlockSize,
    pub payload: Vec<u8>,
    pub payload_digest_256: String,
}
