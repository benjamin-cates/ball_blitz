# [Ball Blitz](https://benjamin-cates.github.io/ball_blitz)

View the project hosted [here](https://benjamin-cates.github.io/ball_blitz)!

Ball Blitz is a 3D ball matching game built with the [Bevy game engine](https://github.com/bevyengine/bevy) in Rust. It was originally based on [Suika game](https://suika-game.app/), but uses a sports ball theme and is in 3D. 

## Game description

The main play area has a transparent box that has an orbit camera around it. The player gets balls of random size from ping pong to tennis ball that they can spawn at the top of the box. If two balls of the same type touch, they merge to create the next largest ball. The current order of balls is:

1. Ping Pong ball
2. Golf ball
3. Billiards ball
4. Tennis ball
5. Baseball
6. Bowling ball
7. Soccer ball
8. Basketball
9. Beach ball
10. ??? (to be added later)


Points are gained when spawning balls and when merging balls, and points are lost when balls don't fit in the box and fall. The goal of the game is to create a beach ball without going into negative points.

## Compiling (native)
1. Clone the repository with `git clone https://github.com/benjamin-cates/ball_blitz`
2. Install the rust compiler toolchain from rustup
2. Compile and run with `cargo run` in the project directory. 
A window with the game will pop up. Note that if you are on WSL also follow the WSL instructions If you have problems with the linker, build with `cargo run --no-default-features` to turn off the "dynamic-linking", which will make the compilation take much longer but might fix a linker issue.

## Compiling (WebAssembly)
Note: WebAssembly compilation should happen on the `web` branch. In order to prevent large git folders, delete the web branch and create a new `web` branch from `main` every time there is a release.

Since we want the wasm binary to be as small as possible, there are several optimizations we need to do in order to make it around 20 MB. 
1. Ensure you have the wasm32 rust toolchain installed
The `--no-default-features` flag disables Bevy dynamic linking, which is not supported on wasm.
2. Ensure you have `wasm-bindgen` installed. If not, this can be installed with `cargo install wasm-bindgen` and then make sure `.cargo/bin` is in your path before running.
3. To optimize code size, ensure you have `wasm-opt` installed. If you don't have it installed, install it with `cargo install wasm-opt`.
4. Run the following commands in this order:
```
cargo build --target wasm32-unknown-unknown --release --no-default-features
wasm-bindgen --no-typescript --target web --out-dir ./out/ ./target/wasm32-unknown-unknown/release/ball_blitz.wasm
wasm-opt -Oz -o out/ball_blitz_bg.wasm out/ball_blitz_bg.wasm
```
First time compilation will take about 10 minutes and the size of the wasm binary at `out/ball_blitz_bg.wasm` should be about 20 MB.
5. Host the project directory over http and open localhost in the browser.

## Additional dependencies for compiling on WSL Ubuntu

You'll need the `alsa` audio library and `libudev-dev`.
```bash
sudo apt install librust-alsa-sys-dev libudev-dev
```

You'll also need to update mesa
```bash
sudo add-apt-repository ppa:kisak/kisak-mesa
sudo apt update
sudo apt upgrade
```
