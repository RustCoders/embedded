Instead of command given [here](https://docs.rust-embedded.org/discovery/microbit/03-setup/verify.html), use this:

```
lsusb | grep NXP
```

I got this output:

```
Bus 001 Device 014: ID 0d28:0204 NXP LPC1768
```

To install cargo embed I needed:

```
sudo apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev
sudo apt-get install libudev-dev
cargo install cargo-embed
```

Also needed ? 

```
cargo install cargo-binutils
```

Also very important:

```
rustup target add thumbv7em-none-eabihf
```