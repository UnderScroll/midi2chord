mod chord;
mod midi;
mod note;

use midi::MidiKeyboard;
use std::{error::Error, io::stdin};

fn main() {
    let mut _midi_keyboard = MidiKeyboard::new();

    loop {
        let mut user_cmd = String::new();
        stdin()
            .read_line(&mut user_cmd)
            .expect("Failed to read stdin");
        match user_cmd.trim_end() {
            "exit" => break,
            "reconnect" => {
                _midi_keyboard = MidiKeyboard::new();
            }
            _ => println!("Unknown command"),
        }
    }
}
