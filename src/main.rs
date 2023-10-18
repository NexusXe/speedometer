#![feature(inherent_associated_types)]
#![allow(incomplete_features)]
#![feature(const_trait_impl)]

type StandardCharacter = i16; // 5 x 3, with sign bit as visibility
type WideCharacter = i32; // 5 x 5, with sign bit as visibility and 6 trailing unused 0s

const A_PIXELS: StandardCharacter = 0b1111101111101101u16 as i16;
const B_PIXELS: StandardCharacter = 0b1110101110101110u16 as i16;
const C_PIXELS: StandardCharacter = 0b1111110110110111u16 as i16;
const D_PIXELS: StandardCharacter = 0b1110101101101110u16 as i16;
const E_PIXELS: StandardCharacter = 0b1111100111100111u16 as i16;
const F_PIXELS: StandardCharacter = 0b1111100111100100u16 as i16;
const G_PIXELS: StandardCharacter = 0b1111100101101111u16 as i16;
const H_PIXELS: StandardCharacter = 0b1101101111101101u16 as i16;
const I_PIXELS: StandardCharacter = 0b1111010010010111u16 as i16;
const J_PIXELS: StandardCharacter = 0b1111010010010110u16 as i16;
const K_PIXELS: StandardCharacter = 0b1101101110101101u16 as i16;
const L_PIXELS: StandardCharacter = 0b1101101101101111u16 as i16;
const M_PIXELS: WideCharacter = 0b11101110101101011000110001000000u32 as i32;
const N_PIXELS: StandardCharacter = 0b1110101101101101u16 as i16;
const O_PIXELS: StandardCharacter = 0b1111101101101111u16 as i16;
const P_PIXELS: StandardCharacter = 0b1111101111100100u16 as i16;
const Q_PIXELS: StandardCharacter = 0b1111101101111001u16 as i16;
const R_PIXELS: StandardCharacter = 0b1111101110101101u16 as i16;
const S_PIXELS: StandardCharacter = 0b1111100111001111u16 as i16;
const T_PIXELS: StandardCharacter = 0b1111010010010010u16 as i16;
const U_PIXELS: StandardCharacter = 0b1101101101101111u16 as i16;
const V_PIXELS: StandardCharacter = 0b1101101101101010u16 as i16;
const W_PIXELS: WideCharacter = 0b11000110001101011010111011000000u32 as i32;
const X_PIXELS: StandardCharacter = 0b1101101010101101u16 as i16;
const Y_PIXELS: StandardCharacter = 0b1101101101010010u16 as i16;
const Z_PIXELS: StandardCharacter = 0b1111001010100111u16 as i16;
const ZERO_PIXELS: StandardCharacter = 0b1111101101101111u16 as i16;
const ONE_PIXELS: StandardCharacter = 0b1110010010010111u16 as i16;
const TWO_PIXELS: StandardCharacter = 0b1111001011100111u16 as i16;
const THREE_PIXELS: StandardCharacter = 0b1111001111001111u16 as i16;
const FOUR_PIXELS: StandardCharacter = 0b1101101111001001u16 as i16;
const FIVE_PIXELS: StandardCharacter = 0b1111100111001110u16 as i16;
const SIX_PIXELS: StandardCharacter = 0b1111110111101111u16 as i16;
const SEVEN_PIXELS: StandardCharacter = 0b1111001010010010u16 as i16;
const EIGHT_PIXELS: StandardCharacter = 0b1111101111101111u16 as i16;
const NINE_PIXELS: StandardCharacter = 0b1111101111001001u16 as i16;
const UNKNOWN_PIXELS: StandardCharacter = 0b1101010101010101u16 as i16;

pub enum DisplayCharacter {
    Standard(StandardCharacter),
    Wide(WideCharacter),
}

impl DisplayCharacter {
    pub const fn new_from_ascii(character: u8) -> Self {
        match character {
            b'A' => return Self::Standard(A_PIXELS),
            b'B' => return Self::Standard(B_PIXELS),
            b'C' => return Self::Standard(C_PIXELS),
            b'D' => return Self::Standard(D_PIXELS),
            b'E' => return Self::Standard(E_PIXELS),
            b'F' => return Self::Standard(F_PIXELS),
            b'G' => return Self::Standard(G_PIXELS),
            b'H' => return Self::Standard(H_PIXELS),
            b'I' => return Self::Standard(I_PIXELS),
            b'J' => return Self::Standard(J_PIXELS),
            b'K' => return Self::Standard(K_PIXELS),
            b'L' => return Self::Standard(L_PIXELS),
            b'M' => return Self::Wide(M_PIXELS),
            b'N' => return Self::Standard(N_PIXELS),
            b'O' => return Self::Standard(O_PIXELS),
            b'P' => return Self::Standard(P_PIXELS),
            b'Q' => return Self::Standard(Q_PIXELS),
            b'R' => return Self::Standard(R_PIXELS),
            b'S' => return Self::Standard(S_PIXELS),
            b'T' => return Self::Standard(T_PIXELS),
            b'U' => return Self::Standard(U_PIXELS),
            b'V' => return Self::Standard(V_PIXELS),
            b'W' => return Self::Wide(W_PIXELS),
            b'X' => return Self::Standard(X_PIXELS),
            b'Y' => return Self::Standard(Y_PIXELS),
            b'Z' => return Self::Standard(Z_PIXELS),
            b'0' => return Self::Standard(ZERO_PIXELS),
            b'1' => return Self::Standard(ONE_PIXELS),
            b'2' => return Self::Standard(TWO_PIXELS),
            b'3' => return Self::Standard(THREE_PIXELS),
            b'4' => return Self::Standard(FOUR_PIXELS),
            b'5' => return Self::Standard(FIVE_PIXELS),
            b'6' => return Self::Standard(SIX_PIXELS),
            b'7' => return Self::Standard(SEVEN_PIXELS),
            b'8' => return Self::Standard(EIGHT_PIXELS),
            b'9' => return Self::Standard(NINE_PIXELS),
            _ => return Self::Standard(UNKNOWN_PIXELS),
        };
    }
}

enum CharacterPixels {
    Standard([[bool; 3]; 5]),
    Wide([[bool; 5]; 5]),
}

impl CharacterPixels {
    pub const fn into_display_pixels(self) -> DisplayPixels {
        const fn bools_to_displaypixels<const T: usize>(data: [[bool; T]; 5]) -> DisplayPixels {
            let mut output: DisplayPixels = [0u128; 64];

            output
        }

        let output: DisplayPixels = match self {
            Self::Standard(data) => bools_to_displaypixels(data),

            Self::Wide(data) => bools_to_displaypixels(data),
        };
        output
    }
}

const fn char_to_pixels(input: char) -> CharacterPixels {
    match input.to_ascii_uppercase() {
        'A' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, true],
            [true, true, true],
            [true, false, true],
            [true, false, true],
        ]),
        'B' => CharacterPixels::Standard([
            [true, true, false],
            [true, false, true],
            [true, true, false],
            [true, false, true],
            [true, true, false],
        ]),
        'C' => CharacterPixels::Standard([
            [true, true, true],
            [true, true, false],
            [true, true, false],
            [true, true, false],
            [true, true, true],
        ]),
        'D' => CharacterPixels::Standard([
            [true, true, false],
            [true, false, true],
            [true, false, true],
            [true, false, true],
            [true, true, false],
        ]),
        'E' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, false],
            [true, true, true],
            [true, false, false],
            [true, true, true],
        ]),
        'F' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, false],
            [true, true, true],
            [true, false, false],
            [true, false, false],
        ]),
        'G' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, false],
            [true, false, true],
            [true, false, true],
            [true, true, true],
        ]),
        'H' => CharacterPixels::Standard([
            [true, false, true],
            [true, false, true],
            [true, true, true],
            [true, false, true],
            [true, false, true],
        ]),
        'I' => CharacterPixels::Standard([
            [true, true, true],
            [false, true, false],
            [false, true, false],
            [false, true, false],
            [true, true, true],
        ]),
        'J' => CharacterPixels::Standard([
            [true, true, true],
            [false, true, false],
            [false, true, false],
            [false, true, false],
            [true, true, false],
        ]),
        'K' => CharacterPixels::Standard([
            [true, false, true],
            [true, false, true],
            [true, true, false],
            [true, false, true],
            [true, false, true],
        ]),
        'L' => CharacterPixels::Standard([
            [true, false, true],
            [true, false, true],
            [true, false, true],
            [true, false, true],
            [true, true, true],
        ]),
        'M' => CharacterPixels::Wide([
            [true, true, false, true, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ]),
        'N' => CharacterPixels::Standard([
            [true, true, false],
            [true, false, true],
            [true, false, true],
            [true, false, true],
            [true, false, true],
        ]),
        'O' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, true],
            [true, false, true],
            [true, false, true],
            [true, true, true],
        ]),
        'P' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, true],
            [true, true, true],
            [true, false, false],
            [true, false, false],
        ]),
        'Q' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, true],
            [true, false, true],
            [true, true, true],
            [false, false, true],
        ]),
        'R' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, true],
            [true, true, false],
            [true, false, true],
            [true, false, true],
        ]),
        'S' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, false],
            [true, true, true],
            [false, false, true],
            [true, true, true],
        ]),
        'T' => CharacterPixels::Standard([
            [true, true, true],
            [false, true, false],
            [false, true, false],
            [false, true, false],
            [false, true, false],
        ]),
        'U' => CharacterPixels::Standard([
            [true, false, true],
            [true, false, true],
            [true, false, true],
            [true, false, true],
            [true, true, true],
        ]),
        'V' => CharacterPixels::Standard([
            [true, false, true],
            [true, false, true],
            [true, false, true],
            [true, false, true],
            [false, true, false],
        ]),
        'W' => CharacterPixels::Wide([
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, true, false, true, true],
        ]),
        'X' => CharacterPixels::Standard([
            [true, false, true],
            [true, false, true],
            [false, true, false],
            [true, false, true],
            [true, false, true],
        ]),
        'Y' => CharacterPixels::Standard([
            [true, false, true],
            [true, false, true],
            [true, false, true],
            [false, true, false],
            [false, true, false],
        ]),
        'Z' => CharacterPixels::Standard([
            [true, true, true],
            [false, false, true],
            [false, true, false],
            [true, false, false],
            [true, true, true],
        ]),
        '0' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, true],
            [true, false, true],
            [true, false, true],
            [true, true, true],
        ]),
        '1' => CharacterPixels::Standard([
            [true, true, false],
            [false, true, false],
            [false, true, false],
            [false, true, false],
            [true, true, true],
        ]),
        '2' => CharacterPixels::Standard([
            [true, true, true],
            [false, false, true],
            [false, true, true],
            [true, false, false],
            [true, true, true],
        ]),
        '3' => CharacterPixels::Standard([
            [true, true, true],
            [false, false, true],
            [true, true, true],
            [false, false, true],
            [true, true, true],
        ]),
        '4' => CharacterPixels::Standard([
            [true, false, true],
            [true, false, true],
            [true, true, true],
            [false, false, true],
            [false, false, true],
        ]),
        '5' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, false],
            [true, true, true],
            [false, false, true],
            [true, true, false],
        ]),
        '6' => CharacterPixels::Standard([
            [true, true, true],
            [true, true, false],
            [true, true, true],
            [true, false, true],
            [true, true, true],
        ]),
        '7' => CharacterPixels::Standard([
            [true, true, true],
            [false, false, true],
            [false, true, false],
            [false, true, false],
            [false, true, false],
        ]),
        '8' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, true],
            [true, true, true],
            [true, false, true],
            [true, true, true],
        ]),
        '9' => CharacterPixels::Standard([
            [true, true, true],
            [true, false, true],
            [true, true, true],
            [false, false, true],
            [false, false, true],
        ]),
        _ => CharacterPixels::Standard([
            [true, false, true],
            [false, true, false],
            [true, false, true],
            [false, true, false],
            [true, false, true],
        ]),
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
    position: &'static Position,
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
        while i < self.len() {
            // character by character
            let mut x: usize = 0;
            let current_character_pixels = char_to_pixels(self.text.chars().nth(i).unwrap());
            while x < output.len() {
                // row by row

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
