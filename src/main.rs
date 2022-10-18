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
const RECT_Y_CENTERED: f32 = WINDOW_HEIGHT_HALF - RECT_HEIGHT_HALF;
const RECT_SPEED: f32 = 10.;

// Ball constants:
const BALL_RADIUS: f32 = 25.;

struct Ball {
  x: f32,
  y: f32,
  direction_x: f32,
  direction_y: f32,
  speed: f32,
}

impl Ball {
  fn new() -> Self {
    Self {
      x: WINDOW_WIDTH_HALF,
      y: WINDOW_HEIGHT_HALF,
      direction_x: 1.,
      direction_y: 1.,
      speed: 0.,
    }
  }

  fn kick_off(&mut self) {
    // TODO make ball kick-off direction random
    self.speed = 5.;
  }

  fn reset(&mut self) {
    self.x = WINDOW_WIDTH_HALF;
    self.y = WINDOW_HEIGHT_HALF;
    self.direction_x = 1.;
    self.direction_y = 1.;
    self.speed = 0.;
  }

  fn inside_player_y_coords(y: f32, p: &Player) -> bool {
    y > p.y && y < p.y + RECT_HEIGHT
  }

  fn collides_left_player(x: f32, y: f32, p: &Player) -> bool {
    x - BALL_RADIUS < p.x + RECT_WIDTH && Ball::inside_player_y_coords(y, p)
  }

  fn collides_right_player(x: f32, y: f32, p: &Player) -> bool {
    x + BALL_RADIUS > p.x && Ball::inside_player_y_coords(y, p)
  }

  fn collides_upper_wall(y: f32) -> bool {
    y - BALL_RADIUS < 0.
  }

  fn collides_lower_wall(y: f32) -> bool {
    y + BALL_RADIUS > WINDOW_HEIGHT
  }

  fn collides_edge(x: f32) -> bool {
    x - BALL_RADIUS < 0. || x + BALL_RADIUS > WINDOW_WIDTH
  }

  fn update(&mut self, player_left: &Player, player_right: &Player) {
    let x = self.x + (self.direction_x * self.speed);
    let y = self.y + (self.direction_y * self.speed);

    if Ball::collides_left_player(x, y, player_left) {
      self.direction_x = 1.;
      self.x = player_left.x + RECT_WIDTH + BALL_RADIUS;
    } else if Ball::collides_right_player(x, y, player_right) {
      self.direction_x = -1.;
      self.x = player_right.x - BALL_RADIUS;
    } else if Ball::collides_upper_wall(y) {
      self.direction_y = 1.;
      self.y = BALL_RADIUS;
    } else if Ball::collides_lower_wall(y) {
      self.direction_y = -1.;
      self.y = WINDOW_HEIGHT - BALL_RADIUS;
    } else if Ball::collides_edge(x) {
      self.speed = 0.;
      self.direction_x = 0.;
      self.direction_y = 0.;
      return;
    } else {
      self.y = y;
      self.x = x;
    }
  }

  fn draw(&self) {
    draw_circle(self.x, self.y, BALL_RADIUS, WHITE);
  }
}

struct Player {
  x: f32,
  y: f32,
}

impl Player {
  fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }

  fn left() -> Self {
    Self::new(RECT_PADDING, RECT_Y_CENTERED)
  }

  fn right() -> Self {
    Self::new(WINDOW_WIDTH - RECT_WIDTH - RECT_PADDING, RECT_Y_CENTERED)
  }

  fn update(&mut self, direction: f32) {
    if direction == 0. {
      return;
    }
    let new_y = self.y + (RECT_SPEED * direction);
    self.y = if new_y <= RECT_PADDING {
      RECT_PADDING
    } else if new_y + RECT_HEIGHT >= WINDOW_HEIGHT - RECT_PADDING {
      WINDOW_HEIGHT - RECT_HEIGHT - RECT_PADDING
    } else {
      new_y
    };
  }

  fn draw(&self) {
    draw_rectangle(self.x, self.y, RECT_WIDTH, RECT_HEIGHT, WHITE);
  }
}

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
  let mut player_left = Player::left();
  let mut player_right = Player::right();
  let mut ball = Ball::new();
  ball.kick_off();

  loop {
    if is_key_down(KeyCode::R) {
      ball.reset();
      ball.kick_off();
    }

    // Detect player left direction changes
    let left_direction = if is_key_down(KeyCode::S) {
      1.
    } else if is_key_down(KeyCode::W) {
      -1.
    } else {
      0.
    };

    // Detect player right direction changes
    let right_direction: f32 = if is_key_down(KeyCode::Down) {
      1.
    } else if is_key_down(KeyCode::Up) {
      -1.
    } else {
      0.
    };

    clear_background(GREEN);

    draw_line(
      WINDOW_WIDTH_HALF,
      0.,
      WINDOW_WIDTH_HALF,
      WINDOW_HEIGHT,
      10.0,
      WHITE,
    );

    player_left.update(left_direction);
    player_right.update(right_direction);
    ball.update(&player_left, &player_right);

    player_left.draw();
    player_right.draw();
    ball.draw();

    next_frame().await
  }
}
