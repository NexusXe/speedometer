#![feature(inherent_associated_types)]
#![allow(incomplete_features)]
#![feature(const_trait_impl)]

enum CharacterPixels {
    Standard([[bool; 3]; 5]),
    Wide([[bool; 5]; 5]),
}

impl CharacterPixels {
    

    pub const fn into_display_pixels(self) -> DisplayPixels {

        const fn bools_to_displaypixels<const T: usize>(data: [[bool; T]; 5]) -> DisplayPixels {
            todo!();
        }
        
        let mut output: DisplayPixels = match self {
            Self::Standard(data) => {
                bools_to_displaypixels(data)
            },

            Self::Wide(data) => {
                bools_to_displaypixels(data)
            }
        }

        output
    }
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
    fn into_displaypixels(self) -> DisplayPixels;
}

struct StaticText {
    text: &'static str,
    position: &'static Position
}

impl StaticText {
    pub const fn len(&self) -> usize {
        self.text.len()
    }
}

impl const Displayable for StaticText {
    fn into_displaypixels(self) -> DisplayPixels {
        let mut output: DisplayPixels = [0u128; 64];
        let starting_column: u8 = self.position.x;
        let starting_row: u8 = self.position.y;
        let mut current_column: u8 = starting_column;
        let mut i: usize = 0;
        while i < self.len() { // character by character
            let mut x: usize = 0;
            let current_character_pixels = char_to_pixels(self.text.chars().nth(i).unwrap());
            while x < output.len() { // row by row

                x += 1;
            }
            
            i += 1;
        }
        
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
