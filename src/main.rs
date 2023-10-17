#![feature(inherent_associated_types)]
#![feature(const_trait_impl)]

enum CharacterPixels {
    Standard([[bool; 3]; 5]),
    Wide([[bool; 5]; 5]),
}

const fn char_to_pixels(input: char) -> CharacterPixels {
    match input.to_ascii_uppercase() {
        'A' => CharacterPixels::Standard([[true, true, true], [true, false, true], [true, true, true], [true, false, true], [true, false, true]]),
        'B' => CharacterPixels::Standard([[true, true, false], [true, false, true], [true, true, false], [true, false, true], [true, true, false]]),
        'C' => CharacterPixels::Standard([[true, true, true], [true, true, false], [true, true, false], [true, true, false], [true, true, true]]),
        'D' => CharacterPixels::Standard([[true, true, false], [true, false, true], [true, false, true], [true, false, true], [true, true, false]]),
        'E' => CharacterPixels::Standard([[true, true, true], [true, false, false], [true, true, true], [true, false, false], [true, true, true]]),
        'F' => CharacterPixels::Standard([[true, true, true], [true, false, false], [true, true, true], [true, false, false], [true, false, false]]),
        'G' => CharacterPixels::Standard([[true, true, true], [true, false, false], [true, false, true], [true, false, true], [true, true, true]]),
        'H' => CharacterPixels::Standard([[true, false, true], [true, false, true], [true, true, true], [true, false, true], [true, false, true]]),
        'I' => CharacterPixels::Standard([[true, true, true], [false, true, false], [false, true, false], [false, true, false], [true, true, true]]),
        'J' => CharacterPixels::Standard([[true, true, true], [false, true, false], [false, true, false], [false, true, false], [true, true, false]]),
        'K' => CharacterPixels::Standard([[true, false, true], [true, false, true], [true, true, false], [true, false, true], [true, false, true]]),
        'L' => CharacterPixels::Standard([[true, false, true], [true, false, true], [true, false, true], [true, false, true], [true, true, true]]),
        'M' => CharacterPixels::Wide([[true, true, false, true, true], [true, false, true, false, true], [true, false, true, false, true], [true, false, false, false, true], [true, false, false, false, true]]),
        'N' => CharacterPixels::Standard([[true, true, false], [true, false, true], [true, false, true], [true, false, true], [true, false, true]]),
        'O' => CharacterPixels::Standard([[true, true, true], [true, false, true], [true, false, true], [true, false, true], [true, true, true]]),
        'P' => CharacterPixels::Standard([[true, true, true], [true, false, true], [true, true, true], [true, false, false], [true, false, false]]),
        'Q' => CharacterPixels::Standard([[true, true, true], [true, false, true], [true, false, true], [true, true, true], [false, false, true]]),
        'R' => CharacterPixels::Standard([[true, true, true], [true, false, true], [true, true, false], [true, false, true], [true, false, true]]),
        'S' => CharacterPixels::Standard([[true, true, true], [true, false, false], [true, true, true], [false, false, true], [true, true, true]]),
        'T' => CharacterPixels::Standard([[true, true, true], [false, true, false], [false, true, false], [false, true, false], [false, true, false]]),
        'U' => CharacterPixels::Standard([[true, false, true], [true, false, true], [true, false, true], [true, false, true], [true, true, true]]),
        'V' => CharacterPixels::Standard([[true, false, true], [true, false, true], [true, false, true], [true, false, true], [false, true, false]]),
        'W' => CharacterPixels::Wide([[true, false, false, false, true], [true, false, false, false, true], [true, false, true, false, true], [true, false, true, false, true], [true, true, false, true, true]]),
        'X' => CharacterPixels::Standard([[true, false, true], [true, false, true], [false, true, false], [true, false, true], [true, false, true]]),
        'Y' => CharacterPixels::Standard([[true, false, true], [true, false, true], [true, false, true], [false, true, false], [false, true, false]]),
        'Z' => CharacterPixels::Standard([[true, true, true], [false, false, true], [false, true, false], [true, false, false], [true, true, true]]),
        '0' => CharacterPixels::Standard([[true, true, true], [true, false, true], [true, false, true], [true, false, true], [true, true, true]]),
        '1' => CharacterPixels::Standard([[true, true, false], [false, true, false], [false, true, false], [false, true, false], [true, true, true]]),
        '2' => CharacterPixels::Standard([[true, true, true], [false, false, true], [false, true, true], [true, false, false], [true, true, true]]),
        '3' => CharacterPixels::Standard([[true, true, true], [false, false, true], [true, true, true], [false, false, true], [true, true, true]]),
        '4' => CharacterPixels::Standard([[true, false, true], [true, false, true], [true, true, true], [false, false, true], [false, false, true]]),
        '5' => CharacterPixels::Standard([[true, true, true], [true, false, false], [true, true, true], [false, false, true], [true, true, false]]),
        '6' => CharacterPixels::Standard([[true, true, true], [true, true, false], [true, true, true], [true, false, true], [true, true, true]]),
        '7' => CharacterPixels::Standard([[true, true, true], [false, false, true], [false, true, false], [false, true, false], [false, true, false]]),
        '8' => CharacterPixels::Standard([[true, true, true], [true, false, true], [true, true, true], [true, false, true], [true, true, true]]),
        '9' => CharacterPixels::Standard([[true, true, true], [true, false, true], [true, true, true], [false, false, true], [false, false, true]]),
        _ =>   CharacterPixels::Standard([[true, false, true], [false, true, false], [true, false, true], [false, true, false], [true, false, true]]),
    }
}

type DisplayLine = u128;
type DisplayPixels = [DisplayLine; 64];

struct Position {
    x: u8,
    y: u8,
}

trait Displayable {
    fn into_mask(self) -> DisplayPixels;
}

struct StaticText {
    text: &'static str,
    position: &'static Position
}

impl const Displayable for StaticText {
    fn into_mask(self) -> DisplayPixels {
        let mut output: DisplayPixels = [0u128; 64];
        let starting_row: u8 = self.position.y;


        output
    }
}

pub enum GearPosition {
    Park,
    Reverse,
    Neutral,
    First,
    Second,
    Third,
    Fourth,
}

impl GearPosition {
    pub type DisplayedType = char;

    pub const fn as_char(&self) -> char {
        match self {
            Self::Park => 'P',
            Self::Reverse => 'R',
            Self::Neutral => 'N',
            Self::First => '1',
            Self::Second => '2',
            Self::Third => '3',
            Self::Fourth => '4',
        }
    }
}

pub enum DataField {
    Gear(GearPosition),
}

fn main() {
    println!("Hello, world!");
}
