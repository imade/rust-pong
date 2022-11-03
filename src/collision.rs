use crate::shapes::{Circle, Rectangle};

pub fn is_collision(circle: &Circle, rect: &Rectangle) -> bool {
  let x = circle.x.clamp(rect.x, rect.x + rect.w);
  let y = circle.y.clamp(rect.y, rect.y + rect.h);
  let dist = ((circle.x - x).powi(2) + (circle.y - y).powi(2)).sqrt();
  dist <= circle.r
}
