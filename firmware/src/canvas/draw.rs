use super::{Canvas, Chunk, Color, Vec2};
use core::{
    mem,
    ops::{Range, RangeInclusive},
};

impl<RST, SCE, DC, DIN, CLK> Canvas<RST, SCE, DC, DIN, CLK> {
    pub fn set_pixel_color(&mut self, point: Vec2<isize>, color: Color) {
        let point = match Vec2::<usize>::try_from(point) {
            Ok(point) => point,
            Err(_) => return,
        };

        let chunk = self.chunk_at(point);
        let mask = Chunk::bit(point.y % 8);

        match color {
            Color::On => *chunk |= mask,
            Color::Off => *chunk &= !mask,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(Default::default());
    }

    pub fn draw_horizontal_line(&mut self, x_start: isize, x_end: isize, y: isize, color: Color) {
        for x in sorted(x_start..=x_end) {
            self.set_pixel_color(Vec2::new(x, y), color);
        }
    }

    pub fn draw_rect(&mut self, position: Vec2<isize>, size: Vec2<isize>, color: Color) {
        for dy in sorted(0..size.y) {
            for dx in sorted(0..size.x) {
                self.set_pixel_color(position + Vec2::new(dx, dy), color);
            }
        }
    }

    /// Draws a circle using the Bresenham's circle algorithm.
    ///
    /// The circle's diameter in pixels is always an odd number and is
    /// `radius * 2 + 1`. Negative values of `radius` are valid -- the circle
    /// gets drawn the same way it would have if the sign was flipped.
    pub fn draw_circle(&mut self, center: Vec2<isize>, radius: isize, color: Color) {
        let mut x = 0;
        let mut y = radius.abs();
        let mut error = 5 - 4 * y;

        while x <= y {
            self.set_pixel_color(center + Vec2::new(x, y), color);
            self.set_pixel_color(center + Vec2::new(x, -y), color);
            self.set_pixel_color(center + Vec2::new(-x, y), color);
            self.set_pixel_color(center + Vec2::new(-x, -y), color);
            self.set_pixel_color(center + Vec2::new(y, x), color);
            self.set_pixel_color(center + Vec2::new(y, -x), color);
            self.set_pixel_color(center + Vec2::new(-y, x), color);
            self.set_pixel_color(center + Vec2::new(-y, -x), color);

            if error > 0 {
                y -= 1;
                error -= 8 * y;
            }

            x += 1;
            error += 8 * x + 4;
        }
    }

    /// Draws a filled circle using the Bresenham's circle algorithm.
    ///
    /// The circle's diameter in pixels is always an odd number and is
    /// `radius * 2 + 1`. Negative values of `radius` are valid -- the circle
    /// gets drawn the same way it would have if the sign was flipped.
    pub fn draw_filled_circle(&mut self, center: Vec2<isize>, radius: isize, color: Color) {
        let mut x = 0;
        let mut y = radius.abs();
        let mut error = 5 - 4 * y;

        while x <= y {
            for dx in (center.x - y)..=(center.x + y) {
                self.set_pixel_color(Vec2::new(dx, center.y - x), color);
                self.set_pixel_color(Vec2::new(dx, center.y + x), color);
            }

            if error > 0 {
                for dx in (center.x - x)..=(center.x + x) {
                    self.set_pixel_color(Vec2::new(dx, center.y - y), color);
                    self.set_pixel_color(Vec2::new(dx, center.y + y), color);
                }

                y -= 1;
                error -= 8 * y;
            }

            x += 1;
            error += 8 * x + 4;
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

impl Sorted for RangeInclusive<isize> {
    fn sorted(self) -> Self {
        // Fields of `RangeInclusive` are private for some reason
        let mut start = *self.start();
        let mut end = *self.end();

        if start > end {
            mem::swap(&mut start, &mut end);
        }

        Self::new(start, end)
    }
}

fn sorted<T: Sorted>(x: T) -> T {
    x.sorted()
}
