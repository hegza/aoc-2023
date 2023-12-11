use std::{num::TryFromIntError, ops};

/// 2D coordinate represented as a two-value tuple
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Co2<T>(pub T, pub T);

// Impl (T, T) + (T, T) as (T + T, T + T) when T is Addable
impl<T: ops::Add<Output = T>> ops::Add for Co2<T> {
    type Output = Co2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Co2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> From<(T, T)> for Co2<T> {
    fn from(value: (T, T)) -> Self {
        Co2(value.0, value.1)
    }
}

impl<T> Co2<T>
where
    T: Copy,
{
    pub fn as_tuple(&self) -> (T, T) {
        (self.0, self.1)
    }
}

impl ops::Sub for Co2<usize> {
    type Output = (isize, isize);

    fn sub(self, rhs: Self) -> Self::Output {
        (
            self.0 as isize - rhs.0 as isize,
            self.1 as isize - rhs.1 as isize,
        )
    }
}

impl ops::Sub for Co2<u64> {
    type Output = (i64, i64);

    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 as i64 - rhs.0 as i64, self.1 as i64 - rhs.1 as i64)
    }
}

impl TryFrom<(isize, isize)> for Co2<usize> {
    type Error = TryFromIntError;

    fn try_from(value: (isize, isize)) -> Result<Self, Self::Error> {
        let row = usize::try_from(value.0)?;
        let col = usize::try_from(value.1)?;
        Ok(Co2(row, col))
    }
}

impl TryFrom<(i64, i64)> for Co2<u64> {
    type Error = TryFromIntError;

    fn try_from(value: (i64, i64)) -> Result<Self, Self::Error> {
        let row = u64::try_from(value.0)?;
        let col = u64::try_from(value.1)?;
        Ok(Co2(row, col))
    }
}
