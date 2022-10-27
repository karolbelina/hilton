use super::*;
use stockbook::{stamp, Stamp};

static EAR_STAMP: Stamp = stamp!("assets/ear.png");
static STRAND_STAMP: Stamp = stamp!("assets/strand.png");
static WHISKERS_STAMP: Stamp = stamp!("assets/whiskers.png");
static TORSO_STAMP: Stamp = stamp!("assets/torso.png");
static TAIL_STAMP: Stamp = stamp!("assets/tail.png");

macro_rules! bitmaps {
    ($($name:ident($stamp:ident)),* $(,)?) => {
        $(
            impl Bitmap {
                pub fn $name(position: Vec2<isize>) -> Self {
                    Self::new(&$stamp, position)
                }
            }
        )*
    };
}

bitmaps! {
    ear(EAR_STAMP),
    strand(STRAND_STAMP),
    whiskers(WHISKERS_STAMP),
    torso(TORSO_STAMP),
    tail(TAIL_STAMP),
}

pub struct Bitmap {
    stamp: &'static Stamp,
    position: Vec2<isize>,
    flip_h: bool,
    flip_v: bool,
}

impl Bitmap {
    fn new(stamp: &'static Stamp, position: Vec2<isize>) -> Self {
        Self {
            stamp,
            position,
            flip_h: false,
            flip_v: false,
        }
    }
}

impl Bitmap {
    pub fn flip_h(mut self) -> Self {
        self.flip_h = !self.flip_h;
        self
    }
}

impl Draw for Bitmap {
    fn draw(&self, canvas: &mut impl Canvas) {
        let [width, height] = self.stamp.size();

        for (u, v, color) in self.stamp.pixels() {
            let x = match self.flip_h {
                false => u,
                true => width - u - 1,
            };
            let y = match self.flip_v {
                false => v,
                true => height - v - 1,
            };

            if color == stockbook::Color::Black {
                canvas.blit_pixel(
                    self.position.x + x as isize,
                    self.position.y + y as isize,
                    Color::On,
                );
            }
        }
    }
}
