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

    let collision = is_collision(&circle, &rect);
    circle.collision_mode = collision;
    rect.collision_mode = collision;

    circle.draw();
    rect.draw();

    next_frame().await
  }
}
