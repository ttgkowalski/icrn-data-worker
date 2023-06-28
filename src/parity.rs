use crate::segment::FileSegment;

#[derive(Debug)]
pub struct SegmentParity {
    pub segment_number: u32,
    pub payload: Vec<u8>,
}

impl SegmentParity {
    pub fn from_file_segment(file_segment: &FileSegment) -> SegmentParity {
        let segment_payload = &file_segment.payload;
        let segment_length = &segment_payload.len();

        let segment_first_chunk = &segment_payload[0..segment_length / 2];
        let segment_last_chunk = &segment_payload[segment_length / 2..];

        return SegmentParity {
            segment_number: file_segment.segment_number,
            payload: calculate_parity(segment_first_chunk, segment_last_chunk),
        };
    }
}


pub fn calculate_parity(first_chunk: &[u8], last_chunk: &[u8]) -> Vec<u8> {
    let mut parity_chunk: Vec<u8> = Vec::new();
    
    for (i, byte) in first_chunk.iter().enumerate() {
        parity_chunk.push(byte ^ last_chunk[i]);
    }
    
    return parity_chunk;
}

pub fn return_parity(file_segments: &Vec<FileSegment>) -> Vec<SegmentParity> {
    let mut parity_segments: Vec<SegmentParity> = Vec::new();

    for segment in file_segments {
        parity_segments.push(SegmentParity::from_file_segment(&segment));
    }


    return parity_segments
}

pub trait HasParity {
    fn generate_parity(self) -> Result<Vec<SegmentParity>, String>;
}
