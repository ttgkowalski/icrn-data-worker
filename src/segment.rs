use std::fs;

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
    pub segment_number: u32,
    pub payload: Vec<u8>,
    pub payload_digest_256: String,
}

impl FileSegment {
    pub fn from_u8_vec(payload: Vec<u8>, segment_number: u32) -> FileSegment {
        Self {
            segment_number,
            payload,
            payload_digest_256: String::from("fake_digest"),
        }
    }
}

impl Drop for FileSegment {
    fn drop(&mut self) {
        println!("Dropping segment {} from the memory", self.segment_number);
    }
}

pub fn segment_file(file_path: &String) -> Vec<FileSegment> {
    let qnt_segments: u32 = calculate_segments(calculate_file_size_in_bytes(file_path));
    let mut segments: Vec<FileSegment> = Vec::new();

    let file_content: Vec<u8> = fs::read(file_path).expect("Couldn't read file").to_vec();
    let file_length: u32 = file_content.len().try_into().unwrap();

    for segment in 0..qnt_segments {
        let segment_start: u32 = segment * SegmentSize::Block8MB as u32;

        let segment_end: u32 = match (segment_start + SegmentSize::Block8MB as u32) < file_length {
            true => segment_start + SegmentSize::Block8MB as u32,
            false => segment_start + file_length - segment_start,
        };

        println!(
            "Allocating segment {} ({}->{}) on memory",
            segment, segment_start, segment_end
        );
        let payload: Vec<u8> = file_content[segment_start as usize..segment_end as usize].to_vec();
        let file_segment: FileSegment = FileSegment::from_u8_vec(payload, segment);

        segments.push(file_segment);
    }

    return segments;
}

pub fn calculate_segments(file_size: usize) -> u32 {
    let block_size: usize = SegmentSize::Block8MB as usize;

    if file_size < block_size {
        return 1;
    } else if file_size % block_size > 0 {
        return (file_size / block_size + 1).try_into().unwrap();
    } else {
        return (file_size / block_size).try_into().unwrap();
    }
}

pub fn calculate_file_size_in_bytes(file_path: &String) -> usize {
    return fs::read(&file_path)
        .expect("Should have been able to read the file")
        .to_vec()
        .len();
}
