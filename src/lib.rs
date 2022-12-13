//! A Very Small Bitmap Font
//!
//! Raw font data can be found in the sub-modules.
//!
//! To get the index of a spacific charecter, use [`index`]. to look up a charecter in 16-bit bitmap format, use [`lookup`]
//!
//! ## Compatibility
//!
//! This crate has no dependancies, is no-std compatable, and full const-eval (you read that right).
//!
//! ## Font Layout
//!
//! (dont worry, [`lookup`] and [`index`] handle this for you, this is just a reference)
//!
//! each row is 10 chars. total len is 68
//!  
//! ```text
//! 0123456789
//! abcdefghij
//! klmnopqrst
//! uvwxyz ~`!
//! @#%^&*_=+-
//! (){}[]|\:;
//! "'<>?/,.    // empty space here is not included in the font
//! ```
//!
//! [`index`]: index
//! [`lookup`]: lookup

// should (hopefully) only disable when testing THIS crate
#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

/// length of a font. (all charecters)
const LENGTH: usize = 68;

/// contains the data for a font.
///
/// not constructable outside this crate to ensure validity of
/// fonts for [`index`] and [`lookup`] at the type level.
pub struct Font {
    data: [Option<BitmapChar>; LENGTH],
}

impl Font {
    /// Private function for constructing a new font.
    /// intentionally not available outside the crate to ensure validity
    /// of fonts
    const fn new(data: [Option<BitmapChar>; LENGTH]) -> Self {
        Self { data }
    }

    /// Gets the data for a font. each u16 contains the data for each char in row-major order.
    ///
    /// this is a irreversable operation
    pub const fn as_data(&self) -> &[Option<BitmapChar>; LENGTH] {
        &self.data
    }
}

/// A single bitmap charecter. repr(transparent) on `u16`
///
/// To get the bitmap charecter for some [`char`], use [`lookup`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BitmapChar(u16);

impl BitmapChar {
    /// get the bitmap data (16 bits, or 4 rows of 4 bits each).
    ///
    /// example of the return (for letter 'a', unspecified font, X=1, space=0)
    ///
    /// ```text
    /// XXXX
    /// X XX
    /// XXXX
    /// X XX
    /// ```
    ///
    pub const fn to_bitmap(self) -> u16 {
        self.0
    }

    /// gets the charecter data, as an 4x4 2d array of booleans.
    ///
    /// example of the return (for letter 'a', unspecified font, X=true, space=false, commas omitted)
    ///
    /// ```text
    /// [
    ///     [XXXX]
    ///     [X XX]
    ///     [XXXX]
    ///     [X XX]
    /// ]
    /// ```
    ///
    pub const fn to_array(self) -> [[bool; 4]; 4] {
        // this is *not* nice code, i know.

        // split rows
        let [a, b, c, d] = [
            (self.0 & (0b1111 << 12)) >> 12,
            (self.0 & (0b1111 << 8)) >> 8,
            (self.0 & (0b1111 << 4)) >> 4,
            (self.0 & 0b1111),
        ];
        // split each row into bits
        #[inline(always)]
        const fn to_list(x: u16) -> [bool; 4] {
            #[inline(always)]
            const fn to_bool(x: u16) -> bool {
                if x == 1 {
                    true
                } else if x == 0 {
                    false
                } else {
                    panic!()
                }
            }
            [
                to_bool((x & 0b1000) >> 3),
                to_bool((x & 0b0100) >> 2),
                to_bool((x & 0b0010) >> 1),
                to_bool(x & 0b0001),
            ]
        }
        [to_list(a), to_list(b), to_list(c), to_list(d)]
    }
}

/// creates a charecter from the raw data.
/// always returns Some (for convenience of construction of font arrays)
const fn make_char(a: u16, b: u16, c: u16, d: u16) -> Option<BitmapChar> {
    let m = 0b1111u16;
    let (a, b, c, d) = (a & m, b & m, c & m, d & m);
    let mut raw = 0u16;
    raw |= a << 12;
    raw |= b << 8;
    raw |= c << 4;
    raw |= d;
    Some(BitmapChar(raw))
}

/// creates a charecter from a bool array
/// always returns Some (for convenience of construction of font arrays)
const fn make_char_bool_array(arr: [[bool; 4]; 4]) -> Option<BitmapChar> {
    let mut raw = 0u16;
    let mut i = 0usize;
    while i < 16 {
        let v = arr[i / 4][i % 4];
        raw |= (v as u16) << (15 - i);
        i += 1;
    }
    Some(BitmapChar(raw))
}

/// the original, unmodified font.
pub mod original {
    use crate::{make_char_bool_array, Font};
    include!("../gen/font.rs");
}

/// Get the index into the font of a spacific charecter.
///
/// This index will be a valid index to all font arrays, and will return the
/// font data for the charecter passed to it, or None if the requested charecter does not exist.
///
/// Automatically sets the same case (unspecified) for ascii alphabetics, as all fonts do not support case.
pub const fn index(c: char) -> Option<usize> {
    //TODO: possibly use https://github.com/rust-phf/rust-phf
    // consteval reeeeeee
    // this was auto-generated
    Some(match c.to_ascii_lowercase() {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        'g' => 16,
        'h' => 17,
        'i' => 18,
        'j' => 19,
        'k' => 20,
        'l' => 21,
        'm' => 22,
        'n' => 23,
        'o' => 24,
        'p' => 25,
        'q' => 26,
        'r' => 27,
        's' => 28,
        't' => 29,
        'u' => 30,
        'v' => 31,
        'w' => 32,
        'x' => 33,
        'y' => 34,
        'z' => 35,
        ' ' => 36,
        '~' => 37,
        '`' => 38,
        '!' => 39,
        '@' => 40,
        '#' => 41,
        '%' => 42,
        '^' => 43,
        '&' => 44,
        '*' => 45,
        '_' => 46,
        '=' => 47,
        '+' => 48,
        '-' => 49,
        '(' => 50,
        ')' => 51,
        '{' => 52,
        '}' => 53,
        '[' => 54,
        ']' => 55,
        '|' => 56,
        '\\' => 57,
        ':' => 58,
        ';' => 59,
        '\"' => 60,
        '\'' => 61,
        '<' => 62,
        '>' => 63,
        '?' => 64,
        '/' => 65,
        ',' => 66,
        '.' => 67,
        _ => return None,
    })
}

/// Look up a charecter in a font and return it, returning `None` if
/// the font does not support that charecter
///
/// For more details, see [`index`]
pub const fn lookup(c: char, f: &'static Font) -> Option<BitmapChar> {
    let i = if let Some(i) = index(c) {
        i
    } else {
        return None;
    };
    f.as_data()[i]
}

#[cfg(test)]
mod tests {
    #[test]
    fn bitwise_works_right() {
        assert_eq!(0b0011_0000u8, 0b1100_0000u8 >> 2);
        // BIG PITFALL RIGHT HERE FOLKS
        assert_eq!(0b0000_0011u8, 0b11);
        assert_eq!(0b1u8, 1u8);
    }

    #[test]
    fn bitmap_char_to_array() {
        assert_eq!(
            super::BitmapChar(0b0001_0011_0111_1111).to_array(),
            [
                [false, false, false, true],
                [false, false, true, true],
                [false, true, true, true],
                [true, true, true, true],
            ]
        )
    }

    #[test]
    fn index_works() {
        for (i, c) in "0123456789abcdefghijklmnopqrstuvwxyz ~`!@#%^&*_=+-(){}[]|\\:;\"'<>?/,."
            .chars()
            .enumerate()
        {
            assert_eq!(Some(i), super::index(c))
        }
    }

    #[test]
    fn test_make_char() {
        assert_eq!(
            super::make_char(
                0b1100,
                0b1111,
                0b1101,
                0b1111,
            ),
            Some(crate::BitmapChar(0b1100_1111_1101_1111))
        )
    }

    #[test]
    fn test_make_char_bool_array() {
        let a =             super::make_char_bool_array([[true, true, false, false], [true, true, true, true], [true, true, false, true], [true, true, true, true]])
;
        println!("{:#0b}", a.unwrap().0);
        assert_eq!(
            a,
            Some(crate::BitmapChar(0b1100_1111_1101_1111))
        )
    }
}
