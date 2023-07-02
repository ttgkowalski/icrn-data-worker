use crate::object::Object;

#[test]
fn initialize_object_from_file_test() {
    let dummy_object: Object = Object::new_from_path(
        "kowalskittg@gmail.com".to_string(),
        "64mb-dummy-file.bin".to_string(),
        "input/64mb-dummy-file.bin",
        true,
        true,
    );

    assert_eq!(
        dummy_object.uuid,
        "66313e4c1dc2b78264b87e550c4f70e8".to_string()
    );
    assert_eq!(dummy_object.client, "kowalskittg@gmail.com".to_string());
    assert_eq!(dummy_object.name, "64mb-dummy-file.bin".to_string());
    // assert_eq!(dummy_object.source, "input/64mb-dummy-file.bin".to_string());
    assert_eq!(dummy_object.size, 67108864);
    assert_eq!(dummy_object.segments.unwrap().len(), 8);
}

#[test]
fn write_segments_to_dir_test() {
    let dummy_object: Object = Object::new_from_path(
        "kowalskittg@gmail.com".to_string(),
        "64mb-dummy-file.bin".to_string(),
        "input/64mb-dummy-file.bin",
        true,
        true,
    );
    assert_eq!(dummy_object.write_segments_to_dir("output/"), Ok(()))
}

#[test]
fn write_parities_to_dir_test() {
    let dummy_object: Object = Object::new_from_path(
        "kowalskittg@gmail.com".to_string(),
        "64mb-dummy-file.bin".to_string(),
        "input/64mb-dummy-file.bin",
        true,
        true,
    );
    assert_eq!(
        dummy_object.write_parities_to_dir("output/".to_string()),
        Ok(())
    )
}

