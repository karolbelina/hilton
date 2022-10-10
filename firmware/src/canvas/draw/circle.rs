use super::*;

pub struct Circle {
    center: Vec2<isize>,
    radius: isize,
    color: Color,
}

impl Circle {
    #[inline]
    pub fn new(center: Vec2<isize>, radius: isize, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }
}

impl Draw for Circle {
    /// Draws a circle using the Bresenham's circle algorithm.
    ///
    /// The circle's diameter in pixels is always an odd number and is
    /// `radius * 2 + 1`. Negative values of `radius` are valid -- the circle
    /// gets drawn the same way it would have if the sign was flipped.
    fn draw<T: Canvas>(&self, canvas: &mut T) {
        let mut x = 0;
        let mut y = self.radius.abs();
        let mut error = 5 - 4 * y;

        while x <= y {
            for dx in self.center.x - y..=self.center.x + y {
                canvas.blit_pixel(dx, self.center.y - x, self.color);
                canvas.blit_pixel(dx, self.center.y + x, self.color);
            }

            if error > 0 {
                for dx in self.center.x - x..=self.center.x + x {
                    canvas.blit_pixel(dx, self.center.y - y, self.color);
                    canvas.blit_pixel(dx, self.center.y + y, self.color);
                }

                y -= 1;
                error -= 8 * y;
            }

            x += 1;
            error += 8 * x + 4;
        }
    }
}
