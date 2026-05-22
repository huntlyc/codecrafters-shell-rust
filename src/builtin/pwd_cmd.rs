/// runs the pwd command
pub fn run() {
    let path = std::env::current_dir().unwrap();
    println!("{}", path.display());
}
