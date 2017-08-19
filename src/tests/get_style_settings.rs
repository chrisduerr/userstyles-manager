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
    assert_eq!(result[0].val, "ik-placeholder");
}

#[test]
fn with_github_dark_id__is_not_an_error() {
    get_style_settings(37035).unwrap();
}

#[test]
fn with_dropdown_setting__is_comment_with_options() {
    let result = get_style_settings(107653).unwrap();

    assert_eq!(result[0].comment, " # dropdown: ik-tera ik-black");
}

#[test]
fn with_color_setting__is_comment_with_color() {
    let result = get_style_settings(146771).unwrap();

    assert_eq!(result[0].comment, " # color: '#0F9D58'");
}
