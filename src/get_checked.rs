use std::ops::{self, Bound, RangeBounds};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error
{
    #[error("index {0} out of range for slice of length {1}")]
    IndexError(usize, usize),
    #[error("range start index {0} out of range for slice of length {1}")]
    StartIndexError(usize, usize),
    #[error("range end index {0} out of range for slice of length {1}")]
    EndIndexError(usize, usize),

    #[error("attempted to index slice from after maximum usize")]
    StartIndexOverflowError(),
    #[error("attempted to index slice from after maximum usize")]
    EndIndexOverflowError(),
}

pub trait GetCheckedSlice<T: ?Sized>
{
    type Output: ?Sized;
    fn get_checked(self, slice: &T) -> Result<&Self::Output, Error>;
}

impl<T> GetCheckedSlice<[T]> for usize
{
    type Output = T;

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&T, Error>
    {
        // SAFETY: `self` is checked to be in bounds.
        if self < slice.len()
        {
            unsafe { Ok(&*slice.get_unchecked(self)) }
        }
        else
        {
            Err(Error::IndexError(self, slice.len()))
        }
    }
}

impl<T> GetCheckedSlice<[T]> for ops::Range<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        let len = slice.len();
        if self.start > self.end
        {
            Err(Error::StartIndexError(self.start, self.end))
        }
        else if self.end > len
        {
            Err(Error::EndIndexError(self.end, len))
        }
        else
        {
            unsafe { Ok(&*slice.get_unchecked(self)) }
        }
    }
}

impl<T> GetCheckedSlice<[T]> for ops::RangeTo<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        (0..self.end).get_checked(slice)
    }
}

impl<T> GetCheckedSlice<[T]> for ops::RangeFrom<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        (self.start..slice.len()).get_checked(slice)
    }
}

impl<T> GetCheckedSlice<[T]> for ops::RangeFull
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        Ok(slice)
    }
}

impl<T> GetCheckedSlice<[T]> for ops::RangeInclusive<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        if *self.end() == usize::MAX
        {
            Err(Error::EndIndexOverflowError())
        }
        else
        {
            let start = match self.start_bound()
            {
                | Bound::Included(x) => *x,
                | Bound::Excluded(x) => x.checked_add(1).ok_or(Error::StartIndexOverflowError())?,
                | Bound::Unbounded => 0,
            };

            let end = match self.end_bound()
            {
                | Bound::Included(x) => x.checked_add(1).ok_or(Error::EndIndexOverflowError())?,
                | Bound::Excluded(x) => *x,
                | Bound::Unbounded => slice.len(),
            };

            let len = slice.len();

            match slice
            {
                | _ if start > end => Err(Error::StartIndexError(start, end))?,
                | _ if start > len => Err(Error::StartIndexError(start, slice.len()))?,
                | _ if end > len => Err(Error::EndIndexError(end, slice.len()))?,
                | _ => Ok(unsafe { &*slice.get_unchecked(self) }),
            }
        }
    }
}

impl<T> GetCheckedSlice<[T]> for ops::RangeToInclusive<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        (0..=self.end).get_checked(slice)
    }
}

pub trait GetChecked<I>
{
    #[inline]
    fn get_checked<T>(&self, index: T) -> Result<&T::Output, Error>
    where T: GetCheckedSlice<Self>
    {
        index.get_checked(self)
    }
}

impl<T> GetChecked<T> for [T] {}

#[cfg(test)]
mod test
{
    use super::{Error, GetChecked};

    #[test]
    fn index()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = *bytes.get_checked(4).unwrap();
        assert_eq!(ret, bytes[4]);
    }

    #[test]
    fn index_edge()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = *bytes.get_checked(15).unwrap();
        assert_eq!(ret, bytes[15]);
    }

    #[test]
    fn index_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked(16).unwrap_err();
        assert_eq!(err.to_string(), "index 16 out of range for slice of length 16");
    }

    #[test]
    fn range_full()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(..).unwrap();
        assert_eq!(ret.len(), 16);
        assert_eq!(ret, bytes);
    }

    #[test]
    fn range_full_exclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(2..5).unwrap();
        assert_eq!(ret.len(), 3);
        assert_eq!(ret, &bytes[2..5]);
    }

    #[test]
    fn range_full_inclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(2..=5).unwrap();
        assert_eq!(ret.len(), 4);
        assert_eq!(ret, &bytes[2..=5]);
    }

    #[test]
    fn range_full_zero_exclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(0..0).unwrap();
        assert_eq!(ret.len(), 0);
        assert_eq!(ret, &bytes[0..0]);
    }

    #[test]
    fn range_full_zero_inclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(0..=0).unwrap();
        assert_eq!(ret.len(), 1);
        assert_eq!(ret, &bytes[0..=0]);
    }

    #[test]
    fn range_from()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(5..).unwrap();
        assert_eq!(ret.len(), 16 - 5);
        assert_eq!(ret, &bytes[5..]);
    }

    #[test]
    fn range_from_zero()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(16..).unwrap();
        assert_eq!(ret.len(), 0);
        assert_eq!(ret, &bytes[16..]);
    }

    #[test]
    fn range_from_oob_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked(17..).unwrap_err();
        assert_eq!(err.to_string(), "range start index 17 out of range for slice of length 16");
    }

    #[test]
    fn range_from_inverse_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked(17..5).unwrap_err();
        assert_eq!(err.to_string(), "range start index 17 out of range for slice of length 5");
    }

    #[test]
    fn range_to_exclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(..5).unwrap();
        assert_eq!(ret.len(), 5);
        assert_eq!(ret, &bytes[..5]);
    }

    #[test]
    fn range_to_inclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(..=5).unwrap();
        assert_eq!(ret.len(), 6);
        assert_eq!(ret, &bytes[..6]);
    }

    #[test]
    fn range_overflow_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked(0..=usize::MAX).unwrap_err();
        assert_eq!(err.to_string(), Error::EndIndexOverflowError().to_string());
    }
}
