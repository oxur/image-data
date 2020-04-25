use imgdata::manager;

fn main() {
    let opts = manager::ManagerOptions {
        image_path: String::from("examples/biomes/biomes.png"),
        color_file_path: String::from("examples/biomes/colors.ron"),
    };
    let manager = manager::Manager::new(opts);
    manager.color_names();
    for name in manager.color_names() {
        println!("{}", name);
    }
}
