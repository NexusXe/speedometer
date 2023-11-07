use core::fmt::Formatter;
use core::intrinsics::const_eval_select;
use core::result::Result;
use core::result::Result::Err;
use core::result::Result::Ok;
use core::simd::prelude::*;
use core::unreachable;
use core::write;

pub struct DisplayArrIndexError {
    expected: usize,
    actual: usize,
}

/// Error variants for the [DisplayArr] struct.
pub enum DisplayArrErr {
    /// Index out of bounds error for a row. Argument 0 is expected maximum length, and argument 1 is actual length.
    InvalidRowError(DisplayArrIndexError),
    /// Index out of bounds error for a column. Argument 0 is expected maximum length, and argument 1 is actual length.
    InvalidColumnError(DisplayArrIndexError),
}

impl DisplayArrErr {
    const fn idx_error<const T: bool>(expected: usize, actual: usize) -> Self {
        if actual <= expected {
            panic!("threw an error where the actual index was <= expected maximum!");
        } else {
            match T {
                true => Self::InvalidRowError(DisplayArrIndexError { expected, actual }),
                false => Self::InvalidColumnError(DisplayArrIndexError { expected, actual }),
            }
        }
    }

    #[allow(non_upper_case_globals)]
    const row_err: fn(usize, usize) -> Self =
        |expected, actual| Self::idx_error::<true>(expected, actual);
    #[allow(non_upper_case_globals)]
    const column_err: fn(usize, usize) -> Self =
        |expected, actual| Self::idx_error::<false>(expected, actual);
}

impl core::fmt::Display for DisplayArrErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let (expected, actual, text) = match self {
            DisplayArrErr::InvalidRowError(DisplayArrIndexError { expected, actual }) => {
                (expected, actual, "row")
            }
            DisplayArrErr::InvalidColumnError(DisplayArrIndexError { expected, actual }) => {
                (expected, actual, "column")
            }
        };
        write!(
            f,
            "Invalid {} index provided: expected 0..={:}, found {:}",
            text, expected, actual
        )
    }
}

impl core::fmt::Debug for DisplayArrErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}


#[derive(Clone, Copy)]
pub struct DisplayArr {
    left: u64x64,
    right: u64x64,
}

impl PartialEq for DisplayArr {
    fn eq(&self, other: &Self) -> bool {
        (self.left == other.left) && (self.right == other.right)
    }
}

impl Eq for DisplayArr {}

impl DisplayArr {
    pub const LEN: usize = 64;

    pub const fn len() -> usize {
        Self::LEN
    }

    /// Splits a [u128] into `(u64, u64)`, where `0` is the upper 64 bits and `1` is the lower 64 bits.
    /// So, a u128 of `0x0123456789abcdef` will be split into `(0x01234567, 0x89abcdef)`.
    /// Inverse of [combine_u128].
    #[inline]
    const fn split_u128(num: u128) -> (u64, u64) {
        ((num << 64) as u64, num as u64)
    }

    /// Combines two [u64]s into a [u128], where the first [u64] is the upper 64 bits and the second [u64] is the lower 64 bits.
    /// So, `0x01234567, 0x89abcdef` will be combined into a u128 of `0x0123456789abcdef`.
    /// Inverse of [split_u128].
    #[inline]
    const fn combine_u128(a: u64, b: u64) -> u128 {
        ((a as u128) << 64) | b as u128
    }

    /// Gets bit `idx` from `self`, where 0 is the highest (leftmost) bit. Returns [DisplayArrErr::InvalidColumnError] if idx is out of bounds.
    const fn get_bit(number: u64, position: usize) -> Result<u64, DisplayArrErr> {
        if position > 63 {
            return Err(DisplayArrErr::InvalidColumnError(DisplayArrIndexError {
                expected: 63,
                actual: position,
            }));
        }
        let mask = 1u64 << position;
        let output = (number & mask) >> position;
        if !((output == 0) || (output == 1)) {
            unsafe {
                core::hint::unreachable_unchecked();
            }
        }
        Ok(output)
    }

    /// Creates a new [DisplayArr], where all left rows are the `left` parameter and all right rows are the `right` parameter
    pub const fn splat(left: u64, right: u64) -> Self {
        /// Fast splat at runtime
        fn simd_splat(left: u64, right: u64) -> DisplayArr {
            DisplayArr {
                left: u64x64::splat(left),
                right: u64x64::splat(right),
            }
        }

        /// Allow splat at compile-time
        #[inline(always)]
        const fn const_splat(left: u64, right: u64) -> DisplayArr {
            DisplayArr {
                left: u64x64::from_array([left; 64]),
                right: u64x64::from_array([right; 64]),
            }
        }

        unsafe { const_eval_select((left, right), const_splat, simd_splat) } // trollface
    }
    /// Creates a new zeroed [DisplayArr].
    pub const fn new() -> Self {
        Self::splat(0, 0)
    }
    /// Creates a new [DisplayArr] where are all pixels are 1.
    pub const fn new_full() -> Self {
        Self::splat(u64::MAX, u64::MAX)
    }

    /// Construct a new [DisplayArr] from two halves.
    pub fn new_from_halves(left: u64x64, right: u64x64) -> Self {
        Self {
            left: left,
            right: right,
        }
    }
    /// Construct a new [DisplayArr] from an array of the same shape.
    pub fn new_from_array(array: [u128; 64]) -> Self {
        let mut leftarr = [0u64; 64];
        let mut rightarr = [0u64; 64];
        let mut i: usize = 0;

        while i < 64 {
            (leftarr[i], rightarr[i]) = Self::split_u128(array[i]);
            i += 1;
        }
        Self::new_from_halves(u64x64::from_array(leftarr), u64x64::from_array(rightarr))
    }

    /// Get row-half `x` from `self`, where the returned value is the upper (left) half if y = true, else the returned value us the lower (right) half. Returns [DisplayArrErr::InvalidRowError] if idx is out of bounds.
    fn row_half(&self, x: usize, y: bool) -> Result<u64, DisplayArrErr> {
        return match x {
            0..=63 => Ok(if !y { self.left[x] } else { self.right[x] }),
            _ => Err(DisplayArrErr::row_err(63, x)),
        };
    }

    /// Gets row `idx` from `self`, where 0 is the top row. Returns [DisplayArrErr::InvalidRowError] if idx is out of bounds.
    pub fn row(&self, idx: usize) -> Result<u128, DisplayArrErr> {
        return match idx {
            0..=63 => Ok(Self::combine_u128(self.left[idx], self.right[idx])),
            _ => Err(DisplayArrErr::row_err(63, idx)),
        };
    }
    /// Gets column `idx` from `self`, where 0 is the highest (leftmost) column. Returns [DisplayArrErr::InvalidColumnError] if idx is out of bounds.
    pub fn column(&self, idx: usize) -> Result<u64, DisplayArrErr> {
        // TODO: can probably get this from a shifted mask?

        let mut output: u64 = 0;
        match idx {
            0..=63 => {
                for i in 0..64 {
                    output |= Self::get_bit(self.left[i], idx)? << i;
                }
                Ok(output)
            }

            64..=127 => {
                for i in 0..64 {
                    output |= Self::get_bit(self.right[i], idx - 64)? << i;
                }
                Ok(output)
            }

            _ => return Err(DisplayArrErr::column_err(127, idx)),
        }
    }

    /// Sets row `idx` in `self`. Returns [DisplayArrErr::InvalidRowError] if `idx` is out of bounds.
    pub fn set_row(&mut self, idx: usize, src: u128) -> Result<(), DisplayArrErr> {
        return match idx {
            0..=63 => {
                (self.left[idx], self.right[idx]) = Self::split_u128(src);
                Ok(())
            }
            _ => Err(DisplayArrErr::row_err(63, idx)),
        };
    }

    /// Preforms a bitwise or-assignment to `self`.
    pub fn oreq_row(&mut self, idx: usize, src: u128) -> Result<(), DisplayArrErr> {
        return match idx {
            0..=63 => {
                let (left, right) = Self::split_u128(src);
                self.left[idx] |= left;
                self.right[idx] |= right;
                Ok(())
            }

            _ => Err(DisplayArrErr::row_err(63, idx)),
        };
    }

    /// Gets a single boolean bit from `self`.
    pub fn bit(&self, row_idx: usize, column_idx: usize) -> Result<bool, DisplayArrErr> {
        let (column_idx_adj, half) = if column_idx > 63 {
            (column_idx - 64, true)
        } else {
            (column_idx, false)
        };
        let row_val: u64 = self.row_half(row_idx, half)?;

        Ok(Self::get_bit(row_val, column_idx_adj)? != 0)
    }

    /// Rotates rows down by a constant amount. It is preferable to have a single call to this with a value `N` determined
    /// at compile time, as this operation can be expensive if done in a loop.
    /// For example, `for _ in 0..4 {x.rotate_rows_down<1>()}` is ***much*** slower than `x.rotate_rows_down<4>()`.
    pub fn rotate_rows_down<const N: usize>(&mut self) {
        self.left = self.left.rotate_lanes_right::<N>();
        self.right = self.right.rotate_lanes_right::<N>();
    }
    /// Rotates rows up. It is preferable to have a single call to this with a value `N` determined
    /// at compile time, as this operation can be expensive if done in a loop.
    /// For example, `for _ in 0..4 {x.rotate_rows_up<1>()}` is ***much*** slower than `x.rotate_rows_up<4>()`.
    pub fn rotate_rows_up<const N: usize>(&mut self) {
        self.left = self.left.rotate_lanes_left::<N>();
        self.right = self.right.rotate_lanes_left::<N>();
    }
    /// Rotates columns right. This is an expensive operation, so use sparingly.
    pub fn rotate_rows_right(&mut self, idx: u32) {
        for i in 0..=63 {
            self.set_row(i, self.row(i).unwrap().rotate_right(idx))
                .unwrap();
        }
    }
    /// Rotates columns left. This is an expensive operation, so use sparingly.
    pub fn rotate_rows_left<const N: u32>(&mut self, idx: u32) {
        for i in 0..=63 {
            self.set_row(i, self.row(i).unwrap().rotate_left(idx))
                .unwrap();
        }
    }
}

impl core::ops::BitAnd for DisplayArr {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            left: self.left & rhs.left,
            right: self.right & rhs.right,
        }
    }
}

impl core::ops::BitAndAssign for DisplayArr {
    fn bitand_assign(&mut self, rhs: Self) {
        self.left &= rhs.left;
        self.right &= rhs.right;
    }
}

impl core::ops::BitOr for DisplayArr {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            left: self.left | rhs.left,
            right: self.right | rhs.right,
        }
    }
}

impl core::ops::BitOrAssign for DisplayArr {
    fn bitor_assign(&mut self, rhs: Self) {
        self.left |= rhs.left;
        self.right |= rhs.right;
    }
}

impl core::ops::BitXor for DisplayArr {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            left: self.left ^ rhs.left,
            right: self.right ^ rhs.right,
        }
    }
}

impl core::ops::BitXorAssign for DisplayArr {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.left ^= rhs.left;
        self.right ^= rhs.right;
    }
}

impl core::ops::Shl<usize> for DisplayArr {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut output = Self::new();
        for line in 0..Self::LEN {
            let _ = output.set_row(line, unsafe { self.row(line).unwrap_unchecked() } << rhs);
        }
        output
    }
}

impl core::ops::ShlAssign<usize> for DisplayArr {
    fn shl_assign(&mut self, rhs: usize) {
        for line in 0..Self::LEN {
            let _ = self.set_row(line, unsafe { self.row(line).unwrap_unchecked() } << rhs);
        }
    }
}

impl core::ops::Shr<usize> for DisplayArr {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut output = Self::new();
        for line in 0..Self::LEN {
            let _ = output.set_row(line, unsafe { self.row(line).unwrap_unchecked() } >> rhs);
        }
        output
    }
}

impl core::ops::ShrAssign<usize> for DisplayArr {
    fn shr_assign(&mut self, rhs: usize) {
        for line in 0..Self::LEN {
            let _ = self.set_row(line, unsafe { self.row(line).unwrap_unchecked() } >> rhs);
        }
    }
}

impl core::ops::Not for DisplayArr {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self {
            left: !self.left,
            right: !self.right,
        }
    }
}

impl core::ops::Neg for DisplayArr {
    type Output = Self;
    fn neg(self) -> Self::Output {
        !self
    }
}

impl core::fmt::Display for DisplayArr {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use core::iter::zip;

        const MAX_NUM_WIDTH: usize = 2;

        if self.left.lanes() != self.right.lanes() {
            unreachable!()
        }

        for row in zip(0.., zip((&self.left).to_array(), (&self.right).to_array())) {
            let padding_count: usize = MAX_NUM_WIDTH - if row.0 < 10 { 2 } else { 1 };
            let padding = if padding_count == 1 { " " } else { "  " };
            write!(f, "\n")?;
            write!(
                f,
                "{:}: {}{:0128b}",
                row.0,
                padding,
                Self::combine_u128(row.1 .0, row.1 .1)
            )?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl core::fmt::Debug for DisplayArr {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use core::iter::zip;

        const MAX_NUM_WIDTH: usize = 2;

        if self.left.lanes() != self.right.lanes() {
            unreachable!()
        }

        for row in zip(0.., zip((&self.left).to_array(), (&self.right).to_array())) {
            let padding_count: usize = MAX_NUM_WIDTH - if row.0 < 10 { 2 } else { 1 };
            let padding = if padding_count == 1 { " " } else { "  " };
            write!(f, "\n")?;
            write!(
                f,
                "{:}: {}{:}",
                row.0,
                padding,
                Self::combine_u128(row.1 .0, row.1 .1)
            )?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_U128: u128 = 0x0123456789abcdefu128;
    const UPPER_U64: u64 = (TEST_U128 << 64) as u64;
    const LOWER_U64: u64 = TEST_U128 as u64;
    const EMPTY_DISPLAYARR: DisplayArr = DisplayArr::new();
    const FULL_DISPLAYARR: DisplayArr = DisplayArr::new_full();

    #[test]
    fn u128_split() {
        assert_eq!(
            (UPPER_U64, LOWER_U64),
            super::DisplayArr::split_u128(TEST_U128)
        );
    }

    #[test]
    fn u128_combine() {
        assert_eq!(
            TEST_U128,
            super::DisplayArr::combine_u128(UPPER_U64, LOWER_U64)
        );
    }

    #[test]
    fn displayarr_creation() {
        use core::cell;
        let empty = cell::LazyCell::new(|| DisplayArr::new());
        let full = cell::LazyCell::new(|| DisplayArr::new_full());
        assert_eq!(EMPTY_DISPLAYARR, *empty);
        assert_eq!(FULL_DISPLAYARR, *full);
        assert_ne!(FULL_DISPLAYARR, EMPTY_DISPLAYARR);
        assert_ne!(*empty, *full);
    }

    #[test]
    fn displayarr_row_column_idents() {
        for i in 0usize..=63usize {
            assert_eq!(FULL_DISPLAYARR.row(i).unwrap(), u128::MAX);
            assert_eq!(EMPTY_DISPLAYARR.row(i).unwrap(), 0u128);
        }
        //dbg!(DisplayArr::split_u128(1u128));
        // for x in 0usize..128 {
        //     let (left, right) = DisplayArr::split_u128(1u128 << x);
        //     let arr = DisplayArr::splat(left, right);
        //     for i in 0usize..64 {
        //         assert_eq!(arr.row(i).unwrap(), 1u128 << x);
        //     }
        // }

        assert!(FULL_DISPLAYARR.row(64).is_err());
        assert!(EMPTY_DISPLAYARR.row(64).is_err());
        for i in 0usize..=127usize {
            assert_eq!(FULL_DISPLAYARR.column(i).unwrap(), u64::MAX);
            assert_eq!(EMPTY_DISPLAYARR.column(i).unwrap(), 0u64);
        }

        assert!(FULL_DISPLAYARR.column(128).is_err());
        assert!(EMPTY_DISPLAYARR.column(128).is_err());
    }

    #[test]
    fn displayarr_row_ops() {
        const EXPECTED_VAL: u128 = 128 << 64;
        let mut test = DisplayArr::splat(128u64, 0u64);
        for x in 0..128 {
            for i in 0..64 {
                assert_eq!(test.row(i).unwrap() << x, EXPECTED_VAL << x, "err row {:} shift {:} op {}", i, x, "<<");
            }
        }

        for x in 0..128 {
            for i in 0..64 {
                assert_eq!(test.row(i).unwrap() >> x, EXPECTED_VAL >> x, "err row {:} shift {:} op {}", i, x, ">>");
            }
        }

        // for x in 0..128 {
        //     for i in 0..64 {
        //         assert_eq!((test << x).row(i).unwrap(), EXPECTED_VAL << x, "err row {:} shift {:} op {}", i, x, "<< arr");
        //     }
        // }

        // for x in 0..128 {
        //     for i in 0..64 {
        //         assert_eq!((test >> x).row(i).unwrap(), EXPECTED_VAL >> x, "err row {:} shift {:} op {}", i, x, ">> arr");
        //     }
        // }

        assert_eq!(test ^ test, DisplayArr::new());
        assert_ne!(test  ^ test, test);
    }
}
