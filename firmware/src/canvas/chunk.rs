use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

#[derive(Default, Clone, Copy)]
pub struct Chunk(u8);

impl Chunk {
    #[inline]
    pub fn bit(i: usize) -> Self {
        (1 << i).into()
    }
}

impl From<Chunk> for u8 {
    #[inline]
    fn from(chunk: Chunk) -> Self {
        chunk.0
    }
}

impl From<u8> for Chunk {
    #[inline]
    fn from(mask: u8) -> Self {
        Chunk(mask)
    }
}

impl BitOrAssign for Chunk {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOr for Chunk {
    type Output = Self;

    #[inline]
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl BitAndAssign for Chunk {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAnd for Chunk {
    type Output = Self;

    #[inline]
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl Not for Chunk {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
