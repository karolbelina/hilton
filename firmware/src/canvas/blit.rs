use super::*;
use avr_hal_generic::port::PinOps;

pub trait Blit {
    fn blit_pixel(&mut self, x: isize, y: isize, color: Color);
}

impl<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps> Blit
    for Canvas<RST, SCE, DC, DIN, CLK>
{
    fn blit_pixel(&mut self, x: isize, y: isize, color: Color) {
        let x = match usize::try_from(x) {
            Ok(x) => x,
            Err(_) => return,
        };
        let y = match usize::try_from(y) {
            Ok(y) => y,
            Err(_) => return,
        };

        let chunk = self.chunk_at(x, y);
        let mask = Chunk::bit(y % 8);

        match color {
            Color::On => *chunk |= mask,
            Color::Off => *chunk &= !mask,
        }
    }
}
