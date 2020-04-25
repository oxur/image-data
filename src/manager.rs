use crate::color::Color;
use crate::io;
use image::{Rgba, RgbaImage};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct Manager {
    pub image: RgbaImage,
    pub color_file: io::ColorFile,
    pub lookup: HashMap<Rgba<u8>, String>,
}

pub struct ManagerOptions {
    pub image_path: String,
    pub color_file_path: String,
}

impl Manager {
    pub fn new(opts: ManagerOptions) -> Manager {
        let cf = io::read(&opts.color_file_path[..]);
        return Manager {
            image: io::open(&opts.image_path[..]),
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

    pub fn colors_rgb(&self) -> Vec<[u8; 3]> {
        self.lookup.keys().map(|x| x.rgb()).collect()
    }

    pub fn colors_hex(&self) -> Vec<String> {
        self.lookup
            .keys()
            .map(|x| format!("0x{}", x.hex()))
            .collect()
    }

    pub fn color_names(&self) -> Vec<String> {
        self.lookup.values().map(|x| x.clone()).collect()
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
