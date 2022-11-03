use macroquad::color::Color;
use macroquad::shapes::draw_circle;
use macroquad::shapes::draw_circle_lines;
use macroquad::shapes::draw_rectangle;
use macroquad::shapes::draw_rectangle_lines;
use std::fmt;

pub struct Circle {
  pub x: f32,
  pub y: f32,
  pub r: f32,
  pub color: Color,
  pub collision_mode: bool,
}

impl Circle {
  pub fn new(x: f32, y: f32, r: f32, color: Color) -> Self {
    Self {
      x,
      y,
      r,
      color,
      collision_mode: false,
    }
  }

  pub fn draw(&self) {
    if self.collision_mode {
      draw_circle_lines(self.x, self.y, self.r, 1., self.color);
    } else {
      draw_circle(self.x, self.y, self.r, self.color);
    }
  }
}

impl fmt::Debug for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Circle: x={} y={}, collision={}",
      self.x, self.y, self.collision_mode
    )
  }
}

pub struct Rectangle {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
  pub color: Color,
  pub collision_mode: bool,
}

impl Rectangle {
  pub fn new(x: f32, y: f32, w: f32, h: f32, color: Color) -> Self {
    Self {
      x,
      y,
      w,
      h,
      color,
      collision_mode: false,
    }
  }

  pub fn draw(&self) {
    if self.collision_mode {
      draw_rectangle_lines(self.x, self.y, self.w, self.h, 1., self.color);
    } else {
      draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }
  }
}

impl fmt::Debug for Rectangle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Rect: x={} y={}, collision={}",
      self.x, self.y, self.collision_mode
    )
  }
}
