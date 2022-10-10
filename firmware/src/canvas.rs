mod color;
mod draw;
mod vec2;

pub use self::color::*;
pub use self::draw::*;
pub use self::vec2::*;

pub trait Canvas {
    fn blit_pixel(&mut self, x: isize, y: isize, color: Color);
}
