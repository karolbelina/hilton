use crate::canvas::{Canvas, Color, Vec2};
use avr_hal_generic::port::PinOps;

pub fn render<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>(
    canvas: &mut Canvas<RST, SCE, DC, DIN, CLK>,
) {
    canvas.draw_filled_circle(Vec2::new(42, 20), 12, Color::On);
    render_eyes(canvas, Vec2::new(0, 0));
    render_nose(canvas);
}

fn render_eyes<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>(
    canvas: &mut Canvas<RST, SCE, DC, DIN, CLK>,
    look_direction: Vec2<isize>,
) {
    canvas.draw_filled_circle(Vec2::new(37, 24), 3, Color::Off);
    canvas.draw_filled_circle(Vec2::new(47, 24), 3, Color::Off);

    let [left_pupil_offset, right_pupil_offset] = pupils_offsets(look_direction);
    const PUPIL_SIZE: Vec2<isize> = Vec2::new(2, 3);

    canvas.draw_rect(Vec2::new(37, 23) + left_pupil_offset, PUPIL_SIZE, Color::On);
    canvas.draw_rect(
        Vec2::new(46, 23) + right_pupil_offset,
        PUPIL_SIZE,
        Color::On,
    );
}

fn pupils_offsets(look_direction: Vec2<isize>) -> [Vec2<isize>; 2] {
    let [left_x, right_x] = match look_direction.x {
        0 => [0, 0],
        x if x > 0 => [x - 1, x],
        x if x < 0 => [x, x + 1],
        _ => unreachable!(),
    };

    [
        Vec2::new(left_x, look_direction.y),
        Vec2::new(right_x, look_direction.y),
    ]
}

fn render_nose<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>(
    canvas: &mut Canvas<RST, SCE, DC, DIN, CLK>,
) {
    canvas.set_pixel_color(Vec2::new(41, 28), Color::Off);
    canvas.set_pixel_color(Vec2::new(42, 28), Color::Off);
    canvas.set_pixel_color(Vec2::new(43, 28), Color::Off);
    canvas.set_pixel_color(Vec2::new(42, 29), Color::Off);
}
