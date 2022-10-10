use super::*;

pub struct Pixel {
    position: Vec2<isize>,
    color: Color,
}

impl Pixel {
    #[inline]
    pub fn new(position: Vec2<isize>, color: Color) -> Self {
        Pixel { position, color }
    }
}

impl Draw for Pixel {
    fn draw(&self, canvas: &mut impl Canvas) {
        canvas.blit_pixel(self.position.x, self.position.y, self.color);
    }
}
