#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate toml;

mod errors {
    error_chain! {
        foreign_links {
            Toml(::toml::de::Error);
            Io(::std::io::Error);
        }
    }
}

use std::io::{Read, Write};
use std::fs::File;
use toml::Value;
use errors::*;

// Represent a single userstyle
struct Style {
    id: i64,
    settings: Vec<Setting>,
}

impl Style {
    fn new() -> Style {
        Style {
            id: -1,
            settings: Vec::new(),
        }
    }
}

// Properties that can be set for a style
struct Setting {
    key: String,
    val: String,
}

impl Setting {
    fn new(key: String, val: String) -> Setting {
        Setting { key, val }
    }
}


// Execute `run` using errorchain
quick_main!(run);
fn run() -> Result<()> {
    Ok(())
}

fn load_config(file: &mut File) -> Result<Vec<Style>> {
    // Create main struct vec for all styles
    let mut styles = Vec::new();

    // Load file
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .chain_err(|| "Config file is not valid UTF-8.")?;

    // Parse file as table
    let table_val = file_content
        .parse::<Value>()
        .chain_err(|| "Unable to parse config file.")?;
    let table = table_val.as_table().unwrap(); // Always a table

    // Iterate over styles in file
    for (style_name, style_val) in table {
        // Parse val as table
        let style_table = style_val
            .as_table()
            .ok_or_else(|| format!("Unable to parse '{}' as table.", style_name))?;

        // Iterate over prorerties of style
        let mut style = Style::new();
        for (key, val) in style_table {
            if key == "id" {
                // If key is `id` save it as id
                style.id = val.as_integer()
                    .ok_or_else(|| {
                        format!("ID in style '{}' is not an integer.", style_name)
                    })?;
            } else {
                // Otherwise save it as setting
                let val = val.as_str().ok_or_else(|| {
                    format!(
                        "Setting '{}' in style '{}' is not a string.",
                        key,
                        style_name
                    )
                })?;
                let setting = Setting::new(key.to_owned(), val.to_owned());
                style.settings.push(setting);
            }
        }

        // Complain about missing `id`
        if style.id == -1 {
            Err(format!("Missing 'id' field in style {}.", style_name))?;
        }

        // Add style to styles
        styles.push(style);
    }

    // Return the struct
    Ok(styles)
}


// ----------------------------------------------------------------------------

#[cfg(test)]
extern crate tempfile;
#[cfg(test)]
use tempfile::tempfile;
#[cfg(test)]
use std::io::{Seek, SeekFrom};

// Create temp file and seek it so you can read without closing it
// Like this it is not possible to read multiple times witout re-seeking
#[cfg(test)]
fn temp_file(content: &[u8]) -> File {
    let mut file = tempfile().expect("Test setup error.");
    file.write_all(content).expect("Test setup error.");
    file.seek(SeekFrom::Start(0)).expect("Test setup error.");
    file
}

#[test]
#[allow(non_snake_case)]
fn load_config__with_invalid_utf8__is_utf8_error() {
    let mut file = temp_file(&[0, 159, 146, 150]);

    let result = load_config(&mut file);

    if let Err(e) = result {
        assert_eq!(e.description(), "Config file is not valid UTF-8.");
    } else {
        panic!("No error.");
    }
}

#[test]
#[allow(non_snake_case)]
fn load_config__with_broken_file__is_parse_config_error() {
    let mut file = temp_file(b"Broken Toml");

    let result = load_config(&mut file);

    if let Err(e) = result {
        assert_eq!(e.description(), "Unable to parse config file.");
    } else {
        panic!("No error.");
    }
}

#[test]
#[allow(non_snake_case)]
fn load_config__with_style_not_table__is_a_parse_table_error() {
    let mut file = temp_file(b"key = 'val'");

    let result = load_config(&mut file);

    if let Err(e) = result {
        assert_eq!(e.description(), "Unable to parse 'key' as table.");
    } else {
        panic!("No error.");
    }
}

#[test]
#[allow(non_snake_case)]
fn load_config__with_id_string__is_id_not_integer_error() {
    let mut file = temp_file(b"[style]\nid='test'");

    let result = load_config(&mut file);

    if let Err(e) = result {
        assert_eq!(e.description(), "ID in style 'style' is not an integer.");
    } else {
        panic!("No error.");
    }
}

#[test]
#[allow(non_snake_case)]
fn load_config__with_integer_setting__is_setting_not_string_error() {
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
#[allow(non_snake_case)]
fn load_config__with_style_missing_id__is_missing_id_error() {
    let mut file = temp_file(b"[style]\nfoobar='setting'");

    let result = load_config(&mut file);

    if let Err(e) = result {
        assert_eq!(e.description(), "Missing 'id' field in style style.");
    } else {
        panic!("No error.");
    }
}

#[test]
#[allow(non_snake_case)]
fn load_config__with_only_id__is_only_id_struct() {
    let mut file = temp_file(b"[style]\nid=13");

    let result = load_config(&mut file).unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, 13);
    assert!(result[0].settings.is_empty());
}

#[test]
#[allow(non_snake_case)]
fn load_config__with_setting__is_struct_with_setting() {
    let mut file = temp_file(b"[style]\nid=63\nkey='val'");

    let result = load_config(&mut file).unwrap();

    assert_eq!(result[0].settings.len(), 1);
    assert_eq!(result[0].settings[0].key, String::from("key"));
    assert_eq!(result[0].settings[0].val, String::from("val"));
}
