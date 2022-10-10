use super::*;
use core::{mem, ops::Range};

pub struct Rect {
    position: Vec2<isize>,
    size: Vec2<isize>,
    color: Color,
}

impl Rect {
    #[inline]
    pub fn new(position: Vec2<isize>, size: Vec2<isize>, color: Color) -> Self {
        Self {
            position,
            size,
            color,
        }
    }
}

impl Draw for Rect {
    fn draw<T: Canvas>(&self, canvas: &mut T) {
        for dy in sorted(self.position.y..self.position.y + self.size.y) {
            for dx in sorted(self.position.x..self.position.x + self.size.x) {
                canvas.blit_pixel(dx, dy, self.color);
            }
        }
    }
}

trait Sorted {
    fn sorted(self) -> Self;
}

impl Sorted for Range<isize> {
    fn sorted(self) -> Self {
        let Self { mut start, mut end } = self;

        if start > end {
            mem::swap(&mut start, &mut end);
        }

        Self { start, end }
    }
}

fn sorted<T: Sorted>(x: T) -> T {
    x.sorted()
}
