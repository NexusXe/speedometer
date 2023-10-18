#![feature(inherent_associated_types)]
#![allow(incomplete_features)]
#![feature(const_trait_impl)]
#![feature(ascii_char)]
#![feature(const_mut_refs)]

type StandardCharacter = u16; // 5 x 3, with sign bit as visibility
type WideCharacter = u32; // 5 x 5, with sign bit as visibility and 6 trailing unused 0s
type SmallCharacter = u8; // 3x2, with sign bit as visibility

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

const A_PIXELS: StandardCharacter = 0b1111101111101101u16 as u16;
const B_PIXELS: StandardCharacter = 0b1110101110101110u16 as u16;
const C_PIXELS: StandardCharacter = 0b1111110110110111u16 as u16;
const D_PIXELS: StandardCharacter = 0b1110101101101110u16 as u16;
const E_PIXELS: StandardCharacter = 0b1111100111100111u16 as u16;
const F_PIXELS: StandardCharacter = 0b1111100111100100u16 as u16;
const G_PIXELS: StandardCharacter = 0b1111100101101111u16 as u16;
const H_PIXELS: StandardCharacter = 0b1101101111101101u16 as u16;
const I_PIXELS: StandardCharacter = 0b1111010010010111u16 as u16;
const J_PIXELS: StandardCharacter = 0b1111010010010110u16 as u16;
const K_PIXELS: StandardCharacter = 0b1101101110101101u16 as u16;
const L_PIXELS: StandardCharacter = 0b1101101101101111u16 as u16;
const M_PIXELS: WideCharacter = 0b11101110101101011000110001000000u32 as u32;
const N_PIXELS: StandardCharacter = 0b1110101101101101u16 as u16;
const O_PIXELS: StandardCharacter = 0b1111101101101111u16 as u16;
const P_PIXELS: StandardCharacter = 0b1111101111100100u16 as u16;
const Q_PIXELS: StandardCharacter = 0b1111101101111001u16 as u16;
const R_PIXELS: StandardCharacter = 0b1111101110101101u16 as u16;
const S_PIXELS: StandardCharacter = 0b1111100111001111u16 as u16;
const T_PIXELS: StandardCharacter = 0b1111010010010010u16 as u16;
const U_PIXELS: StandardCharacter = 0b1101101101101111u16 as u16;
const V_PIXELS: StandardCharacter = 0b1101101101101010u16 as u16;
const W_PIXELS: WideCharacter = 0b11000110001101011010111011000000u32 as u32;
const X_PIXELS: StandardCharacter = 0b1101101010101101u16 as u16;
const Y_PIXELS: StandardCharacter = 0b1101101101010010u16 as u16;
const Z_PIXELS: StandardCharacter = 0b1111001010100111u16 as u16;
const ZERO_PIXELS: StandardCharacter = 0b1111101101101111u16 as u16;
const ONE_PIXELS: StandardCharacter = 0b1110010010010111u16 as u16;
const TWO_PIXELS: StandardCharacter = 0b1111001011100111u16 as u16;
const THREE_PIXELS: StandardCharacter = 0b1111001111001111u16 as u16;
const FOUR_PIXELS: StandardCharacter = 0b1101101111001001u16 as u16;
const FIVE_PIXELS: StandardCharacter = 0b1111100111001110u16 as u16;
const SIX_PIXELS: StandardCharacter = 0b1111110111101111u16 as u16;
const SEVEN_PIXELS: StandardCharacter = 0b1111001010010010u16 as u16;
const EIGHT_PIXELS: StandardCharacter = 0b1111101111101111u16 as u16;
const NINE_PIXELS: StandardCharacter = 0b1111101111001001u16 as u16;
const SLASH_PIXELS: StandardCharacter = 0b0001001010100100u16 as u16;
const UNKNOWN_PIXELS: StandardCharacter = 0b1101010101010101u16 as u16;

pub enum DisplayCharacter {
    Standard(StandardCharacter),
    Wide(WideCharacter),
    Small(SmallCharacter),
}

impl DisplayCharacter {
    pub const fn new_from_ascii(character: u8) -> Self {
        debug_assert!(character.is_ascii());
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
            _ => return Self::Standard(UNKNOWN_PIXELS),
        };
    }
}

impl const Displayable for DisplayCharacter {
    fn into_displaypixels(self) -> DisplayPixels {
        let mut output: DisplayPixels = [0u128; 64];
        let mut i: usize = 0;

        match self {
            Self::Standard(data) => {
                const CHAR_PIXEL_MASK: u16 = 0x7000; // 0b01110000
                while i < CHAR_LENGTH {
                    let x = Into::<u128>::into(
                        ((data & (CHAR_PIXEL_MASK >> (i * CHAR_WIDTH)))
                            >> ((CHAR_LENGTH - (i + 1)) * CHAR_WIDTH))
                            as u16,
                    ); // what the fuck?
                    output[i] |= x;
                    i += 1;
                }
            }

            Self::Wide(data) => {
                const CHAR_PIXEL_MASK: u32 = 0x7C000000;
                while i < WIDECHAR_LENGTH {
                    let x = Into::<u128>::into(
                        ((data & (CHAR_PIXEL_MASK >> (i * WIDECHAR_WIDTH)))
                            >> (((WIDECHAR_LENGTH - (i + 1)) * WIDECHAR_WIDTH) + 6))
                            as u32,
                    ); // what the fuck?
                    output[i] |= x;
                    i += 1;
                }
            }

            Self::Small(data) => {
                const CHAR_PIXEL_MASK: u8 = 0xF;
                while i < SMALLCHAR_LENGTH {
                    let x = Into::<u128>::into(
                        ((data & (CHAR_PIXEL_MASK >> (i * SMALLCHAR_WIDTH)))
                            >> ((SMALLCHAR_LENGTH - (i + 1)) * SMALLCHAR_WIDTH))
                            as u8,
                    ); // what the fuck?
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

pub fn main() {
    const I: DisplayCharacter = DisplayCharacter::new_from_ascii(b'W');
    for x in I.into_displaypixels() {
        println!("{:#0128b}", x);
    }
}
