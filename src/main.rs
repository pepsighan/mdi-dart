use heck::MixedCase;
use std::fs::{self, File};
use std::io::BufReader;
use xml::{name::OwnedName, reader::XmlEvent, EventReader};

fn main() {
    let file = File::open("mdi/fonts/materialdesignicons-webfont.svg").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut mdi_content = r#"library mdi;

import 'package:flutter/widgets.dart';

const fontFamily = "Material Design Icons";
const packageName = "mdi";

class Mdi {
"#
    .to_string();

    for item in parser {
        match item {
            Ok(XmlEvent::StartElement {
                name: OwnedName { ref local_name, .. },
                ref attributes,
                ..
            }) if local_name == "glyph" => {
                let icon_name = &attributes
                    .iter()
                    .find(|attr| attr.name.local_name == "glyph-name")
                    .unwrap()
                    .value
                    .to_mixed_case();

                let unicode = attributes
                    .iter()
                    .find(|attr| attr.name.local_name == "unicode")
                    .unwrap()
                    .value
                    .chars()
                    .next()
                    .unwrap() as u32;

                mdi_content.push_str(&format!(
                    "  static const {} = IconData(0x{:X}, fontFamily: fontFamily, fontPackage: packageName);\n",
                    icon_name, unicode,
                ));
            }
            _ => (),
        };
    }

    mdi_content.push_str("}");

    fs::write("mdi/lib/mdi.dart", mdi_content)
        .expect("Could not generate glue code for mdi fonts.");
}
