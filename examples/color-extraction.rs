use imgdata::manager;

fn main() {
    let opts = manager::ManagerOptions {
        image_path: String::from("examples/biomes/biomes.png"),
        color_file_path: String::from("examples/biomes/colors.ron"),
    };
    let manager = manager::Manager::new(opts);
    for rgb in manager.unique_rgb_colors() {
        print!("{:?} ", rgb);
    }
    for hex in manager.unique_hex_colors() {
        print!("{} ", hex);
    }
    println!()
}
