# Ball Blitz

Ball Blitz is a 3D ball matching game built with the [Bevy game engine](https://github.com/bevyengine/bevy) in Rust. It's a fun matching game based on [Suika game](https://suika-game.app/). 

## Game description

The scene is a 3D transparent box that can be moved with an orbit camera. The player is able to insert a ball of a certain size in the open roof of the box. If two balls of the same type touch, they merge to create a larger ball that is a different type. The current order of balls is 

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

Anything between ping pong and tennis ball will be randomly selected for the player to insert next. The goal of the game is to create a beach ball without having any fall out of the arena.

## Compiling (native)
Currently, the project does not have a WebAssembly port, so you will have to compile it to run it.
1. Clone the repository with `git clone https://github.com/benjamin-cates/ball_blitz`
2. Install the rust compiler toolchain from rustup
2. Compile and run with `cargo run` in the project directory. 
A window with the game will pop up. Note that this does not work in WSL right now because the window manager is buggy. If you have problems with the linker, turn off the "dynamic-linking" feature in `Cargo.toml`, which will make the compilation take much longer but might fix a linker issue.

## Compiling (WebAssembly)
Since we want the wasm binary to be as small as possible, there are several optimizations we need to do in order to make it around 20 MB. 
1. Ensure you have the wasm32 rust toolchain installed
The `--no-default-features` flag disables Bevy dynamic linking, which is not supported on wasm.
2. Ensure you have `wasm-bindgen` installed. If not, this can be installed with `cargo install wasm-bindgen` and then make sure `.cargo/bin` is in your path before running.
3. To optimize code size, ensure you have `wasm-opt` installed. If you don't have it installed, install it with `cargo install wasm-opt`.
4. Run the following commands in this order:
```
cargo build --target wasm32-unknown-unknown --release --no-default-features
wasm-bindgen --no-typescript --target web --out-dir ./out/ ./target/wasm32-unknown-unknown/release/ball_blitz.wasm-opt
wasm-opt -Oz -o out/ball_blitz_bg.wasm out/ball_blitz_bg.wasm
```
First time compilation will take about 10 minutes and the size of the wasm binary at `out/ball_blitz_bg.wasm` should be about 20 MB.
5. Host the project directory over http and open localhost in the browser.

