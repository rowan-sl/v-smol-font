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
//! each row is 10 chars. total len is 64
//!  
//! ```text
//! 0123456789
//! abcdefghij
//! klmnopqrst
//! uvwxyz ~`#
//! %^&*()_=+-
//! []|\:;"'<>
//! ?/,.        // empty space here is not included in the font
//! ```
//!
//! [`index`]: index
//! [`lookup`]: lookup

// should (hopefully) only disable when testing THIS crate
#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

// this len was not planned i swear
/// length of a font. (all charecters)
const LENGTH: usize = 64;

/// contains the data for a font.
///
/// not constructable outside this crate to ensure validity of
/// fonts for [`index`] and [`lookup`] at the type level.
pub struct Font {
    data: [BitmapChar; LENGTH],
}

impl Font {
    /// Private function for constructing a new font.
    /// intentionally not available outside the crate to ensure validity
    /// of fonts
    const fn new(data: [BitmapChar; LENGTH]) -> Self {
        Self { data }
    }

    /// Gets the data for a font. each u16 contains the data for each char in row-major order.
    ///
    /// this is a irreversable operation
    pub const fn as_data(&self) -> &[BitmapChar; LENGTH] {
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
    // this is only usefull really for using it to do format conversion (no reason to have it be private)
    /// Create a new BitmapChar. THIS IS LIKELY NOT WHAT YOU ARE LOOKING FOR! take a look at [`lookup`]
    pub const fn new(data: u16) -> Self {
        Self(data)
    }

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
                } else if x == 0{
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
        [
            to_list(a),
            to_list(b),
            to_list(c),
            to_list(d),
        ]
    }
}

/// the original, unmodified font.
pub mod original {
    // pub const FONT: F
}

/// Get the index into the font of a spacific charecter.
///
/// This index will be a valid index to all font arrays, and will return the
/// font data for the charecter passed to it.
///
/// Returns `None` if the requested charecter does not exist in that font.
///
/// Automatically upercases ascii alphabetics, as all fonts do not support lowercase.
pub const fn index(c: char) -> Option<usize> {
    todo!()
}

/// Look up a charecter in a font and return it, returning `None` if
/// the font does not support that charecter
///
/// For more details, see [`index`]
pub const fn lookup(c: char, f: &'static Font) -> Option<BitmapChar> {
    todo!()
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
    fn test_to_array() {
        assert_eq!(
            super::BitmapChar::new(0b0001_0011_0111_1111).to_array(),
            [
                [false, false, false, true],
                [false, false, true, true],
                [false, true, true, true],
                [true, true, true, true],
            ]
        )
    }
}
