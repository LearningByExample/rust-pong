# rust-pong
Amethyst Pong Tutorial

## Info

This project follows the Amethyst pong tutorial :

https://book.amethyst.rs/master/pong-tutorial.html

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```
