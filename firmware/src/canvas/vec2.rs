use core::{
    num::TryFromIntError,
    ops::{Add, AddAssign, Div},
};

#[derive(Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl TryFrom<Vec2<isize>> for Vec2<usize> {
    type Error = TryFromIntError;

    fn try_from(Vec2 { x, y }: Vec2<isize>) -> Result<Self, Self::Error> {
        let x = usize::try_from(x)?;
        let y = usize::try_from(y)?;
        Ok(Vec2 { x, y })
    }
}

impl From<Vec2<usize>> for Vec2<isize> {
    #[inline]
    fn from(Vec2 { x, y }: Vec2<usize>) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: AddAssign> Add for Vec2<T> {
    type Output = Vec2<T>;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Div<Output = T> + Clone> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs,
        }
    }
}
