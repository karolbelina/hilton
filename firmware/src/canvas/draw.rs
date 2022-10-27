use super::*;

mod bitmap;
mod circle;
mod pixel;
mod rect;

pub use self::bitmap::*;
pub use self::circle::*;
pub use self::pixel::*;
pub use self::rect::*;

pub trait Draw {
    fn draw(&self, canvas: &mut impl Canvas);
}
