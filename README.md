# rust-pong
Simple 2D retro Pong game written in Rust language using [macroquad](https://github.com/not-fl3/macroquad) library

## Local development setup
```sh
git clone git@github.com:imade/rust-pong.git && cd rust-pong && cargo run --release
```

## Troubleshooting

### Linking with cc failed

On Linux you might see the following error when you're trying to `cargo run` this project:
```sh
error: linking with `cc` failed: exit status: 1
```

To fix this issue you need to install additional OS libraries as mentioned in [macroquad readme document](https://github.com/not-fl3/macroquad#linux).
