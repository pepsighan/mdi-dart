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

const _fontFamily = "Material Design Icons";
const _packageName = "mdi";

/// A const wrapper for [IconData].
class MdiIconData extends IconData {
  const MdiIconData(int codePoint)
      : super(codePoint, fontFamily: _fontFamily, fontPackage: _packageName);
}

/// Mdi is a collection of icons provided by 
/// [Material Design Icons](https://materialdesignicons.com/).
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
                let mut icon_name = attributes
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

                let is_keyword =
                    ["null", "switch", "sync", "factory"].contains(&icon_name.as_str());
                if is_keyword {
                    icon_name.push_str("Icon");
                }

                mdi_content.push_str(&format!(
                    "  static const {} = MdiIconData(0x{:X});\n",
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
