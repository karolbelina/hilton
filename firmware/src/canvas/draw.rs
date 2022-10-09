use super::{Canvas, Chunk, Color, Vec2};
use core::{mem, ops::Range};

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

    pub fn draw_rect(&mut self, position: Vec2<isize>, size: Vec2<isize>, color: Color) {
        for dy in sorted(0..size.y) {
            for dx in sorted(0..size.x) {
                self.set_pixel_color(position + Vec2::new(dx, dy), color);
            }
        }
    }

    /// Draws a circle using the midpoint circle algorithm.
    pub fn draw_circle(&mut self, center: Vec2<isize>, radius: isize, color: Color) {
        let radius = radius.abs();

        let mut f = 1 - radius;
        let mut ddf_x = 0;
        let mut ddf_y = -2 * radius;
        let mut x = 0;
        let mut y = radius;

        self.set_pixel_color(center + Vec2::new(0, radius), color);
        self.set_pixel_color(center + Vec2::new(0, -radius), color);
        self.set_pixel_color(center + Vec2::new(radius, 0), color);
        self.set_pixel_color(center + Vec2::new(-radius, 0), color);

        while x < y {
            if f >= 0 {
                y -= 1;
                ddf_y += 2;
                f += ddf_y;
            }

            x += 1;
            ddf_x += 2;
            f += ddf_x + 1;

            self.set_pixel_color(center + Vec2::new(x, y), color);
            self.set_pixel_color(center + Vec2::new(-x, y), color);
            self.set_pixel_color(center + Vec2::new(x, -y), color);
            self.set_pixel_color(center + Vec2::new(-x, -y), color);
            self.set_pixel_color(center + Vec2::new(y, x), color);
            self.set_pixel_color(center + Vec2::new(-y, x), color);
            self.set_pixel_color(center + Vec2::new(y, -x), color);
            self.set_pixel_color(center + Vec2::new(-y, -x), color);
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
