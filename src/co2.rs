use std::ops;

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
