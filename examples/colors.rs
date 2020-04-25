use imgdata::manager;

fn main() {
    let opts = manager::ManagerOptions {
        image_path: String::from("examples/biomes/biomes.png"),
        color_file_path: String::from("examples/biomes/colors.ron"),
    };
    let manager = manager::Manager::new(opts);
    for rgb in manager.colors_rgb() {
        print!("{:?} ", rgb);
    }
    for hex in manager.colors_hex() {
        print!("{:?} ", hex);
    }
    println!()
}
