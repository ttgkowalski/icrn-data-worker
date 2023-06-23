pub enum DataBlockSize{
    MBx8  = 8388608,
    MBx16 = 16777216,
    MBx32 = 33554432,
    MBx64 = 67108864,
}

/// Summary
/// - **block_hash:** sha256 digest of all the fields(except payload) concatenated by "."
/// - **previous_block_hash:** required if the segment_number != 0. used to remount the file
/// - **object_uuid:** the object is like the whole virtual file, this field put together all data blocks
/// - **file_name:** the real filename, can includes the extensions
/// - **segment_number:** used to remount the file
/// - **size:** used to calculate billing and distribute the file across the oceans
/// - **payload:** the data block itself. it's a part of the file
/// - **payload_digest_256:** sha256 digest of the payload
pub struct DataBlock {
    pub block_hash: String,
    pub previous_block_hash: String,
    pub object_uuid: String,
    pub file_name: String,
    pub segment_number: u32,
    pub size: DataBlockSize,
    pub payload: Vec<u8>,
    pub digest_256: String,
}

// impl DataBlock {
//     pub fn from_u8_vec(self) -> Self {
        
//     }
// }

pub fn hiasid() {
    let block_size = DataBlockSize::MBx64;

    let expected_parity_block_size = block_size as u8/2;
    println!("{}", expected_parity_block_size)
}