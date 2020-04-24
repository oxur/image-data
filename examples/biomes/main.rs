use imgdata::io;

fn main() {
    let opts = io::ManagerOptions {
        image_path: String::from("examples/biomes/biomes.png"),
        color_file_path: String::from("examples/biomes/colors.ron"),
    };
    let manager = io::Manager::new(opts);
    manager.show_names();
}
