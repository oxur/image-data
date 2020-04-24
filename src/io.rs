use image::RgbaImage;
use ron;
use serde::{Deserialize, Serialize};
use std::fs::File;

pub fn open(path: &str) -> RgbaImage {
    return image::open(path).unwrap().to_rgba();
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ColorEntry {
    pub name: String,
    pub hex: Option<String>,
    pub hexa: Option<String>,
    pub rgb: Option<(u8, u8, u8)>,
    pub rgba: Option<(u8, u8, u8, u8)>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ColorFile {
    pub entries: Vec<ColorEntry>,
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
        assert_eq!(color_file.entries[0].hex.as_ref().unwrap(), "#123abc");
        assert_eq!(color_file.entries[1].name, "thing2");
        assert_eq!(color_file.entries[1].rgb.unwrap(), (171, 193, 35));
    }
}
