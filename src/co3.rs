use std::{num::TryFromIntError, ops};

/// 3D coordinate represented as a three-value tuple
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Co3<T>(pub T, pub T, pub T);

// Impl (T, T, T) + (T, T, T) as (T + T, T + T, T + T) when T is Addable
impl<T: ops::Add<Output = T>> ops::Add for Co3<T> {
    type Output = Co3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Co3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T> From<(T, T, T)> for Co3<T> {
    fn from(value: (T, T, T)) -> Self {
        Co3(value.0, value.1, value.2)
    }
}

impl<T> Co3<T>
where
    T: Copy,
{
    pub fn as_tuple(&self) -> (T, T, T) {
        (self.0, self.1, self.2)
    }
}

#[test]
fn co3_add() {
    let a = Co3::from((1, 2, 3));
    let b = Co3::from((1, 2, 3));

    assert_eq!(a + b, Co3::from((2, 4, 6)));
}

impl ops::Sub for Co3<usize> {
    type Output = (isize, isize, isize);

    fn sub(self, rhs: Self) -> Self::Output {
        (
            self.0 as isize - rhs.0 as isize,
            self.1 as isize - rhs.1 as isize,
            self.2 as isize - rhs.2 as isize,
        )
    }
}

impl ops::Sub for Co3<u64> {
    type Output = (i64, i64, i64);

    fn sub(self, rhs: Self) -> Self::Output {
        (
            self.0 as i64 - rhs.0 as i64,
            self.1 as i64 - rhs.1 as i64,
            self.2 as i64 - rhs.2 as i64,
        )
    }
}

impl TryFrom<(isize, isize, isize)> for Co3<usize> {
    type Error = TryFromIntError;

    fn try_from(value: (isize, isize, isize)) -> Result<Self, Self::Error> {
        let row = usize::try_from(value.0)?;
        let col = usize::try_from(value.1)?;
        let zcol = usize::try_from(value.2)?;
        Ok(Co3(row, col, zcol))
    }
}

impl TryFrom<(i64, i64, i64)> for Co3<u64> {
    type Error = TryFromIntError;

    fn try_from(value: (i64, i64, i64)) -> Result<Self, Self::Error> {
        let row = u64::try_from(value.0)?;
        let col = u64::try_from(value.1)?;
        let zcol = u64::try_from(value.2)?;
        Ok(Co3(row, col, zcol))
    }
}
