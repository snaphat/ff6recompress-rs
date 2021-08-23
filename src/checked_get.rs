use std::{
    ops::{Bound, Index, IndexMut, Range, RangeBounds, RangeFull},
    slice::SliceIndex,
};

use len_trait::{index::IndexRange, IndexRangeMut, Len};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error
{
    #[error("range start index {0} out of range for slice of length {1}")]
    StartIndexError(usize, usize),
    #[error("range end index {0} out of range for slice of length {1}")]
    EndIndexError(usize, usize),

    #[error("attempted to index slice from after maximum usize")]
    StartIndexOverflowError(),
    #[error("attempted to index slice from after maximum usize")]
    EndIndexOverflowError(),
}

pub trait CheckedGet: IndexRange<usize> + IndexRangeMut<usize> + Len
{
    fn get_checked<I>(&self, index: I) -> Result<&<Self as Index<I>>::Output, Error>
    where
        I: RangeBounds<usize>,
        Self: Index<I>,
    {
        let start = match index.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(Error::StartIndexOverflowError())?,
            | Bound::Unbounded => 0,
        };

        let end = match index.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(Error::EndIndexOverflowError())?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => self.len(),
        };

        if start > end
        {
            return Err(Error::StartIndexError(start, end));
        }

        let len = self.len();

        if start > len
        {
            return Err(Error::StartIndexError(start, self.len()));
        }

        if end > len
        {
            return Err(Error::EndIndexError(end, self.len()));
        }
        Ok(&self[index])
    }

    fn get_checked_mut<I>(&mut self, index: I) -> Result<&mut <Self as Index<I>>::Output, Error>
    where
        I: RangeBounds<usize>,
        Self: IndexMut<I>,
    {
        let start = match index.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(Error::StartIndexOverflowError())?,
            | Bound::Unbounded => 0,
        };

        let end = match index.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(Error::EndIndexOverflowError())?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => self.len(),
        };

        if start > end
        {
            return Err(Error::StartIndexError(start, end));
        }

        let len = self.len();

        if start > len
        {
            return Err(Error::StartIndexError(start, self.len()));
        }

        if end > len
        {
            return Err(Error::EndIndexError(end, self.len()));
        }
        Ok(&mut self[index])
    }
}

impl CheckedGet for [u8] {}

#[cfg(test)]
mod test
{
    use super::{CheckedGet, Error};

    #[test]
    fn range_full()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let ret = bytes.get_checked(..).unwrap();
        assert_eq!(ret.len(), 16);
        assert_eq!(ret, bytes);
    }

    #[test]
    fn range_full_exclusive()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let ret = bytes.get_checked(2..5).unwrap();
        assert_eq!(ret.len(), 3);
        assert_eq!(ret, &bytes[2..5]);
    }

    #[test]
    fn range_full_inclusive()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let ret = bytes.get_checked(2..=5).unwrap();
        assert_eq!(ret.len(), 4);
        assert_eq!(ret, &bytes[2..=5]);
    }

    #[test]
    fn range_full_zero_exclusive()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let ret = bytes.get_checked(0..0).unwrap();
        assert_eq!(ret.len(), 0);
        assert_eq!(ret, &bytes[0..0]);
    }

    #[test]
    fn range_full_zero_inclusive()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let ret = bytes.get_checked(0..=0).unwrap();
        assert_eq!(ret.len(), 1);
        assert_eq!(ret, &bytes[0..=0]);
    }

    #[test]
    fn range_from()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let ret = bytes.get_checked(5..).unwrap();
        assert_eq!(ret.len(), 16 - 5);
        assert_eq!(ret, &bytes[5..]);
    }

    #[test]
    fn range_from_zero()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let ret = bytes.get_checked(16..).unwrap();
        assert_eq!(ret.len(), 0);
        assert_eq!(ret, &bytes[16..]);
    }

    #[test]
    fn range_from_oob_error()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let err = bytes.get_checked(17..).unwrap_err();
        assert_eq!(err.to_string(), "range start index 17 out of range for slice of length 16");
    }

    #[test]
    fn range_from_inverse_error()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let err = bytes.get_checked(17..5).unwrap_err();
        assert_eq!(err.to_string(), "range start index 17 out of range for slice of length 5");
    }

    #[test]
    fn range_to_exclusive()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let ret = bytes.get_checked(..5).unwrap();
        assert_eq!(ret.len(), 5);
        assert_eq!(ret, &bytes[..5]);
    }

    #[test]
    fn range_to_inclusive()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let ret = bytes.get_checked(..=5).unwrap();
        assert_eq!(ret.len(), 6);
        assert_eq!(ret, &bytes[..6]);
    }

    #[test]
    fn range_overflow_error()
    {
        let bytes = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let err = bytes.get_checked(0..=usize::MAX).unwrap_err();
        assert_eq!(err.to_string(), Error::EndIndexOverflowError().to_string());
    }
}
