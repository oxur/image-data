use crate::color::{self, Color};
use image::{Rgba, RgbaImage};
use ron;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};

pub fn open(path: &str) -> RgbaImage {
    return image::open(path).unwrap().to_rgba();
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ColorEntry {
    pub name: String,
    // Color is a hexidecimal 3-component, RGB color with no alpha
    pub color: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ColorFile {
    pub entries: Vec<ColorEntry>,
}

impl ColorFile {
    pub fn create_lookup(&self) -> HashMap<Rgba<u8>, String> {
        let mut lookup = HashMap::new();
        for color_entry in self.entries.iter() {
            let c = color::from_hex(color_entry.color.clone());
            lookup.insert(c, color_entry.name.clone());
        }
        return lookup;
    }
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

pub struct Manager {
    pub image: RgbaImage,
    pub color_file: ColorFile,
    pub lookup: HashMap<Rgba<u8>, String>,
}

pub struct ManagerOptions {
    pub image_path: String,
    pub color_file_path: String,
}

impl Manager {
    pub fn new(opts: ManagerOptions) -> Manager {
        let cf = read(&opts.color_file_path[..]);
        return Manager {
            image: open(&opts.image_path[..]),
            color_file: cf.clone(),
            lookup: cf.create_lookup(),
        };
    }

    pub fn get(&self, x: u32, y: u32) -> PixelData {
        let pixel = self.image.get_pixel(x, y);
        match self.lookup.get(&pixel) {
            Some(name) => {
                return PixelData {
                    x: x,
                    y: y,
                    color: pixel.clone(),
                    color_name: name.clone(),
                }
            }
            _ => {
                return PixelData {
                    x: x,
                    y: y,
                    color: pixel.clone(),
                    color_name: String::from("UNKNOWN"),
                }
            }
        }
    }

    pub fn hash(&self, x: u32, y: u32) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.get(x, y).hash(&mut hasher);
        return hasher.finish();
    }

    pub fn show_names(&self) {
        for pixel in self.image.pixels() {
            let c = pixel.paint("██");
            match self.lookup.get(&pixel) {
                Some(name) => print!("{} {} :: ", c, name),
                _ => println!("Color {:?} not found in lookup ...", pixel),
            }
        }
        println!();
    }
}

pub struct PixelData {
    pub x: u32,
    pub y: u32,
    pub color: Rgba<u8>,
    pub color_name: String,
}

impl Hash for PixelData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.color.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;

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

    #[test]
    fn test_manager_get() {
        let opts = ManagerOptions {
            image_path: String::from("src/test-image.png"),
            color_file_path: String::from("src/test-color-file.ron"),
        };
        let manager = Manager::new(opts);
        let pixel_data = manager.get(0, 0);
        assert_eq!(pixel_data.x, 0);
        assert_eq!(pixel_data.y, 0);
        assert_eq!(pixel_data.color.rgba(), [255, 0, 0, 255]);
        assert_eq!(pixel_data.color_name, "red");
    }

    #[test]
    fn test_manager_hash() {
        let opts = ManagerOptions {
            image_path: String::from("src/test-image.png"),
            color_file_path: String::from("src/test-color-file.ron"),
        };
        let manager = Manager::new(opts);
        assert_eq!(manager.hash(0, 0), 14349191598533205778);
    }
}
