# image-data

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

[![][logo]][logo-large]

*Process image band values as data, for use in procedural generation projects*

## About

This project aims to provide convenient functions, stuctures, and methods for
working with PNG image pixes as data, with ability to map these pixels' color
values to names.

Projects that would benefit most from this functioality would be those that
want to utilize PNG images as data layers in applications or services.

## Example Usage

There are examples that cover various use cases in the
[examples dir](./examples). For instance:

```rust
use imgdata::{ColorEntry, ColorFile, manager};

fn main() -> std::io::Result<()> {
    let opts = manager::ManagerOptions {
        image_path: String::from("examples/biomes/biomes.png"),
        color_file_path: String::from("examples/biomes/colors.ron"),
    };
    let manager = manager::Manager::new(opts);

    // Extract unique colors from the source image:
    for rgb in manager.unique_rgb_colors() {
        print!("{:?} ", rgb);
    }
    for hex in manager.unique_hex_colors() {
        print!("{} ", hex);
    }

    // Get the list of canonical color names from the color file:
    for name in manager.color_names() {
        println!("{}", name);
    }

    // Get the list of canonical colors from the color file:
    for rgb in manager.colors_rgb() {
        print!("{:?} ", rgb);
    }
    for hex in manager.colors_hex() {
        print!("{:?} ", hex);
    }

    // Create a new color file:
    let e1 = ColorEntry {
        name: String::from("thing1"),
        color: String::from("#123abc"),
    };
    let e2 = ColorEntry {
        name: String::from("thing2"),
        color: String::from("#abc123"),
    };
    let cf = ColorFile {
        entries: vec![e1, e2],
    };
    cf.write("examples/example-color-file.ron")?;
    Ok(())
}
```

## Credits

The project logo is derived from a combination of the "media-flash" icon and
the "image viewer" icon in the
[Ubuntu Yaru icon set](https://github.com/ubuntu/yaru).

## License

Copyright © 2020, Oxur Group

MIT License

<!-- Named page links below: /-->

[logo]: resources/images/logo-250x.png
[logo-large]: resources/images/logo-1000x.png
[build]: https://github.com/oxur/image-data/actions?query=workflow%3Abuild+
[build-badge]: https://github.com/oxur/image-data/workflows/build/badge.svg
[crate]: https://crates.io/crates/image-data
[crate-badge]: https://img.shields.io/crates/v/image-data.svg
[docs]: https://docs.rs/image-data/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/image-data.svg
[tag]: https://github.com/oxur/image-data/tags
