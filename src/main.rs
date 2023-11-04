#![feature(inherent_associated_types)]
#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(const_trait_impl)]
#![feature(ascii_char)]
#![feature(const_mut_refs)]
#![feature(const_option)]

type StandardCharacter = u16; // 5 x 3, with sign bit as visibility
type WideCharacter = u32; // 5 x 5, with sign bit as visibility and 6 trailing unused 0s
type SmallCharacter = u8; // 4x2

type DisplayLine = u128;
type DisplayPixels = [DisplayLine; 64];

#[const_trait]
trait Displayable {
    fn into_displaypixels(self) -> DisplayPixels;
}

const CHAR_LENGTH: usize = 5;
const CHAR_WIDTH: usize = 3;
const WIDECHAR_LENGTH: usize = 5;
const WIDECHAR_WIDTH: usize = 5;
const SMALLCHAR_LENGTH: usize = 3;
const SMALLCHAR_WIDTH: usize = 2;

const SPACE_PIXELS: StandardCharacter = 0b1000000000000000u16;
const EXCL_PIXELS: StandardCharacter = 0b1010010010000010u16;
const QUOTE_PIXELS: StandardCharacter = 0b1101101000000000u16;
const HASH_PIXELS: WideCharacter = 0b10101011111010101111101010000000u32;
const DOLLAR_PIXELS: WideCharacter = 0b10111010100011100010101110000000u32;

const AT_PIXELS: WideCharacter = 0b11111110001101111011011111000000u32;
const PERCENT_PIXELS: StandardCharacter = 0b1101001010100101u16;
const CARET_PIXELS: StandardCharacter = 0b1010101101000000u16;
const AMPERSAND_PIXELS: WideCharacter = 0b10110010000011011001001101000000u32;
const ASTERISK_PIXELS: StandardCharacter = 0b1101010101000000u16;
const LEFT_PAREN_PIXELS: StandardCharacter = 0b1001010010010001u16;
const RIGHT_PAREN_PIXELS: StandardCharacter = 0b1100010010010100u16;
const MINUS_PIXELS: StandardCharacter = 0b1000000111000000u16;
// TODO: finish ASCII
const A_PIXELS: StandardCharacter = 0b1111101111101101u16;
const B_PIXELS: StandardCharacter = 0b1110101110101110u16;
const C_PIXELS: StandardCharacter = 0b1111100100100111u16;
const D_PIXELS: StandardCharacter = 0b1110101101101110u16;
const E_PIXELS: StandardCharacter = 0b1111100111100111u16;
const F_PIXELS: StandardCharacter = 0b1111100111100100u16;
const G_PIXELS: StandardCharacter = 0b1111100101101111u16;
const H_PIXELS: StandardCharacter = 0b1101101111101101u16;
const I_PIXELS: StandardCharacter = 0b1111010010010111u16;
const J_PIXELS: StandardCharacter = 0b1111010010010110u16;
const K_PIXELS: StandardCharacter = 0b1101101110101101u16;
const L_PIXELS: StandardCharacter = 0b1100100100100111u16;
const M_PIXELS: WideCharacter = 0b11101110101101011000110001000000u32;
const N_PIXELS: StandardCharacter = 0b1110101101101101u16;
const O_PIXELS: StandardCharacter = 0b1111101101101111u16;
const P_PIXELS: StandardCharacter = 0b1111101111100100u16;
const Q_PIXELS: StandardCharacter = 0b1111101101111001u16;
const R_PIXELS: StandardCharacter = 0b1111101110101101u16;
const S_PIXELS: StandardCharacter = 0b1111100111001111u16;
const T_PIXELS: StandardCharacter = 0b1111010010010010u16;
const U_PIXELS: StandardCharacter = 0b1101101101101111u16;
const V_PIXELS: StandardCharacter = 0b1101101101101010u16;
const W_PIXELS: WideCharacter = 0b11000110001101011010111011000000u32;
const X_PIXELS: StandardCharacter = 0b1101101010101101u16;
const Y_PIXELS: StandardCharacter = 0b1101101101010010u16;
const Z_PIXELS: StandardCharacter = 0b1111001010100111u16;
const ZERO_PIXELS: StandardCharacter = 0b1111101101101111u16;
const ONE_PIXELS: StandardCharacter = 0b1110010010010111u16;
const TWO_PIXELS: StandardCharacter = 0b1111001011100111u16;
const THREE_PIXELS: StandardCharacter = 0b1111001111001111u16;
const FOUR_PIXELS: StandardCharacter = 0b1101101111001001u16;
const FIVE_PIXELS: StandardCharacter = 0b1111100111001110u16;
const SIX_PIXELS: StandardCharacter = 0b1111100111101111u16;
const SEVEN_PIXELS: StandardCharacter = 0b1111001010010010u16;
const EIGHT_PIXELS: StandardCharacter = 0b1111101111101111u16;
const NINE_PIXELS: StandardCharacter = 0b1111101111001001u16;
const SLASH_PIXELS: StandardCharacter = 0b0001001010100100u16;
const DOT_PIXELS: SmallCharacter = 0b01100110u8;
const COMMA_PIXELS: SmallCharacter = 0b01100010u8;
const UNKNOWN_PIXELS: StandardCharacter = 0b1101010101010101u16;

pub enum DisplayCharacter {
    Standard(StandardCharacter),
    Wide(WideCharacter),
    Small(SmallCharacter),
}

impl DisplayCharacter {
    pub const fn new_from_ascii(character: u8) -> Self {
        match character.to_ascii_uppercase() {
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
            b'/' => return Self::Standard(SLASH_PIXELS),
            b'.' => return Self::Small(DOT_PIXELS),
            b',' => return Self::Small(COMMA_PIXELS),
            b' ' => return Self::Standard(SPACE_PIXELS),
            b'!' => return Self::Standard(EXCL_PIXELS),
            b'@' => return Self::Wide(AT_PIXELS),
            b'#' => return Self::Wide(HASH_PIXELS),
            b'%' => return Self::Standard(PERCENT_PIXELS),
            b'^' => return Self::Standard(CARET_PIXELS),
            b'&' => return Self::Wide(AMPERSAND_PIXELS),
            b'*' => return Self::Standard(ASTERISK_PIXELS),
            b')' => return Self::Standard(LEFT_PAREN_PIXELS),
            b'(' => return Self::Standard(RIGHT_PAREN_PIXELS),
            b'-' => return Self::Standard(MINUS_PIXELS),
            _ => return Self::Standard(UNKNOWN_PIXELS),
        };
    }
}

impl DisplayCharacter {
    const fn into_displaypixels(self) -> DisplayPixels {
        let mut output: DisplayPixels = [0u128; 64];
        let mut i: usize = 0;

        match self {
            Self::Standard(data) => {
                const CHAR_PIXEL_MASK: u16 = 0x7000; // 0b01110000
                while i < CHAR_LENGTH {
                    let x: u128 = ((data & (CHAR_PIXEL_MASK >> (i * CHAR_WIDTH)))
                        >> ((CHAR_LENGTH - (i + 1)) * CHAR_WIDTH) as u16)
                        as u128; // what the fuck?
                    output[i] |= x;
                    i += 1;
                }
            }

            Self::Wide(data) => {
                const CHAR_PIXEL_MASK: u32 = 0x7C000000;
                while i < WIDECHAR_LENGTH {
                    let x = ((data & (CHAR_PIXEL_MASK >> (i * WIDECHAR_WIDTH)))
                        >> (((WIDECHAR_LENGTH - (i + 1)) * WIDECHAR_WIDTH) + 6) as u32)
                        as u128;
                    output[i] |= x;
                    i += 1;
                }
            }

            Self::Small(data) => {
                const CHAR_PIXEL_MASK: u8 = 0xF;
                while i < SMALLCHAR_LENGTH {
                    let x: u128 = ((data & (CHAR_PIXEL_MASK >> (i * SMALLCHAR_WIDTH)))
                        >> ((SMALLCHAR_LENGTH - (i + 1)) * SMALLCHAR_WIDTH) as u8)
                        as u128;
                    output[i] |= x;
                    i += 1;
                }
            }
        };
        output
    }
}

const fn andeq_displaypixels(dest: &mut DisplayPixels, src: &DisplayPixels) {
    let mut i: usize = 0;
    while i < dest.len() {
        dest[i] |= src[i];
        i += 1;
    }
}

pub struct Position {
    x: u8,
    y: u8,
}

pub struct StaticText {
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
        let mut i: usize = 0;
        while i < self.len() {
            andeq_displaypixels(
                &mut output,
                &DisplayCharacter::new_from_ascii(self.text.chars().nth(i).unwrap() as u8)
                    .into_displaypixels(),
            );
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

macro_rules! pixelate_character {
    ($character:literal) => {
        DisplayCharacter::into_displaypixels(DisplayCharacter::new_from_ascii($character))
    };
}

const LETTER_PIXELS: [DisplayPixels; 26] = {
    let mut output: [DisplayPixels; 26] = [[0u128; 64]; 26];
    let mut i: u8 = b'a';
    let end: u8 = b'z' + 1;

    while i < end {
        output[(i - b'a') as usize] =
            DisplayCharacter::into_displaypixels(DisplayCharacter::new_from_ascii(i));
        i += 1;
    }

    output
};

const fn pixelate_text(text: &[u8]) -> DisplayPixels {
    let mut output: DisplayPixels = [0u128; 64];
    let mut i: usize = text.len();

    let mut right_cursor: usize = 127;

    while i > 0 {
        i -= 1;
        let letter: u8 = text[i];
        let this_letter = DisplayCharacter::new_from_ascii(letter);
        let (letter_length, letter_width) = match this_letter {
            DisplayCharacter::Small(_) => (SMALLCHAR_LENGTH, SMALLCHAR_WIDTH),
            DisplayCharacter::Standard(_) => (CHAR_LENGTH, CHAR_WIDTH),
            DisplayCharacter::Wide(_) => (WIDECHAR_LENGTH, WIDECHAR_WIDTH),
        };

        let left_shift: usize = 127 - right_cursor;
        let mut x: usize = 0;

        let letter_pixels = this_letter.into_displaypixels();

        while x < letter_length {
            output[x] |= letter_pixels[x] << left_shift;
            x += 1;
        }

        right_cursor = usize::checked_sub(right_cursor, letter_width + 1).unwrap();
    }

    output
}

pub fn main() {
    for i in LETTER_PIXELS {
        for x in i {
            let bytestring = format!("{:#0128b}", x)
                .replace("0", " ")
                .replace("1", "█")
                .replace("b", "");

            println!("{}", bytestring);
        }
    }
    let text: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let specials: &[u8] = b"0123456789!@#$%^&*()_+-";
    for x in 0..6 {
        println!(
            "{}",
            format!("{:#0128b}", pixelate_text(text)[x])
                .replace("0", " ")
                .replace("1", "█")
                .replace("b", "")
        );
    }
    for x in 0..6 {
        println!(
            "{}",
            format!("{:#0128b}", pixelate_text(specials)[x])
                .replace("0", " ")
                .replace("1", "█")
                .replace("b", "")
        );
    }
    for x in 0..6 {
        println!(
            "{}",
            format!("{:#0128b}", pixelate_text(b"DONT GET MY WORDS TWISTED")[x])
                .replace("0", " ")
                .replace("1", "█")
                .replace("b", "")
        );
    }
    for x in 0..6 {
        println!(
            "{}",
            format!("{:#0128b}", pixelate_text(b"CALL THAT SHIT TORSION")[x])
                .replace("0", " ")
                .replace("1", "█")
                .replace("b", "")
        );
    }
}

#![feature(portable_simd)]

use std::simd::prelude::*;

pub struct DisplayArr {
    left: u64x64,
    right: u64x64,
}

impl DisplayArr {
    // fn splat(left: u64, right: u64) -> Self {
    //     DisplayArr {
    //         left: u64x64::splat(left),
    //         right: u64x64::splat(right),
    //     }
    // }
    const fn constsplat(left: u64, right: u64) -> Self {
        DisplayArr {
            left: u64x64::from_array([left; 64]),
            right: u64x64::from_array([right; 64]),
        }
    }
    pub const fn new() -> Self {
        Self::constsplat(0, 0)
    }

    pub const fn new_full() -> Self {
        Self::constsplat(1, 1)
    }

    pub fn new_from_halves(left: u64x64, right: u64x64) -> Self {
        Self {
            left: left,
            right: right
        }
    }

    pub fn row(&self, idx: usize) -> u128 {
        let index: usize = if idx > 63 {63} else {idx};

        let left: u128 = self.left[index] as u128;
        let right: u128 = self.right[index] as u128;
        (left << 64) | right
    }

    #[inline(never)]
    pub fn column(&self, idx: usize) -> u64 { // TODO: can probably get this from a shifted mask?
        let index = if idx > 127 {127} else {idx};


        const fn get_bit(number: u64, position: usize) -> u64 {
            let pos: usize = if position > 63 {63} else {position};
            let mask = 1u64 << pos;
            let output = (number & mask) >> pos;
            if !((output == 0) || (output == 1)) {
                unsafe{std::hint::unreachable_unchecked();}
                //unreachable!(); //382
                //382
            }
            output
        }

        match index {
            0..=63 => {
                let mut output: u64 = 0;
                for i in 0..64 {
                    output |= get_bit(self.left[i], index) << i;
                }
                output
            },

            64..=127 => {
                let mut output: u64 = 0;
                for i in 0..64 {
                    output |= get_bit(self.right[i], index - 64) << i;
                }
                output
            },

            _ => unreachable!(),
        }
    }
}
