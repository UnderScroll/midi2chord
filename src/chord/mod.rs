use std::ops::{Deref, DerefMut};

use crate::note::Note;

struct IntervalFlag(u16);

impl Deref for IntervalFlag {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IntervalFlag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

const PERFECT_FIRST: IntervalFlag = IntervalFlag(0b0000_0000_0001);
const MINOR_SECOND: IntervalFlag = IntervalFlag(0b0000_0000_0010);
const MAJOR_SECOND: IntervalFlag = IntervalFlag(0b0000_0000_0100);
const MINOR_THIRD: IntervalFlag = IntervalFlag(0b0000_0000_1000);
const MAJOR_THIRD: IntervalFlag = IntervalFlag(0b0000_0001_0000);
const PERFECT_FOURTH: IntervalFlag = IntervalFlag(0b0000_0010_0000);
const DIMINISHED_FIFTH: IntervalFlag = IntervalFlag(0b0000_0100_0000);
const PERFECT_FIFTH: IntervalFlag = IntervalFlag(0b0000_1000_0000);
const MINOR_SIXTH: IntervalFlag = IntervalFlag(0b0001_0000_0000);
const MAJOR_SIXTH: IntervalFlag = IntervalFlag(0b0010_0000_0000);
const MINOR_SEVENTH: IntervalFlag = IntervalFlag(0b0100_0000_0000);
const MAJOR_SEVENTH: IntervalFlag = IntervalFlag(0b1000_0000_0000);

macro_rules! has_interval {
    ($state:tt, $flags:expr) => {
        $state & *$flags == *$flags
    };
}

macro_rules! has_interval_exclusive {
    ($state:tt, $flags:expr) => {
        $state ^ *$flags == 0
    };
}

macro_rules! remove_interval {
    ($state:tt, $flags:expr) => {
        $state & !(*$flags)
    };
}

fn is_sus(interval_bitmap: u16) -> bool {
    is_sus2(interval_bitmap) || is_sus4(interval_bitmap)
}

fn is_sus2(interval_bitmap: u16) -> bool {
    if has_interval!(interval_bitmap, MAJOR_THIRD) || has_interval!(interval_bitmap, MINOR_THIRD) {
        return false;
    }
    if (has_interval!(interval_bitmap, MINOR_SEVENTH)
        || has_interval!(interval_bitmap, MAJOR_SEVENTH))
        && has_interval!(interval_bitmap, MAJOR_SECOND)
    {
        return false;
    }

    has_interval!(interval_bitmap, MAJOR_SECOND) || has_interval!(interval_bitmap, MINOR_SECOND)
}

fn is_sus4(interval_bitmap: u16) -> bool {
    if has_interval!(interval_bitmap, PERFECT_FIFTH) {
        return false;
    }
    if (has_interval!(interval_bitmap, MINOR_SEVENTH)
        || has_interval!(interval_bitmap, MAJOR_SEVENTH))
        && has_interval!(interval_bitmap, PERFECT_FOURTH)
    {
        return false;
    }

    has_interval!(interval_bitmap, PERFECT_FOURTH)
        || has_interval!(interval_bitmap, DIMINISHED_FIFTH)
}

fn is_min(interval_bitmap: u16) -> bool {
    has_interval!(interval_bitmap, MINOR_THIRD) && !has_interval!(interval_bitmap, MAJOR_THIRD)
}

fn is_maj(interval_bitmap: u16) -> bool {
    has_interval!(interval_bitmap, MAJOR_THIRD)
}

fn is_dim(interval_bitmap: u16) -> bool {
    has_interval!(interval_bitmap, MINOR_THIRD)
        && has_interval!(interval_bitmap, PERFECT_FOURTH)
        && !has_interval!(interval_bitmap, PERFECT_FIFTH)
}

fn is_aug(interval_bitmap: u16) -> bool {
    has_interval!(interval_bitmap, MINOR_THIRD)
        && has_interval!(interval_bitmap, MINOR_SIXTH)
        && !has_interval!(interval_bitmap, PERFECT_FIFTH)
}

pub fn to_chord(notes: Vec<Note>) -> Vec<(String, u32)> {
    let mut chords = vec![];

    if notes.len() == 0 {
        return chords;
    }

    let first_note = notes.first().unwrap();

    for potential_root in &notes {
        let chord = to_chord_root(&notes, potential_root);
        match chord {
            Some((name, weight)) => {
                if first_note != potential_root {
                    let new_name = format!("{name}/{}{}", first_note.name, first_note.accidental);
                    chords.push((new_name, weight + 3));
                } else {
                    chords.push((name, weight));
                }
            }
            None => (),
        }
    }

    chords.sort_by(|a, b| a.1.cmp(&b.1));

    chords
}

pub fn to_chord_root(notes: &Vec<Note>, root: &Note) -> Option<(String, u32)> {
    if notes.len() == 0 {
        return None;
    }

    let mut interval_bitmap: u16 = 0;

    let root_key_number = u8::from(root);
    for note in notes {
        let key_number = u8::from(note);
        let mut semitone = key_number as i8 - root_key_number as i8;
        while semitone < 0 {
            semitone += 12;
        }
        interval_bitmap |= 0b1 << (semitone % 12);
    }

    interval_bitmap = remove_interval!(interval_bitmap, PERFECT_FIRST);

    if has_interval_exclusive!(interval_bitmap, PERFECT_FIFTH) {
        return Some((format!("{}5", root.name), 0));
    }

    let mut weight = 0;

    //Quality
    let mut is_major = false;
    let mut is_minor = false;
    let mut quality = String::new();
    if is_maj(interval_bitmap) {
        //quality is major but is not displayed
        is_major = true;
        interval_bitmap = remove_interval!(interval_bitmap, MAJOR_THIRD);
        weight += 1;
        if !has_interval!(interval_bitmap, PERFECT_FIFTH) {
            weight += 3;
        }
    } else if is_min(interval_bitmap) {
        is_minor = true;
        quality = "min".to_string();
        interval_bitmap = remove_interval!(interval_bitmap, MINOR_THIRD);
        weight += 1;
        if !has_interval!(interval_bitmap, PERFECT_FIFTH) {
            weight += 3;
        }
    } else if is_dim(interval_bitmap) {
        quality = "dim".to_string();
        interval_bitmap = remove_interval!(interval_bitmap, MINOR_THIRD);
        interval_bitmap = remove_interval!(interval_bitmap, PERFECT_FOURTH);
        weight += 3;
    } else if is_aug(interval_bitmap) {
        quality = "aug".to_string();
        interval_bitmap = remove_interval!(interval_bitmap, MAJOR_THIRD);
        interval_bitmap = remove_interval!(interval_bitmap, MINOR_SIXTH);
        weight += 3;
    }

    //Sus
    let mut sus = String::new();
    if is_sus(interval_bitmap) {
        if is_sus2(interval_bitmap) && !(is_major || is_minor) {
            let sus2;
            if has_interval!(interval_bitmap, MAJOR_SECOND) {
                sus2 = "2";
                interval_bitmap = remove_interval!(interval_bitmap, MAJOR_SECOND);
                weight += 4;
            } else {
                sus2 = "♭2";
                interval_bitmap = remove_interval!(interval_bitmap, MINOR_SECOND);
                weight += 5;
            }
            if is_sus4(interval_bitmap) {
                if has_interval!(interval_bitmap, PERFECT_FOURTH) {
                    sus = format!("sus({sus2}/4)");
                    interval_bitmap = remove_interval!(interval_bitmap, PERFECT_FOURTH);
                    weight += 4;
                } else {
                    sus = format!("sus({sus2}/#4)");
                    interval_bitmap = remove_interval!(interval_bitmap, DIMINISHED_FIFTH);
                    weight += 5;
                }
            } else {
                sus = format!("sus{sus2}");
                if !has_interval!(interval_bitmap, PERFECT_FIFTH) {
                    weight += 3;
                }
            }
        } else if is_sus4(interval_bitmap) && !has_interval!(interval_bitmap, PERFECT_FIFTH) {
            if has_interval!(interval_bitmap, PERFECT_FOURTH) {
                sus = "sus4".to_string();
                interval_bitmap = remove_interval!(interval_bitmap, PERFECT_FOURTH);
                weight += 4;
            } else {
                sus = "sus#4".to_string();
                interval_bitmap = remove_interval!(interval_bitmap, DIMINISHED_FIFTH);
                weight += 5;
            }
        }
        if is_minor {
            quality = format!("min({sus})");
            weight += 5;
        } else {
            quality = sus;
        }
    }

    if has_interval!(interval_bitmap, PERFECT_FIFTH) {
        interval_bitmap = remove_interval!(interval_bitmap, PERFECT_FIFTH);
    }

    //Extensions
    let mut extension = String::new();
    if has_interval!(interval_bitmap, MAJOR_SEVENTH)
        || has_interval!(interval_bitmap, MINOR_SEVENTH)
    {
        if is_min(interval_bitmap) {
            if has_interval!(interval_bitmap, MAJOR_SEVENTH)
                && !has_interval!(interval_bitmap, MINOR_SEVENTH)
            {
                extension += "(maj";
                interval_bitmap = remove_interval!(interval_bitmap, MAJOR_SEVENTH);
                weight += 5;
            } else {
                interval_bitmap = remove_interval!(interval_bitmap, MINOR_SEVENTH);
                weight += 4;
            }
            if has_interval!(interval_bitmap, MAJOR_SECOND) {
                if has_interval!(interval_bitmap, PERFECT_FOURTH) {
                    if has_interval!(interval_bitmap, MAJOR_SIXTH) {
                        extension += "13)";
                        interval_bitmap = remove_interval!(interval_bitmap, MAJOR_SIXTH);
                    } else {
                        extension += "11)";
                        interval_bitmap = remove_interval!(interval_bitmap, PERFECT_FOURTH);
                    }
                } else {
                    extension += "9)";
                    interval_bitmap = remove_interval!(interval_bitmap, MAJOR_SECOND);
                }
            } else {
                extension += "7)";
            }
        } else {
            if has_interval!(interval_bitmap, MAJOR_SEVENTH)
                && !has_interval!(interval_bitmap, MINOR_SEVENTH)
            {
                extension += "maj";
                interval_bitmap = remove_interval!(interval_bitmap, MAJOR_SEVENTH);
                weight += 5;
            } else {
                interval_bitmap = remove_interval!(interval_bitmap, MINOR_SEVENTH);
                weight += 4;
            }
            if has_interval!(interval_bitmap, MAJOR_SECOND) {
                if has_interval!(interval_bitmap, PERFECT_FOURTH) {
                    if has_interval!(interval_bitmap, MAJOR_SIXTH) {
                        extension += "13";
                        interval_bitmap = remove_interval!(interval_bitmap, MAJOR_SIXTH);
                    } else {
                        extension += "11";
                    }
                    interval_bitmap = remove_interval!(interval_bitmap, PERFECT_FOURTH);
                } else {
                    extension += "9";
                }
                interval_bitmap = remove_interval!(interval_bitmap, MAJOR_SECOND);
            } else {
                extension += "7";
            }
        }
    }

    //No matchs -> add
    let mut add = String::new();
    if has_interval!(interval_bitmap, MINOR_SECOND) {
        add += "(♭9)";
        interval_bitmap = remove_interval!(interval_bitmap, MINOR_SECOND);
        weight += 7;
    }
    if has_interval!(interval_bitmap, MAJOR_SECOND) {
        add += "(9)";
        interval_bitmap = remove_interval!(interval_bitmap, MAJOR_SECOND);
        weight += 6;
    }
    if has_interval!(interval_bitmap, MINOR_THIRD) {
        add += "(#9)";
        interval_bitmap = remove_interval!(interval_bitmap, MINOR_THIRD);
        weight += 7;
    }
    if has_interval!(interval_bitmap, PERFECT_FOURTH) {
        add += "(4)";
        interval_bitmap = remove_interval!(interval_bitmap, PERFECT_FOURTH);
        weight += 6;
    }
    if has_interval!(interval_bitmap, DIMINISHED_FIFTH) {
        add += "(♭5)";
        interval_bitmap = remove_interval!(interval_bitmap, DIMINISHED_FIFTH);
        weight += 7;
    }
    if has_interval!(interval_bitmap, MINOR_SIXTH) {
        add += "(♭13)";
        interval_bitmap = remove_interval!(interval_bitmap, MINOR_SIXTH);
        weight += 7;
    }
    if has_interval!(interval_bitmap, MAJOR_SIXTH) {
        add += "(13)";
        interval_bitmap = remove_interval!(interval_bitmap, MAJOR_SIXTH);
        weight += 6;
    }
    if has_interval!(interval_bitmap, MINOR_SEVENTH) {
        add += "(7)";
        interval_bitmap = remove_interval!(interval_bitmap, MINOR_SEVENTH);
        weight += 6;
    }
    if has_interval!(interval_bitmap, MAJOR_SEVENTH) {
        add += "(maj7)";
        interval_bitmap = remove_interval!(interval_bitmap, MAJOR_SEVENTH);
        weight += 7;
    }

    //Return
    let mut chord_name = String::new();
    if interval_bitmap != 0 {
        return None;
    }
    if !quality.is_empty() || is_major {
        chord_name = format!("{}{quality}{extension}{add}", root.name);
    } else {
        return None;
    }

    Some((chord_name, weight))
}
