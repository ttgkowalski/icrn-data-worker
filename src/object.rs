use crate::parity::{SegmentParity, HasParity, return_parity};
use crate::segment::{calculate_file_size_in_bytes, segment_file, FileSegment};
use chrono::Utc;

#[derive(Debug)]
pub struct Object {
    pub uuid: String,
    pub client: String,
    pub name: String,
    pub source: String,
    pub size: usize,
    pub segments: Option<Vec<FileSegment>>,
    pub parity_segments: Option<Vec<SegmentParity>>,
    pub created_at: i64,
}

impl Object {
    pub fn from_file(client: String, object_name: String, file_path: String, load_content: bool, generate_parity: bool) -> Self {
        let content_length: usize = calculate_file_size_in_bytes(&file_path);

        let mut object = Object {
            uuid: format!("{:?}", md5::compute(format!("{}.{}.{}", client, object_name, content_length))),
            client,
            name: object_name,
            source: file_path.clone(),
            size: content_length,
            segments: None,
            parity_segments: None,
            created_at: Utc::now().timestamp(),
        };

        if load_content == true {
            object.segments = Some(segment_file(&file_path));
        } else {
            object.segments = None;
        }

        if generate_parity == true {
            object.parity_segments = Some(return_parity(object.segments.as_ref().unwrap()))
        }

        return object;
    }

    pub fn write_segments_to_dir(self, output_dir: String) -> Result<(), String> {
        for segment in self.segments.unwrap() {
            let output_name = format!("{}.{}.segment.{}",self.uuid, self.name, segment.segment_number);

            match std::fs::write(format!("{output_dir}/{output_name}"), segment.payload.clone()) {
                Ok(_) => {}
                Err(err) => return Err(format!("Failed to write ({:?}) -> {:?}", output_name, err))
            }
        }

        Ok(())
    }

    pub fn write_parities_to_dir(self, output_dir: String) -> Result<(), String> {
        for parity_segment in self.parity_segments.unwrap() {
            let output_name = format!("{}.{}.segment.{}.parity",self.uuid, self.name, parity_segment.segment_number);

            match std::fs::write(format!("{output_dir}/{output_name}"), parity_segment.payload.clone()) {
                Ok(_) => {}
                Err(err) => return Err(format!("Failed to write ({:?}) -> {:?}", output_name, err))
            }
        }

        Ok(())
    }
}

impl HasParity for Object {
    fn generate_parity(self) -> Result<Vec<SegmentParity>, String> {
        let mut parity_segments: Vec<SegmentParity> = Vec::new();

        match self.segments {
            Some(segments) => {
                for segment in segments {
                    parity_segments.push(SegmentParity::from_file_segment(&segment));
                }
            }
            None => return Err(format!("Object \"{}\" has no FileSegment to generate parity", {self.uuid}))
        }

        return Ok(parity_segments)
    }
}
