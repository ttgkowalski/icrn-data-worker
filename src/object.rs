use crate::parity::SegmentParity;
use crate::segment::{calculate_file_size_in_bytes, segment_file, FileSegment};
use chrono::Utc;

pub enum DeploySource {
    File(String),
    Object(String),
}

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
    pub fn from_file(
        client: String,
        object_name: String,
        file_path: String,
        load_content: bool,
        generate_parity: bool,
    ) -> Self {
        let content_length: usize = calculate_file_size_in_bytes(&file_path);

        let mut object = Object {
            uuid: format!(
                "{:?}",
                md5::compute(format!("{}.{}.{}", client, object_name, content_length))
            ),
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
        }

        if generate_parity == true {
            let mut parity_segments: Vec<SegmentParity> = Vec::new();

            match &object.segments {
                Some(segments) => {
                    for segment in segments {
                        let parity_segment = SegmentParity::from_file_segment(&segment);
                        parity_segments.push(parity_segment);
                    }
                    object.parity_segments = Some(parity_segments);
                }
                None => eprintln!("Couldn't generate parity blocs without a content loaded"),
            }
        }

        return object;
    }

    pub fn write_segments_to_dir(self, output_dir: String) -> Result<(), String> {
        match self.segments {
            Some(segments) => {
                for segment in segments {
                    let output_name = format!(
                        "{}.{}.segment.{}",
                        self.uuid, self.name, segment.segment_number
                    );

                    match std::fs::write(
                        format!("{output_dir}/{output_name}"),
                        segment.payload.clone(),
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(format!("Failed to write ({:?}) -> {:?}", output_name, err))
                        }
                    }
                }
            }
            None => eprintln!("Couldn't generate parity blocs without a content loaded"),
        }

        Ok(())
    }

    pub fn generate_parity_segments() {}
}
