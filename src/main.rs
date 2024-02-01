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
            "help" => {
                println!(
                    "Commands:
                \n\texit : exits the program
                \n\treconnect : Reconnects the midi keyboard, you can use it to change inputs"
                )
            }
            _ => println!("Unknown command, type help to get available commands"),
        }
    }
}
