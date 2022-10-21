# rust-pong
Simple 2D retro Pong game written in Rust language using [macroquad](https://github.com/not-fl3/macroquad) library

## Local setup
```sh
git clone git@github.com:imade/rust-pong.git && cd rust-pong && cargo run --release
```

⚠️ If you encounter errors with the `cargo` command then:

1. If you are on Linux then check that you have [installed the necessary libraries](https://github.com/not-fl3/macroquad#linux) that macroquad needs.
2. If you are on Debian / Ubuntu-like distribution then also install `build-essential`
    ```sh
    sudo apt-get install build-essential
    ```
3. If you are on Fedora then install `"Development Tools"`
    ```
    sudo dnf groupinstall "Development Tools"
    ```
