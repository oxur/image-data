use crate::color;
use image::{Rgba, RgbaImage};
use ron;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ColorEntry {
    pub name: String,
    // Color is a hexidecimal 3-component, RGB color with no alpha
    pub color: String,
}

pub fn open(path: &str) -> RgbaImage {
    return image::open(path).unwrap().to_rgba();
}

pub fn read(path: &str) -> ColorFile {
    let f = File::open(path).expect("Failed opening file");
    let color_file: ColorFile = match ron::de::from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            panic!("Failed to load color-file: {}", e);
        }
    };
    return color_file;
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ColorFile {
    pub entries: Vec<ColorEntry>,
}

impl ColorFile {
    pub fn new(colors: Vec<String>) -> ColorFile {
        ColorFile {
            entries: colors
                .iter()
                .map(|x| ColorEntry {
                    name: String::from(""),
                    color: x.clone(),
                })
                .collect(),
        }
    }
    pub fn create_lookup(&self) -> HashMap<Rgba<u8>, String> {
        let mut lookup = HashMap::new();
        for color_entry in self.entries.iter() {
            let c = color::from_hex(color_entry.color.clone());
            lookup.insert(c, color_entry.name.clone());
        }
        return lookup;
    }

    pub fn write(&self, path: &str) -> io::Result<()> {
        let serialized = ron::ser::to_string(self).expect("Serialization failed");
        match File::create(path) {
            Ok(mut file) => {
                file.write_all(serialized.as_bytes())?;
                Ok(())
            }
            _ => panic!("Could not write color file!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let img = open("src/test-image.png");
        assert_eq!(img.dimensions(), (4, 4));
    }

    #[test]
    fn test_read() {
        let color_file = read("src/test-color-file.ron");

        assert_eq!(color_file.entries[0].name, "thing1");
        assert_eq!(color_file.entries[0].color, "#123abc");
        assert_eq!(color_file.entries[1].name, "thing2");
        assert_eq!(color_file.entries[1].color, "#abc123");
    }

    #[test]
    fn test_new_color_file() {
        let color_file = ColorFile::new(vec![String::from("#abc123"), String::from("#123abc")]);
        assert_eq!(color_file.entries[0].name, "");
        assert_eq!(color_file.entries[0].color, "#abc123");
        assert_eq!(color_file.entries[1].name, "");
        assert_eq!(color_file.entries[1].color, "#123abc");
    }

    #[test]
    fn test_create_lookup() {
        let color_file = read("src/test-color-file.ron");
        let lu = color_file.create_lookup();
        let c1 = color::from_hex(String::from("#123abc").clone());
        let c2 = color::from_hex(String::from("#abc123").clone());

        match lu.get(&c1) {
            Some(name) => assert_eq!(name.clone(), "thing1"),
            _ => panic!("WAT"),
        }

        match lu.get(&c2) {
            Some(name) => assert_eq!(name.clone(), "thing2"),
            _ => panic!("WUT"),
        }
    }
}
