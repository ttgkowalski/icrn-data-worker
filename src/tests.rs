use chrono::Utc;
use crate::object::Object;

#[test]
fn initialize_object_from_file() {
    let dummy_object: Object = Object::from_file(
        "kowalskittg@gmail.com".to_string(),
        "64mb-dummy-file.bin".to_string(),
        "input/64mb-dummy-file.bin".to_string(),
    );

    assert_eq!(dummy_object, Object {
        uuid: "66313e4c1dc2b78264b87e550c4f70e8".to_string(),
        client: "kowalskittg@gmail.com".to_string(),
        name: "64mb-dummy-file.bin".to_string(),
        size: 67108864,
        created_at: Utc::now().timestamp(),
    });
}
