#![allow(non_snake_case)]

use get_style_settings;

#[test]
fn with_zero_id__is_not_found_error() {
    let result = get_style_settings(0);

    if let Err(e) = result {
        assert_eq!(e.description(), "Style '0' does not exist.");
    } else {
        panic!("No error!");
    }
}

#[test]
fn with_allo_id__is_single_color_setting() {
    let result = get_style_settings(146771).unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, "ik-ACCENTCOLOR");
    assert_eq!(result[0].val, "#0F9D58");
}

#[test]
fn with_github_dark_id__is_not_an_error() {
    get_style_settings(37035).unwrap();
}
