use macroquad::prelude::*;

// Window constants
const WINDOW_HEIGHT: f32 = 600.;
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT_HALF: f32 = WINDOW_HEIGHT / 2.;
const WINDOW_WIDTH_HALF: f32 = WINDOW_WIDTH / 2.;

// Player rectangle constants
const RECT_PADDING: f32 = 5.;
const RECT_HEIGHT: f32 = 80.;
const RECT_WIDTH: f32 = 20.;
const RECT_HEIGHT_HALF: f32 = RECT_HEIGHT / 2.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Pong".to_string(),
        window_height: WINDOW_HEIGHT as i32,
        window_width: WINDOW_WIDTH as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(GREEN);

        draw_line(
            WINDOW_WIDTH_HALF,
            0.,
            WINDOW_WIDTH_HALF,
            WINDOW_HEIGHT,
            10.0,
            WHITE,
        );

        draw_rectangle(
            RECT_PADDING,
            WINDOW_HEIGHT_HALF - RECT_HEIGHT_HALF,
            RECT_WIDTH,
            RECT_HEIGHT,
            WHITE,
        );

        draw_rectangle(
            WINDOW_WIDTH - RECT_WIDTH - RECT_PADDING,
            WINDOW_HEIGHT_HALF - RECT_HEIGHT_HALF,
            RECT_WIDTH,
            RECT_HEIGHT,
            WHITE,
        );

        next_frame().await
    }
}
