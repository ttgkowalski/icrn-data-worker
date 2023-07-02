use crate::segment::FileSegment;

#[derive(Debug)]
pub struct SegmentParity {
    pub segment_number: usize,
    pub payload: Vec<u8>,
}

impl SegmentParity {
    pub fn from_file_segment(file_segment: &FileSegment) -> Self {
        Self {
            segment_number: file_segment.segment_number,
            payload: calculate_parity(&file_segment.first_chunk, &file_segment.last_chunk),
        }
    }
}

pub fn calculate_parity(first_chunk: &[u8], last_chunk: &[u8]) -> Vec<u8> {
    first_chunk
        .iter()
        .zip(last_chunk)
        .map(|(a, b)| a ^ b)
        .collect()
}

pub fn return_parity(file_segments: &[FileSegment]) -> Vec<SegmentParity> {
    let mut parity_segments: Vec<SegmentParity> = Vec::new();

    for segment in file_segments {
        parity_segments.push(SegmentParity::from_file_segment(&segment));
    }

    return parity_segments;
}

pub trait HasParity {
    fn generate_parity(self) -> Result<Vec<SegmentParity>, String>;
}
