use image::Rgba;

const R_BAND: usize = 0;
const G_BAND: usize = 1;
const B_BAND: usize = 2;
const A_BAND: usize = 3;

pub trait Band {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn a(&self) -> u8;
}

impl Band for Rgba<u8> {
    fn r(&self) -> u8 {
        return self[R_BAND];
    }

    fn g(&self) -> u8 {
        return self[G_BAND];
    }

    fn b(&self) -> u8 {
        return self[B_BAND];
    }

    fn a(&self) -> u8 {
        return self[A_BAND];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io;
    use image::GenericImageView;

    #[test]
    fn test_bands() {
        let img = io::open("src/test-image.png");
        // Red
        let pixel00 = img.get_pixel(0, 0);
        assert_eq!(pixel00.r(), 255);
        assert_eq!(pixel00.g(), 0);
        assert_eq!(pixel00.b(), 0);
        assert_eq!(pixel00.a(), 255);
        // Yellow-green
        let pixel30 = img.get_pixel(3, 0);
        assert_eq!(pixel30[R_BAND], 128);
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
    }
}
