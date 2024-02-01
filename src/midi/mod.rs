use crate::{chord, note::Note, Error};
use midir::{Ignore, MidiInput, MidiInputConnection, MidiInputPort};
use std::{
    fmt::{self, Display},
    io::{self, Write},
    ops::Deref,
};

enum Event {
    KeyOff,
    KeyOn,
    PolyphonicKeyPressure,
    ControlChange,
    ProgramChange,
    ChannelPressure,
    PitchBendChange,
    SystemMessage,
}

pub struct MidiKeyboard(u128);

impl Deref for MidiKeyboard {
    type Target = u128;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MidiKeyboard {
    pub fn new() -> Result<MidiInputConnection<MidiKeyboard>, Box<dyn Error>> {
        let mut midi_in = MidiInput::new("Keyboard")?;
        midi_in.ignore(Ignore::None);

        let midi_port = &Self::select_input_port_until_valid(&midi_in);

        let new: MidiKeyboard = MidiKeyboard(0);

        Ok(midi_in.connect(&midi_port, "midir-in", Self::midi_callback, new)?)
    }

    pub fn select_input_port(midi_in: &MidiInput) -> Result<MidiInputPort, Box<dyn Error>> {
        let ports = midi_in.ports();

        if ports.len() == 0 {
            panic!("No midi port available")
        }
        if ports.len() == 1 {
            println!("Only one port available, selecting the only option");
            println!("Selected port 0 - {}", midi_in.port_name(&ports[0])?);
            return Ok(ports[0].clone());
        }

        println!("Please select midi input port: ");

        //Enumerate midi ports
        for (i, port) in ports.iter().enumerate() {
            let port_name = midi_in.port_name(port)?;
            println!("  {i}) {port_name}");
        }
        io::stdout().flush()?;

        //Reading user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        //Validating user input
        let selection_number = input.trim().parse::<usize>()?;
        if selection_number >= ports.len() {
            return Err(format!(
                "Selected a port out of range (0..{})",
                ports.len() - 1
            ))?;
        }

        let port = ports.get(selection_number).unwrap();
        println!(
            "Selected port {selection_number}: {}",
            midi_in.port_name(port)?
        );

        return Ok(ports.get(selection_number).unwrap().clone());
    }

    pub fn select_input_port_until_valid(midi_in: &MidiInput) -> MidiInputPort {
        loop {
            match Self::select_input_port(midi_in) {
                Ok(port) => return port,
                Err(e) => eprintln!("[ERROR] {e}"),
            }
        }
    }

    fn midi_callback(_stamp: u64, message: &[u8], keyboard: &mut MidiKeyboard) {
        let (event, key_number) = Self::decode_message(message);

        match event {
            Event::KeyOff => keyboard.on_key_released(key_number), //on_key_released(message[1]),
            Event::KeyOn => keyboard.on_key_pressed(key_number),   //on_key_pressed(message[1]),
            Event::PolyphonicKeyPressure => (), //println!("[INFO] Unhandeled message (Polyphonic Key Pressure)"),
            Event::ControlChange => (), //println!("[INFO] Unhandeled message (Control Change)"),
            Event::ProgramChange => (), //println!("[INFO] Unhandeled message (Program Change)"),
            Event::ChannelPressure => (), //println!("[INFO] Unhandeled message (Channel Pressure)"),
            Event::PitchBendChange => (), //println!("[INFO] Unhandeled message (Pitch Bend Change)"),
            Event::SystemMessage => (),   //println!("[INFO] Unhandeled message (System message)"),
            _ => eprintln!("[ERROR] Unknown message type"),
        }
    }

    fn on_key_pressed(&mut self, key_number: u8) {
        self.0 |= (0b1 as u128) << key_number;

        println!("{}", self);

        let note_opts = self.to_notes();
        let mut notes = vec![];
        for (note, _) in note_opts {
            notes.push(note);
        }

        let chords = chord::to_chord(notes);

        if chords.len() > 0 {
            print!("Chords: ");
            for chord in chords {
                print!("\n\t{} [{}]", chord.0, chord.1);
            }
            println!();
        } else {
            println!("No chord found");
        }
    }

    fn on_key_released(&mut self, key_number: u8) {
        self.0 &= !((0b1 as u128) << key_number);

        println!("{}", self);

        let note_opts = self.to_notes();
        let mut notes = vec![];
        for (note, _) in note_opts {
            notes.push(note);
        }

        if notes.len() > 0 {
            let binding = self.to_notes();
            let root = binding.get(0);
            let root = match root {
                Some((note, _)) => note,
                None => panic!("Unknown note"),
            };

            let chords = chord::to_chord(notes);
            if chords.len() > 0 {
                print!("Chords: ");
                for chord in chords {
                    print!("\n\t{} [{}]", chord.0, chord.1);
                }
                println!();
            } else {
                println!("No chord found");
            }
        }
    }

    fn decode_message(message: &[u8]) -> (Event, u8) {
        let (event_index, key_number) = (message[0], message[1]);

        let event = match event_index & 0b1111_0000 {
            0b1000_0000 => Event::KeyOff,
            0b1001_0000 => Event::KeyOn,
            0b1010_0000 => Event::PolyphonicKeyPressure,
            0b1011_0000 => Event::ControlChange,
            0b1100_0000 => Event::ProgramChange,
            0b1101_0000 => Event::ChannelPressure,
            0b1110_0000 => Event::PitchBendChange,
            0b1111_0000 => Event::SystemMessage,
            _ => {
                eprintln!("[ERROR] Unknown event index");
                Event::SystemMessage
            }
        };

        (event, key_number)
    }

    pub fn to_notes(&self) -> Vec<(Note, Option<Note>)> {
        let mut result = vec![];

        let mut bit_mask: u128 = 0b1;

        for i in 1..128 {
            bit_mask = bit_mask << 1;
            if (**self & bit_mask) != 0 {
                match Note::from(i) {
                    Some(n) => result.push((n.0, n.1)),
                    None => (),
                }
            }
        }

        result
    }
}

impl Display for MidiKeyboard {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let notes = self.to_notes();
        if notes.len() == 0 {
            print!("-");
            return Ok(());
        }

        for note in notes {
            print!("{}", note.0);
            match note.1 {
                Some(flat_note) => print!("/{flat_note} "),
                None => print!(" "),
            }
        }
        Ok(())
    }
}
