#![allow(non_snake_case)]

use std::io::Read;
use std::fs::File;
use get_style;
use Setting;
use Style;

#[test]
fn with_allo_style__is_allo_style_css() {
    let style = Style {
        name: String::new(),
        id: 146771,

        settings: vec![
            Setting::new(
                String::from("ik-ACCENTCOLOR"),
                String::from("#f006a2"),
                String::new(),
            ),
        ],
    };
    let mut expected = String::new();
    File::open("./src/tests/allo_output.css")
        .and_then(|mut f| f.read_to_string(&mut expected))
        .expect("Test setup error.");
    expected.pop(); // Delet EOF newline

    let result = get_style(&style).unwrap();

    // I blame microsoft for this
    assert_eq!(expected, result.replace("\r\n", "\n"));
}
