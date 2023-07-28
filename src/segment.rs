use std::io::{Read, Seek, SeekFrom};

/// Enum in MB, Value in Bytes
#[derive(Debug, Copy, Clone)]
pub enum SegmentSize {
    Block8MB = 8388608,
    Block16MB = 16777216,
    Block32MB = 33554432,
    Block64MB = 67108864,
}

#[derive(Default, Debug, Clone)]
pub struct FileSegment {
    pub segment_number: usize,
    pub first_chunk: Vec<u8>,
    pub last_chunk: Vec<u8>,
}

impl FileSegment {
    pub fn from_u8_vec(mut payload: Vec<u8>, segment_number: usize) -> FileSegment {
        let last_chunk = payload.split_off(payload.len() / 2);
        Self {
            segment_number,
            first_chunk: payload,
            last_chunk,
        }
    }
}

//impl Drop for FileSegment {
//    fn drop(&mut self) {
//        println!("Dropping segment {} from the memory", self.segment_number);
//    }
//}

pub fn segment_file<B: Read + Seek>(buff: &mut B) -> Vec<FileSegment> {
    let qnt_segments: usize = calculate_segments(calculate_file_size_in_bytes(buff));
    let mut segments: Vec<FileSegment> = Vec::new();

    let mut file_content = Vec::<u8>::new();

    buff.read_to_end(&mut file_content)
        .expect("Couldn't read file");

    let file_length = file_content.len();

    for segment in 0..qnt_segments {
        let segment_start = segment * SegmentSize::Block8MB as usize;

        let segment_end = (segment_start + SegmentSize::Block8MB as usize).min(file_length);

        // println!("Allocating segment {} ({}->{}) on memory", segment, segment_start, segment_end);

        let payload: Vec<u8> = file_content[segment_start..segment_end].to_vec();
        let file_segment: FileSegment = FileSegment::from_u8_vec(payload, segment);

        segments.push(file_segment);
    }

    segments
}

#[must_use]
pub const fn calculate_segments(file_size: usize) -> usize {
    let block_size: usize = SegmentSize::Block8MB as usize;

    if file_size < block_size {
        1
    } else if file_size % block_size > 0 {
        file_size / block_size + 1
    } else {
        file_size / block_size
    }
}

pub fn calculate_file_size_in_bytes<B>(buff: &mut B) -> usize
where
    B: Read + Seek,
{
    let file_size: usize = buff
        .seek(SeekFrom::End(0))
        .expect("Couldn't calculate file size") as usize
        + 1;
    buff.seek(SeekFrom::Start(0))
        .expect("Couldn't reset file pointer");
    file_size - 1
}
