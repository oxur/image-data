use ansi_term::Colour::RGB;
use imgdata::color::Color;
use imgdata::io;

fn main() {
    let img = io::open("examples/biomes/biomes.png");
    let color_file = io::read("examples/biomes/colors.rom");
    let map = color_file.create_lookup();

    for pixel in img.pixels() {
        let c = RGB(pixel.r(), pixel.g(), pixel.b()).paint("██");
        match map.get(&pixel) {
            Some(&name) => print!("{} {} :: ", c, name),
            _ => println!("Color {:?} not found in lookup ...", pixel),
        }
    }
}
