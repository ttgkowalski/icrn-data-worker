use crate::segment::{FileSegment, calculate_file_size_in_bytes, segment_file};
use chrono::Utc;

pub enum DeploySource {
    File(String),
}

#[derive(Debug)]
pub struct Object {
    pub uuid: String,
    pub client: String,
    pub name: String,
    pub source: String,
    pub size: usize,
    pub segments: Vec<FileSegment>,
    pub created_at: i64,
}

impl Object {
    pub fn from_file(client: String, object_name: String, file_path: String) -> Self {
        let content_length: usize = calculate_file_size_in_bytes(&file_path);

        return Object {
            uuid: format!(
                "{:?}",
                md5::compute(format!("{}.{}.{}", client, object_name, content_length))
            ),
            client: client,
            name: object_name,
            source: file_path.clone(),
            size: content_length,
            segments: segment_file(&file_path),
            created_at: Utc::now().timestamp(),
        };
    }

    pub fn commit_to_dir(self, output_dir: String) -> Result<(), String> {
        for segment in self.segments{
            let output_name = format!("{}.{}.{}.segment", self.uuid, self.name, segment.segment_number);
            match std::fs::write(format!("{output_dir}/{output_name}"), segment.payload) {
                Ok(_) => {},
                Err(err) => return Err(format!("Failed to write ({:?}) -> {:?}", output_name, err))
            }
        }
        
        Ok(())
    }

    pub fn commit_to_network(self) {
        todo!()
    }
}
