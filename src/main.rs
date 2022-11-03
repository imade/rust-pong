mod collision;
mod shapes;

use collision::is_collision;
use macroquad::prelude::*;
use shapes::{Circle, Rectangle};

// Window constants
const WINDOW_HEIGHT: f32 = 600.;
const WINDOW_WIDTH: f32 = 800.;

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
  let speed = 1f32;
  let mut circle = Circle::new(330., 281., 50., RED);
  let mut rect = Rectangle::new(418., 231., 50., 100., GREEN);

  loop {
    clear_background(BLACK);

    if is_key_down(KeyCode::Up) {
      rect.y -= speed;
    }

    if is_key_down(KeyCode::Down) {
      rect.y += speed;
    }

    if is_key_down(KeyCode::Left) {
      rect.x -= speed;
    }

    if is_key_down(KeyCode::Right) {
      rect.x += speed;
    }

    if is_key_down(KeyCode::W) {
      circle.y -= speed;
    }

    if is_key_down(KeyCode::S) {
      circle.y += speed;
    }

    if is_key_down(KeyCode::A) {
      circle.x -= speed;
    }

    if is_key_down(KeyCode::D) {
      circle.x += speed;
    }

    // let collision = is_collision(&circle, &rect);
    circle.collision_mode = true;
    rect.collision_mode = true;

    let circle_dbg = format!("{:?}", circle);
    let rect_dbg = format!("{:?}", rect);

    draw_text(&circle_dbg, 10., 20., 16., WHITE);
    draw_text(&rect_dbg, 10., 40., 16., WHITE);

    circle.draw();
    rect.draw();

    let x = circle.x.clamp(rect.x, rect.x + rect.w);
    let y = circle.y.clamp(rect.y, rect.y + rect.h);
    let dist = ((circle.x - x).powi(2) + (circle.y - y).powi(2)).sqrt();
    let dist_dbg = format!("Distance is {}", dist);
    let line_color = if dist <= circle.r { YELLOW } else { BLUE };
    draw_text(&dist_dbg, 10., 60., 16., WHITE);
    draw_line(circle.x, circle.y, x, y, 1., line_color);

    next_frame().await
  }
}
