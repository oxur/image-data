use image::DynamicImage;

pub fn open(path: &str) -> DynamicImage {
    return image::open(path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;

    #[test]
    fn test_open() {
        let img = open("src/test-image.png");
        assert_eq!(img.dimensions(), (4, 4));
    }
}
