use std::io::{Read, Seek};
use std::path::Path;

use crate::parity::{return_parity, HasParity, SegmentParity};
use crate::segment::{calculate_file_size_in_bytes, segment_file, FileSegment};
use chrono::Utc;

#[derive(Debug)]
// pub struct Object<Parity = WithoutParity> {
pub struct Object {
    pub uuid: String,
    pub client: String,
    pub name: String,
    pub size: usize,
    pub segments: Option<Vec<FileSegment>>,
    pub parity_segments: Option<Vec<SegmentParity>>,
    // pub parity: Parity,
    pub created_at: i64,
}

pub struct WithParity(Vec<SegmentParity>);
pub struct WithoutParity;

impl Object {
    pub fn new_from_path<P: AsRef<Path>>(
        client: String,
        object_name: String,
        path: P,
        load_content: bool,
        generate_parity: bool,
    ) -> Self {
        Self::new(
            client,
            object_name,
            &mut std::fs::File::open(path).expect("Couldn't open file"),
            load_content,
            generate_parity,
        )
    }
    pub fn new<B: Read + Seek>(
        client: String,
        object_name: String,
        buff: &mut B,
        load_content: bool,
        generate_parity: bool,
    ) -> Self {
        let content_length: usize = calculate_file_size_in_bytes(buff);

        let mut object = Self {
            uuid: format!(
                "{:?}",
                md5::compute(format!("{client}.{object_name}.{content_length}"))
            ),
            client,
            name: object_name,
            size: content_length,
            segments: None,
            parity_segments: None,
            created_at: Utc::now().timestamp(),
        };

        if load_content {
            object.segments = Some(segment_file(buff));
        } else {
            object.segments = None;
        }

        if generate_parity {
            object.parity_segments = Some(return_parity(
                object
                    .segments
                    .as_ref()
                    .expect("Failed to return parity segments"),
            ));
        }

        object
    }

    fn write_segment_to_dir(
        &self,
        chunk: &[u8],
        chunk_number: usize,
        segment_number: usize,
        output_dir: impl AsRef<Path>,
    ) -> Result<(), String> {
        let output_name = format!(
            "{}.{}.segment.{}.part.{:?}",
            self.uuid, self.name, segment_number, chunk_number
        );

        match std::fs::write(output_dir.as_ref().join(&output_name), chunk) {
            Ok(_) => {}
            Err(err) => return Err(format!("Failed to write ({:?}) -> {:?}", output_name, err)),
        }

        Ok(())
    }

    pub fn write_segments_to_dir(&self, output_dir: impl AsRef<Path>) -> Result<(), String> {
        for segment in self
            .segments
            .as_ref()
            .expect("Objects segment is not defined")
        {
            self.write_segment_to_dir(
                &segment.first_chunk,
                0,
                segment.segment_number,
                output_dir.as_ref(),
            )?;
            self.write_segment_to_dir(
                &segment.last_chunk,
                1,
                segment.segment_number,
                output_dir.as_ref(),
            )?;
        }

        Ok(())
    }

    pub fn write_parities_to_dir(self, output_dir: &str) -> Result<(), String> {
        for parity_segment in self
            .parity_segments
            .expect("parity segment not found in this object")
        {
            let output_name = format!(
                "{}.{}.segment.{}.parity",
                self.uuid, self.name, parity_segment.segment_number
            );

            match std::fs::write(
                format!("{output_dir}/{output_name}"),
                parity_segment.payload.clone(),
            ) {
                Ok(_) => {}
                Err(err) => return Err(format!("Failed to write ({output_name:?}) -> {err:?}")),
            }
        }

        Ok(())
    }
}

impl HasParity for Object {
    fn generate_parity(&self) -> Option<Vec<SegmentParity>> {
        self.segments.as_ref().map(|segments| {
            segments
                .iter()
                .map(SegmentParity::from_file_segment)
                .collect()
        })
    }
}
