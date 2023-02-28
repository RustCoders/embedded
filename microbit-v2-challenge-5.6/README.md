# Microbit v2 challenge

* This is my solution to this [Discovery Challenge](https://docs.rust-embedded.org/discovery/microbit/05-led-roulette/the-challenge.html). 

For project set up steps, see that discovery challenge or the article (forthcoming).

To build use:

```
cargo build --features v2 --target thumbv7em-none-eabihf
```

To reflash to the microbit, use:

```
cargo embed --features v2 --target thumbv7em-none-eabihf
```

