use std::fmt::{write, Display};

use crate::note::Note;

enum Interval {
    PerfectFirst = 0,
    MinorSecond = 1,
    MajorSecond = 2,
    MinorThird = 3,
    MajorThird = 4,
    PerfectFourth = 5,
    DiminishedFifth = 6,
    PerfectFifth = 7,
    MinorSixth = 8,
    MajorSixth = 9,
    MinorSeventh = 10,
    MajorSeventh = 11,
}

impl Interval {
    fn to_semitone(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl TryFrom<u8> for Interval {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Interval::PerfectFirst),
            1 => Ok(Interval::MinorSecond),
            2 => Ok(Interval::MajorSecond),
            3 => Ok(Interval::MinorThird),
            4 => Ok(Interval::MajorThird),
            5 => Ok(Interval::PerfectFourth),
            6 => Ok(Interval::DiminishedFifth),
            7 => Ok(Interval::PerfectFifth),
            8 => Ok(Interval::MinorSixth),
            9 => Ok(Interval::MajorSixth),
            10 => Ok(Interval::MinorSeventh),
            11 => Ok(Interval::MajorSeventh),
            _ => Err(()),
        }
    }
}

pub fn to_chord_from_root(notes: Vec<Note>, root: Note) -> String {
    let mut interval_bitmap: u16 = 0;

    let root_key_number = u8::from(&root);
    for note in &notes {
        let key_number = u8::from(note);
        let semitone = key_number as i8 - root_key_number as i8;
        if semitone != 0 {
            interval_bitmap |= 0b1 << (semitone % 12);
        }
    }

    let mut remaining_intervals = interval_bitmap;

    let mut chord_name: String = String::new();

    chord_name += root.name.to_string().as_str();
    chord_name += root.accidental.to_string().as_str();

    //Power chord
    if notes.len() == 2 {
        if has_interval(interval_bitmap, Interval::PerfectFifth) {
            return chord_name + "5";
        } else {
            return chord_name;
        }
    }

    let mut hasSixth = false;
    //Main structure
    if has_interval(interval_bitmap, Interval::MajorThird) {
        remaining_intervals &= !(0b1 << Interval::MajorThird.to_semitone());
        if has_interval(interval_bitmap, Interval::DiminishedFifth) {
            chord_name += "(♭5)";
            remaining_intervals &= !(0b1 << Interval::DiminishedFifth.to_semitone());
        } else if has_interval(interval_bitmap, Interval::MinorSixth) {
            chord_name += "aug";
            remaining_intervals &= !(0b1 << Interval::MinorSixth.to_semitone());
        } else if has_interval(interval_bitmap, Interval::MajorSixth) && notes.len() < 7 {
            chord_name += "6";
            hasSixth = true;
            remaining_intervals &= !(0b1 << Interval::MajorSixth.to_semitone());
        } else if has_interval(interval_bitmap, Interval::MajorSeventh) {
            chord_name += "maj";
            remaining_intervals &= !(0b1 << Interval::MajorSeventh.to_semitone());
        }
    } else if has_interval(interval_bitmap, Interval::MinorThird) {
        if has_interval(interval_bitmap, Interval::DiminishedFifth) {
            chord_name += "dim";
        } else if has_interval(interval_bitmap, Interval::MajorSixth) && notes.len() < 7 {
            chord_name += "min6";
        } else {
            chord_name += "min";
        }
    } else if has_interval(interval_bitmap, Interval::MinorSecond) //If is sus
        || has_interval(interval_bitmap, Interval::MajorSecond)
        || has_interval(interval_bitmap, Interval::PerfectFourth)
        || has_interval(interval_bitmap, Interval::DiminishedFifth)
    {
        chord_name += "sus";
        if (has_interval(interval_bitmap, Interval::MinorSecond) //If sus4 & sus2
            || has_interval(interval_bitmap, Interval::MajorSecond))
            && (has_interval(interval_bitmap, Interval::PerfectFourth)
                || has_interval(interval_bitmap, Interval::DiminishedFifth))
        {
            if has_interval(interval_bitmap, Interval::MinorSecond) {
                chord_name += "(♭2/";
            } else {
                chord_name += "(2/";
            }
            if has_interval(interval_bitmap, Interval::PerfectFourth) {
                chord_name += "4)";
            } else {
                chord_name += "#4)";
            }
        } else {
            if has_interval(interval_bitmap, Interval::MinorSecond) {
                chord_name += "(♭2)";
            } else if has_interval(interval_bitmap, Interval::MajorSecond) {
                chord_name += "2";
            }
            if has_interval(interval_bitmap, Interval::PerfectFourth) {
                chord_name += "4";
            } else if has_interval(interval_bitmap, Interval::DiminishedFifth) {
                chord_name += "#4";
            }
        }
    }

    //Extensions
    if !hasSixth {
        if has_interval(interval_bitmap, Interval::MinorSeventh)
            || has_interval(interval_bitmap, Interval::MajorSeventh)
        {
            if has_interval(interval_bitmap, Interval::MajorSecond) {
                if has_interval(interval_bitmap, Interval::PerfectFourth) {
                    if has_interval(interval_bitmap, Interval::MajorSixth) {
                        chord_name += "13";
                    } else {
                        chord_name += "11";
                    }
                } else {
                    chord_name += "9";
                }
            } else {
                chord_name += "7"
            }
        }
    }

    chord_name
}

pub fn to_chord(notes: Vec<Note>) -> String {
    "".to_string()
}

fn get_intervals(notes: Vec<Note>, root: Note) -> Vec<Interval> {
    let mut intervals = vec![];
    let root_key_number = u8::from(&root);
    for note in notes {
        let key_number = u8::from(&note);

        let semitone = (key_number - root_key_number) % 12;
        let interval = Interval::try_from(semitone);
        match interval {
            Ok(interval) => intervals.push(interval),
            Err(_) => (),
        }
    }

    intervals
}

fn has_semitone(interval_bitmap: u16, semitone: i8) -> bool {
    if semitone == 0 {
        return true;
    }

    let mask = 0b1 << (semitone % 12);

    interval_bitmap & mask != 0
}

fn has_interval(interval_bitmap: u16, interval: Interval) -> bool {
    has_semitone(interval_bitmap, interval.to_semitone() as i8)
}

/*
 */
