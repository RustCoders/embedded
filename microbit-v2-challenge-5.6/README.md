# Microbit v2 challenge

* This is my solution to this [Discovery Challenge](https://docs.rust-embedded.org/discovery/microbit/05-led-roulette/the-challenge.html). 

For complete project set up steps, see the article:  [Embedded Rust Development on Apple Silicon (Mac M1)](https://rustassured.com/embedded-rust-development-on-apple-silicon-mac-m1/)

To build use:

```
cargo build --features v2 --target thumbv7em-none-eabihf
```

To reflash to the microbit, use:

```
cargo embed --features v2 --target thumbv7em-none-eabihf
```

