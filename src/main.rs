#![recursion_limit = "1024"]

#[cfg(test)]
extern crate tempfile;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate json;
extern crate toml;

#[cfg(test)]
mod tests;
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

const API_URI: &str = "https://userstyles.org/api/v1/styles/";
const STYLE_URI: &str = "https://userstyles.org/styles/";
const CONFIG_FILE: &str = "userstyles.toml";

// Represent a single userstyle
struct Style {
    id: i64,
    name: String,
    settings: Vec<Setting>,
}

impl Style {
    fn new(name: String) -> Style {
        Style {
            id: -1,
            name,
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
    let mut styles = {
        // Open readonly config file
        let mut config = File::open(CONFIG_FILE)
            .chain_err(|| "Unable to open the config file.")?;

        // Load config
        load_config(&mut config)?
    };

    // Update settings if they are empty
    for style in &mut styles {
        if style.settings.is_empty() {
            style.settings = get_style_settings(style.id)?;
        }
    }

    // Open config in write mode
    let mut config = File::create(CONFIG_FILE)
        .chain_err(|| "Unable to open the config file.")?;

    // Save the updated settings
    save_style_settings(&mut config, &styles)?;

    // Print all styles to stdout
    for style in &styles {
        println!("{}", get_style(style)?);
    }

    Ok(())
}

// Load a file and parse it into the Style struct
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
        let mut style = Style::new(style_name.to_owned());
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

// Get all the default settings for a userstyle
fn get_style_settings(style_id: i64) -> Result<Vec<Setting>> {
    // Vec for return
    let mut settings_vec = Vec::new();

    // Send request to api
    let uri = &[API_URI, &style_id.to_string()].concat();
    let mut response = reqwest::get(uri).chain_err(|| "Web Request failed.")?;
    let mut response_text = String::new();
    response
        .read_to_string(&mut response_text)
        .chain_err(|| "Invalid utf-8.")?;

    // Convert to json
    let json = json::parse(&response_text).chain_err(|| "Invalid json.")?;

    if !json["not_found"].is_null() || !json["error"].is_null() {
        Err(format!("Style '{}' does not exist.", style_id))?;
    }

    // Get settings
    let settings = &json["style_settings"];

    // Iterate over settings
    for setting in settings.members() {
        // Get install key
        let json_str = setting["install_key"]
            .as_str()
            .ok_or_else(|| "Unable to parse install key.")?;
        let install_key = ["ik-", json_str].concat();

        // Get default value
        let mut default_value = "";
        for setting_option in setting["style_setting_options"].members() {
            if setting_option["default"] == true {
                default_value = setting_option["value"]
                    .as_str()
                    .ok_or_else(|| "Unable to parse default value")?;
            }
        }

        // Create setting struct and add it to vec
        settings_vec.push(Setting::new(
            install_key.to_owned(),
            default_value.to_owned(),
        ));
    }

    // Return all settings
    Ok(settings_vec)
}

// Store all userstyle settings in the config
fn save_style_settings(file: &mut File, styles: &[Style]) -> Result<()> {
    // Create string from styles struct vec
    let mut output = String::new();
    for style in styles {
        output = format!("{}[{}]\nid = {}\n", output, style.name, style.id);
        for setting in &style.settings {
            output.push_str(&[&setting.key, " = \"", &setting.val, "\"\n"].concat());
        }
    }

    // Save styles string to file
    file.write_all(output.as_bytes())
        .chain_err(|| "Unable to update config file")?;

    Ok(())
}

// Get the CSS for a style
fn get_style(style: &Style) -> Result<String> {
    // Construct request url and data
    let mut settings_str = String::new();
    for setting in &style.settings {
        settings_str = format!("{}{}={}&", settings_str, setting.key, setting.val);
    }
    let _ = settings_str.pop();

    let uri = format!("{}{}.css?", STYLE_URI, style.id);

    // Send request
    let client = reqwest::Client::new()
        .chain_err(|| "Unable to find TLS backend.")?;
    let mut response = client
        .post(&uri)
        .chain_err(|| "Unable to create post request.")?
        .body(settings_str)
        .send()
        .chain_err(|| "Unable to send style request.")?;

    // Return response
    let mut response_text = String::new();
    response
        .read_to_string(&mut response_text)
        .chain_err(|| "Response not valid utf-8.")?;
    Ok(response_text)
}
