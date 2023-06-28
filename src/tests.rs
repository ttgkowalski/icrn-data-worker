use crate::object::Object;
use chrono::Utc;

#[test]
fn initialize_object_from_file_test() {
    let dummy_object: Object = Object::from_file(
        "kowalskittg@gmail.com".to_string(),
        "64mb-dummy-file.bin".to_string(),
        "input/64mb-dummy-file.bin".to_string(),
    );

    assert_eq!(
        dummy_object.uuid,
        "66313e4c1dc2b78264b87e550c4f70e8".to_string()
    );
    assert_eq!(dummy_object.client, "kowalskittg@gmail.com".to_string());
    assert_eq!(dummy_object.name, "64mb-dummy-file.bin".to_string());
    assert_eq!(dummy_object.source, "input/64mb-dummy-file.bin".to_string());
    assert_eq!(dummy_object.size, 67108864);
    assert_eq!(dummy_object.segments.len(), 8);
    assert_eq!(dummy_object.created_at, Utc::now().timestamp());
}

#[test]
fn write_segments_to_dir_test() {
    let dummy_object: Object = Object::from_file(
        "kowalskittg@gmail.com".to_string(),
        "64mb-dummy-file.bin".to_string(),
        "input/64mb-dummy-file.bin".to_string(),
    );
    assert_eq!(
        dummy_object.write_segments_to_dir("output/".to_string()),
        Ok(())
    )
}
