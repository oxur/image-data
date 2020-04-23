use image::RgbaImage;

pub fn open(path: &str) -> RgbaImage {
    return image::open(path).unwrap().to_rgba();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let img = open("src/test-image.png");
        assert_eq!(img.dimensions(), (4, 4));
    }
}
