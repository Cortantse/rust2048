use tui::{style::Color, widgets::canvas::Line};

pub enum Direction {
    Up(f64),
    Down(f64),
    Left(f64),
    Right(f64),
}

pub fn draw_shape(
    ctx: &mut tui::widgets::canvas::Context,
    color: Color,
    directions: &[Direction],
    x: f64,
    y: f64,
) {
    let (mut x, mut y) = (x, y);
    for direction in directions {
        let (x2, y2) = match direction {
            Direction::Up(v) => (x, y + v),
            Direction::Down(v) => (x, y - v),
            Direction::Left(v) => (x - v, y),
            Direction::Right(v) => (x + v, y),
        };
        ctx.draw(&Line {
            x1: x,
            y1: y,
            x2,
            y2,
            color,
        });
        (x, y) = (x2, y2);
    }
}

pub fn get_bg_color_for_n(n: u32) -> Color {
    match n {
        2 => Color::Rgb(238, 228, 218),
        4 => Color::Rgb(237, 224, 200),
        8 => Color::Rgb(242, 177, 121),
        16 => Color::Rgb(245, 149, 99),
        32 => Color::Rgb(246, 124, 95),
        64 => Color::Rgb(246, 94, 59),
        128 => Color::Rgb(237, 207, 114),
        256 => Color::Rgb(237, 204, 97),
        512 => Color::Rgb(237, 200, 80),
        1024 => Color::Rgb(237, 197, 63),
        2048 => Color::Rgb(237, 194, 46),
        _ => Color::Gray,
    }
}

pub fn get_color_for_n(n: u32) -> Color {
    match n {
        n if n > 4 => Color::White,
        _ => Color::Black,
    }
}

pub fn draw_number(ctx: &mut tui::widgets::canvas::Context, n: u32) {
    let color = get_color_for_n(n);
    match n {
        2 => draw_shape(
            ctx,
            color,
            &[
                Direction::Right(4.0),
                Direction::Down(3.0),
                Direction::Left(4.0),
                Direction::Down(3.0),
                Direction::Right(4.0),
            ],
            3.0,
            8.0,
        ),
        4 => draw_shape(
            ctx,
            color,
            &[
                Direction::Down(3.0),
                Direction::Right(4.0),
                Direction::Up(3.0),
                Direction::Down(6.0),
            ],
            3.0,
            8.0,
        ),
        8 => draw_shape(
            ctx,
            color,
            &[
                Direction::Right(4.0),
                Direction::Down(3.0),
                Direction::Left(4.0),
                Direction::Up(3.0),
                Direction::Down(6.0),
                Direction::Right(4.0),
                Direction::Up(3.0),
            ],
            3.0,
            8.0,
        ),
        16 => {
            draw_shape(ctx, color, &[Direction::Down(6.0)], 3.0, 8.0);
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Left(3.0),
                    Direction::Down(6.0),
                    Direction::Right(3.0),
                    Direction::Up(3.0),
                    Direction::Left(3.0),
                ],
                8.0,
                8.0,
            )
        }
        32 => {
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(2.5),
                    Direction::Down(3.0),
                    Direction::Left(2.5),
                    Direction::Right(2.5),
                    Direction::Down(3.0),
                    Direction::Left(2.5),
                ],
                1.5,
                8.0,
            );
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(2.5),
                    Direction::Down(3.0),
                    Direction::Left(2.5),
                    Direction::Down(3.0),
                    Direction::Right(2.5),
                ],
                6.0,
                8.0,
            )
        }
        64 => {
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(2.5),
                    Direction::Left(2.5),
                    Direction::Down(3.0),
                    Direction::Right(2.5),
                    Direction::Down(3.0),
                    Direction::Left(2.5),
                    Direction::Up(2.5),
                ],
                1.5,
                8.0,
            );
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Down(3.0),
                    Direction::Right(2.5),
                    Direction::Up(3.0),
                    Direction::Down(6.0),
                ],
                6.0,
                8.0,
            )
        }
        128 => {
            draw_shape(ctx, color, &[Direction::Down(6.0)], 1.5, 8.0);
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(2.5),
                    Direction::Down(3.0),
                    Direction::Left(2.5),
                    Direction::Down(3.0),
                    Direction::Right(2.5),
                ],
                2.5,
                8.0,
            );
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(2.5),
                    Direction::Down(3.0),
                    Direction::Left(2.5),
                    Direction::Up(3.0),
                    Direction::Down(6.0),
                    Direction::Right(2.5),
                    Direction::Up(3.0),
                ],
                6.0,
                8.0,
            )
        }
        256 => {
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(2.0),
                    Direction::Down(3.0),
                    Direction::Left(2.0),
                    Direction::Down(3.0),
                    Direction::Right(2.0),
                ],
                1.0,
                8.0,
            );
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(2.0),
                    Direction::Left(2.0),
                    Direction::Down(3.0),
                    Direction::Right(2.0),
                    Direction::Down(3.0),
                    Direction::Left(2.0),
                ],
                4.0,
                8.0,
            );
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(2.0),
                    Direction::Left(2.0),
                    Direction::Down(6.0),
                    Direction::Right(2.0),
                    Direction::Up(3.0),
                    Direction::Left(2.0),
                ],
                7.0,
                8.0,
            )
        }
        512 => {
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(2.0),
                    Direction::Left(2.0),
                    Direction::Down(3.0),
                    Direction::Right(2.0),
                    Direction::Down(3.0),
                    Direction::Left(2.0),
                ],
                1.0,
                8.0,
            );
            draw_shape(ctx, color, &[Direction::Down(6.0)], 5.5, 8.0);
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(2.0),
                    Direction::Down(3.0),
                    Direction::Left(2.0),
                    Direction::Down(3.0),
                    Direction::Right(2.0),
                ],
                7.0,
                8.0,
            )
        }
        1024 => {
            draw_shape(ctx, color, &[Direction::Down(6.0)], 1.0, 8.0);
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Down(6.0),
                    Direction::Right(1.8),
                    Direction::Up(6.0),
                    Direction::Left(1.8),
                ],
                2.0,
                8.0,
            );
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(1.8),
                    Direction::Down(3.0),
                    Direction::Left(1.8),
                    Direction::Down(3.0),
                    Direction::Right(1.8),
                ],
                4.8,
                8.0,
            );
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Down(3.0),
                    Direction::Right(1.8),
                    Direction::Up(3.0),
                    Direction::Down(6.0),
                ],
                7.4,
                8.0,
            )
        }
        2048 => {
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(1.5),
                    Direction::Down(3.0),
                    Direction::Left(1.5),
                    Direction::Down(3.0),
                    Direction::Right(1.5),
                ],
                1.0,
                8.0,
            );
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Down(6.0),
                    Direction::Right(1.5),
                    Direction::Up(6.0),
                    Direction::Left(1.5),
                ],
                3.2,
                8.0,
            );
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Down(3.0),
                    Direction::Right(1.5),
                    Direction::Up(3.0),
                    Direction::Down(6.0),
                ],
                5.3,
                8.0,
            );
            draw_shape(
                ctx,
                color,
                &[
                    Direction::Right(1.5),
                    Direction::Down(3.0),
                    Direction::Left(1.5),
                    Direction::Up(3.0),
                    Direction::Down(6.0),
                    Direction::Right(1.5),
                    Direction::Up(3.0),
                ],
                7.8,
                8.0,
            )
        }
        _ => {}
    }
}
