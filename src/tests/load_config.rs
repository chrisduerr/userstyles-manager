#![allow(non_snake_case)]

use tests::utils::temp_file;
use load_config;

#[test]
fn with_invalid_utf8__is_utf8_error() {
    let mut file = temp_file(&[0, 159, 146, 150]);

    let result = load_config(&mut file);

    if let Err(e) = result {
        assert_eq!(e.description(), "stream did not contain valid UTF-8");
    } else {
        panic!("No error.");
    }
}

#[test]
fn with_broken_file__is_parse_config_error() {
    let mut file = temp_file(b"Broken Toml");

    let result = load_config(&mut file);

    if let Err(e) = result {
        assert_eq!(e.description(), "Unable to parse config file.");
    } else {
        panic!("No error.");
    }
}

#[test]
fn with_style_not_table__is_a_parse_table_error() {
    let mut file = temp_file(b"key = 'val'");

    let result = load_config(&mut file);

    if let Err(e) = result {
        assert_eq!(e.description(), "Unable to parse 'key' as table.");
    } else {
        panic!("No error.");
    }
}

#[test]
fn with_id_string__is_id_not_integer_error() {
    let mut file = temp_file(b"[style]\nid='test'");

    let result = load_config(&mut file);

    if let Err(e) = result {
        assert_eq!(e.description(), "ID in style 'style' is not an integer.");
    } else {
        panic!("No error.");
    }
}

#[test]
fn with_integer_setting__is_setting_not_string_error() {
    let mut file = temp_file(b"[style]\nfoobar=13");

    let result = load_config(&mut file);

    if let Err(e) = result {
        assert_eq!(
            e.description(),
            "Setting 'foobar' in style 'style' is not a string."
        );
    } else {
        panic!("No error.");
    }
}

#[test]
fn with_style_missing_id__is_missing_id_error() {
    let mut file = temp_file(b"[style]\nfoobar='setting'");

    let result = load_config(&mut file);

    if let Err(e) = result {
        assert_eq!(e.description(), "Missing 'id' field in style style.");
    } else {
        panic!("No error.");
    }
}

#[test]
fn with_only_id__is_only_id_struct() {
    let mut file = temp_file(b"[style]\nid=13");

    let result = load_config(&mut file).unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, 13);
    assert!(result[0].settings.is_empty());
}

#[test]
fn with_setting__is_struct_with_setting() {
    let mut file = temp_file(b"[style]\nid=63\nkey='val'");

    let result = load_config(&mut file).unwrap();

    assert_eq!(result[0].settings.len(), 1);
    assert_eq!(result[0].settings[0].key, "key");
    assert_eq!(result[0].settings[0].val, "val");
}
