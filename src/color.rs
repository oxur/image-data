use ansi_term::Colour::RGB;
use image::Rgba;
use std::cmp;
use std::i64;

const R: usize = 0;
const G: usize = 1;
const B: usize = 2;
const A: usize = 3;

pub trait Color {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn a(&self) -> u8;
    fn rgb(&self) -> [u8; 3];
    fn rgba(&self) -> [u8; 4];
    fn hex(&self) -> String;
    fn hexa(&self) -> String;
    fn paint(&self, text: &str) -> String;
    fn compare(&self, other: &Rgba<u8>) -> cmp::Ordering;
}

impl Color for Rgba<u8> {
    fn r(&self) -> u8 {
        return self[R];
    }

    fn g(&self) -> u8 {
        return self[G];
    }

    fn b(&self) -> u8 {
        return self[B];
    }

    fn a(&self) -> u8 {
        return self[A];
    }

    fn rgb(&self) -> [u8; 3] {
        return [self[R], self[G], self[B]];
    }

    fn rgba(&self) -> [u8; 4] {
        return [self[R], self[G], self[B], self[A]];
    }

    fn hex(&self) -> String {
        _to_hex(&self.rgb())
    }

    fn hexa(&self) -> String {
        _to_hex(&self.rgba())
    }

    fn paint(&self, text: &str) -> String {
        return RGB(self.r(), self.g(), self.b()).paint(text).to_string();
    }

    fn compare(&self, other: &Rgba<u8>) -> cmp::Ordering {
        self.rgba()
            .iter()
            .zip(other.rgba().iter())
            .map(|(a, b)| a.cmp(b))
            .find(|&ord| ord != cmp::Ordering::Equal)
            .unwrap_or(self.rgba().len().cmp(&other.rgba().len()))
    }
}

pub fn from_hex(hex: String) -> Rgba<u8> {
    Rgba(_from_hex(hex))
}

pub fn from_hexa(hex: String) -> Rgba<u8> {
    Rgba(_from_hexa(hex))
}

fn _to_hex(hex: &[u8]) -> String {
    hex.iter()
        .map(|x| format!("{:02X}", x))
        .fold(String::from(""), |acc, x| format!("{:}{:}", acc, x))
}

fn _from_hex(hex: String) -> [u8; 4] {
    let trimmed_hex = hex.trim_start_matches("0x").trim_start_matches("#");
    let num = i64::from_str_radix(&trimmed_hex, 16).unwrap();
    let r_mask = 0xff0000;
    let g_mask = 0x00ff00;
    let b_mask = 0x0000ff;
    let r = ((num & r_mask) >> 16) as u8;
    let g = ((num & g_mask) >> 8) as u8;
    let b = (num & b_mask) as u8;
    [r, g, b, 255]
}

fn _from_hexa(hex: String) -> [u8; 4] {
    let trimmed_hex = hex.trim_start_matches("0x").trim_start_matches("#");
    let num = i64::from_str_radix(&trimmed_hex, 16).unwrap();
    let r_mask = 0xff000000;
    let g_mask = 0x00ff0000;
    let b_mask = 0x0000ff00;
    let a_mask = 0x000000ff;
    let r = ((num & r_mask) >> 24) as u8;
    let g = ((num & g_mask) >> 16) as u8;
    let b = ((num & b_mask) >> 8) as u8;
    let a = (num & a_mask) as u8;
    [r, g, b, a]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io;

    #[test]
    fn test_components() {
        let img = io::open("src/test-image.png");
        // Red
        let pixel00 = img.get_pixel(0, 0);
        assert_eq!(pixel00.r(), 255);
        assert_eq!(pixel00.g(), 0);
        assert_eq!(pixel00.b(), 0);
        assert_eq!(pixel00.a(), 255);
        // Yellow-green
        let pixel30 = img.get_pixel(3, 0);
        assert_eq!(pixel30.r(), 128);
        assert_eq!(pixel30.g(), 255);
        assert_eq!(pixel30.b(), 0);
        assert_eq!(pixel30.a(), 255);
        // Cyan
        let pixel33 = img.get_pixel(3, 3);
        assert_eq!(pixel33.r(), 0);
        assert_eq!(pixel33.g(), 255);
        assert_eq!(pixel33.b(), 255);
        assert_eq!(pixel33.a(), 255);
        // Purple
        let pixel03 = img.get_pixel(0, 3);
        assert_eq!(pixel03.r(), 128);
        assert_eq!(pixel03.g(), 0);
        assert_eq!(pixel03.b(), 255);
        assert_eq!(pixel03.a(), 255);
        // White
        let pixel11 = img.get_pixel(1, 1);
        assert_eq!(pixel11.r(), 255);
        assert_eq!(pixel11.g(), 255);
        assert_eq!(pixel11.b(), 255);
        assert_eq!(pixel11.a(), 255);
        // Transparent
        let pixel21 = img.get_pixel(2, 1);
        assert_eq!(pixel21.a(), 0);
        let pixel12 = img.get_pixel(1, 2);
        assert_eq!(pixel12.a(), 0);
        // Green
        let pixel31 = *img.get_pixel(3, 1);
        assert_eq!(pixel31.rgb(), [0, 255, 0]);
        assert_eq!(pixel31.rgba(), [0, 255, 0, 255]);
        assert_eq!(pixel31.hex(), "00FF00");
        assert_eq!(pixel31.hexa(), "00FF00FF");
    }

    #[test]
    fn test_private_from_hex() {
        assert_eq!(_from_hex(String::from("336699")), [51, 102, 153, 255]);
        assert_eq!(_from_hex(String::from("0x336699")), [51, 102, 153, 255]);
        assert_eq!(_from_hex(String::from("#336699")), [51, 102, 153, 255]);
    }

    #[test]
    fn test_private_from_hexa() {
        assert_eq!(_from_hexa(String::from("336699ff")), [51, 102, 153, 255]);
        assert_eq!(_from_hexa(String::from("0x336699ff")), [51, 102, 153, 255]);
        assert_eq!(_from_hexa(String::from("#336699ff")), [51, 102, 153, 255]);
    }

    #[test]
    fn test_from_hex() {
        let pixel = from_hex(String::from("336699"));
        assert_eq!(pixel.rgb(), [51, 102, 153]);
        let pixel = from_hex(String::from("0x336699"));
        assert_eq!(pixel.rgb(), [51, 102, 153]);
        let pixel = from_hex(String::from("#336699"));
        assert_eq!(pixel.rgb(), [51, 102, 153]);
    }

    #[test]
    fn test_from_hexa() {
        let pixel = from_hexa(String::from("33669933"));
        assert_eq!(pixel.rgba(), [51, 102, 153, 51]);
        let pixel = from_hexa(String::from("0x33669966"));
        assert_eq!(pixel.rgba(), [51, 102, 153, 102]);
        let pixel = from_hexa(String::from("#33669966"));
        assert_eq!(pixel.rgba(), [51, 102, 153, 102]);
    }
}
