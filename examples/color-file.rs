use imgdata::{ColorEntry, ColorFile};
use ron::ser::to_string;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
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
    let s = to_string(&cf).expect("Serialization failed");
    let mut file = File::create("examples/example-color-file.ron")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}
