use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Name {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Accidental {
    Flat = -1,
    Natural = 0,
    Sharp = 1,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Note {
    pub name: Name,
    pub accidental: Accidental,
    pub octave: u8,
}

impl Name {
    fn discriminant(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl Accidental {
    fn discriminant(&self) -> i8 {
        unsafe { *<*const _>::from(self).cast::<i8>() }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = ('A' as u8 + self.discriminant()) as char;
        write!(f, "{name}")
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Accidental::Sharp => write!(f, "#"),
            Accidental::Flat => write!(f, "♭"),
            _ => fmt::Result::Ok(()),
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.name, self.accidental, self.octave)
    }
}

impl Note {
    pub fn new(name: Name, accidental: Accidental, octave: u8) -> Note {
        Note {
            name: name,
            accidental: accidental,
            octave: octave,
        }
    }

    pub fn from(value: u8) -> Option<(Note, Option<Note>)> {
        if value < 21 {
            return None;
        }

        let offseted_value: u8 = value - 21;
        let octave: u8 = ((offseted_value as i8 - 2) as f64 / 12.0).ceil() as u8;
        let number: u8 = offseted_value % 12;

        let name_map_sharp: [Name; 12] = [
            Name::A,
            Name::A,
            Name::B,
            Name::C,
            Name::C,
            Name::D,
            Name::D,
            Name::E,
            Name::F,
            Name::F,
            Name::G,
            Name::G,
        ];
        let name_map_flat: [Name; 12] = [
            Name::A,
            Name::B,
            Name::B,
            Name::C,
            Name::D,
            Name::D,
            Name::E,
            Name::E,
            Name::F,
            Name::G,
            Name::G,
            Name::A,
        ];

        let accitental_bit_mask: u16 = 0b0000_1010_0101_0010;
        let is_natural: bool = accitental_bit_mask & (0b1 << number) == 0;

        if is_natural {
            Some((
                Note {
                    name: name_map_sharp[number as usize],
                    accidental: Accidental::Natural,
                    octave: octave,
                },
                None,
            ))
        } else {
            Some((
                Note {
                    name: name_map_sharp[number as usize],
                    accidental: Accidental::Sharp,
                    octave: octave,
                },
                Some(Note {
                    name: name_map_flat[number as usize],
                    accidental: Accidental::Flat,
                    octave: octave,
                }),
            ))
        }
    }

    pub fn interval_note(note_a: Note, note_b: Note) -> i8 {
        (u8::from(&note_b) - u8::from(&note_a)) as i8
    }

    pub fn interval_midi_number(number_a: u8, number_b: u8) -> i8 {
        (number_b - number_a) as i8
    }
}

impl From<&Note> for u8 {
    fn from(value: &Note) -> Self {
        let name_to_midi_number_map: [u8; 7] = [9, 11, 0, 2, 4, 5, 7];

        let number_in_ocatve: u8 = name_to_midi_number_map[value.name.discriminant() as usize];
        let number: u8 = ((12 + number_in_ocatve + 12 * value.octave) as i8
            + value.accidental.discriminant()) as u8;
        number
    }
}
