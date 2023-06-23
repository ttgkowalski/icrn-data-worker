use chrono::Utc;
use std::fs;

/// size being u32 means that can contain 4294967295 bytes (4.294967295GB)
/// segments are a bitflag tha indicates how much segments of each size it contains.
/// indexes [0, 1, 2, 3] points to [MBx8, MBx16, MBx32, MBx64]
#[derive(Debug, PartialEq)]
pub struct Object {
    pub uuid: String,
    pub client: String,
    pub name: String,
    pub size: usize,
    pub created_at: i64,
}

impl Object {
    pub fn from_file(client: String, object_name: String, file_path: String) -> Self {
        let content_bytes: Vec<u8> = fs::read(file_path)
            .expect("Should have been able to read the file").to_vec();
        
        let content_lengh: usize = content_bytes.len();

        let uuid = md5::compute(format!("{}.{}.{}", client, object_name, content_lengh));

        return Object {
            uuid: format!("{:?}", uuid),
            client: client,
            name: object_name,
            size: content_lengh,
            created_at: Utc::now().timestamp(),
        };
    }
}
