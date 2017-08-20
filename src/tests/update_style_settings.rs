#![allow(non_snake_case)]

use update_style_settings;
use Setting;
use Style;

fn test_style() -> Style {
    Style {
        id: 0,
        name: String::new(),
        settings: Vec::new(),
    }
}

#[test]
fn with_zero_id__is_not_found_error() {
    let mut style = test_style();

    let result = update_style_settings(&mut style);

    if let Err(e) = result {
        assert_eq!(e.description(), "Style '0' does not exist.");
    } else {
        panic!("No error!");
    }
}

#[test]
fn with_allo_id__is_single_color_setting() {
    let mut style = test_style();
    style.id = 146771;

    update_style_settings(&mut style).unwrap();

    assert_eq!(style.settings[0].key, "ik-ACCENTCOLOR");
    assert_eq!(style.settings[0].val, "ik-placeholder");
}

#[test]
fn with_github_dark_id__is_not_an_error() {
    let mut style = test_style();
    style.id = 37035;

    update_style_settings(&mut style).unwrap();
}

#[test]
fn with_dropdown_setting__is_comment_with_options() {
    let mut style = test_style();
    style.id = 107653;

    update_style_settings(&mut style).unwrap();

    assert_eq!(style.settings[0].comment, " # dropdown: ik-tera ik-black");
}

#[test]
fn with_color_setting__is_comment_with_color() {
    let mut style = test_style();
    style.id = 146771;

    update_style_settings(&mut style).unwrap();

    assert_eq!(style.settings[0].comment, " # color: '#0F9D58'");
}

#[test]
fn with_allo_settings_without_comment__is_allo_settings_unchanged_with_comment() {
    let mut style = test_style();
    style.id = 146771;
    style.settings = vec![
        Setting::new(
            String::from("ik-ACCENTCOLOR"),
            String::from("#f006a2"),
            String::new(),
        ),
    ];

    update_style_settings(&mut style).unwrap();

    assert_eq!(style.name, "");
    assert_eq!(style.id, 146771);
    assert_eq!(style.settings[0].key, "ik-ACCENTCOLOR");
    assert_eq!(style.settings[0].val, "#f006a2");
    assert_eq!(style.settings[0].comment, " # color: '#0F9D58'");
}

#[test]
fn with_nonexistant_setting__is_without_settings() {
    let mut style = test_style();
    style.id = 146775;
    style.settings = vec![
        Setting::new(String::from("noexist"), String::new(), String::new()),
    ];

    update_style_settings(&mut style).unwrap();

    assert_eq!(style.settings.len(), 0);
}
