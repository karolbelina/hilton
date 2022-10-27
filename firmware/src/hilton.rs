use crate::canvas::*;

pub fn draw(canvas: &mut impl Canvas) {
    Circle::new(Vec2::new(42, 20), 12, Color::On).draw(canvas);

    draw_eyes(canvas, Vec2::new(0, 0));
    draw_nose(canvas);

    Bitmap::ear(Vec2::new(48, 4)).draw(canvas);
    Bitmap::ear(Vec2::new(27, 4)).flip_h().draw(canvas);

    Bitmap::strand(Vec2::new(42, 4)).draw(canvas);

    Bitmap::whiskers(Vec2::new(28, 26)).draw(canvas);
    Bitmap::whiskers(Vec2::new(50, 26)).flip_h().draw(canvas);

    Bitmap::torso(Vec2::new(34, 33)).draw(canvas);

    Bitmap::tail(Vec2::new(50, 36)).draw(canvas);
}

fn draw_eyes(canvas: &mut impl Canvas, look_direction: Vec2<isize>) {
    const EYE_POSITIONS: [Vec2<isize>; 2] = [Vec2::new(37, 24), Vec2::new(47, 24)];

    for position in EYE_POSITIONS {
        Circle::new(position, 3, Color::Off).draw(canvas);
    }

    const PUPIL_ORIGINS: [Vec2<isize>; 2] = [Vec2::new(37, 23), Vec2::new(46, 23)];
    let pupil_offsets = pupils_offsets(look_direction);

    for position in PUPIL_ORIGINS
        .into_iter()
        .zip(pupil_offsets)
        .map(|(a, b)| a + b)
    {
        Rect::new(position, Vec2::new(2, 3), Color::On).draw(canvas);
    }
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

fn draw_nose(canvas: &mut impl Canvas) {
    for position in [
        Vec2::new(41, 28),
        Vec2::new(42, 28),
        Vec2::new(43, 28),
        Vec2::new(42, 29),
    ] {
        Pixel::new(position, Color::Off).draw(canvas);
    }
}
