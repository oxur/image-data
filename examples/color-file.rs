use imgdata::{ColorEntry, ColorFile};

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
    cf.write("examples/example-color-file.ron")?;
    Ok(())
}
