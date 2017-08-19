#![allow(non_snake_case)]

use std::io::{Read, Seek, SeekFrom};
use tests::utils::temp_file;
use save_style_settings;
use Setting;
use Style;

#[test]
fn with_no_settings_or_id__is_file_with_only_name() {
    let mut file = temp_file(b"");
    let styles = vec![Style::new(String::from("style"))];

    save_style_settings(&mut file, &styles).unwrap();

    file.seek(SeekFrom::Start(0)).expect("Test setup error.");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    assert_eq!(content, "[style]\nid = -1\n");
}

#[test]
fn with_id__is_file_with_id() {
    let mut file = temp_file(b"");
    let styles = vec![
        Style {
            name: String::from("style"),
            id: 15,
            settings: Vec::new(),
        },
    ];

    save_style_settings(&mut file, &styles).unwrap();

    file.seek(SeekFrom::Start(0)).expect("Test setup error.");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    assert_eq!(content, "[style]\nid = 15\n");
}

#[test]
fn with_setting__is_file_with_setting() {
    let mut file = temp_file(b"");
    let styles = vec![
        Style {
            name: String::from("style"),
            id: -1,
            settings: vec![Setting::new(String::from("key"), String::from("val"))],
        },
    ];

    save_style_settings(&mut file, &styles).unwrap();

    file.seek(SeekFrom::Start(0)).expect("Test setup error.");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    assert_eq!(content, "[style]\nid = -1\nkey = \"val\"\n");
}
