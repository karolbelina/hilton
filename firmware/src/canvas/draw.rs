use super::*;

mod circle;
mod pixel;
mod rect;

pub use self::circle::*;
pub use self::pixel::*;
pub use self::rect::*;

pub trait Draw {
    fn draw<T: Canvas>(&self, canvas: &mut T);
}
