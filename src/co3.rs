use std::ops;

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
