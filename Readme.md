# Hapticslib

## Build

#### On Windows
```
rustup target add i686-pc-windows-msvc
cargo build --release
```

#### On Linux (barely tested)

```
rustup target add i686-pc-windows-gnu
cargo build --target i686-pc-windows-gnu --release
```

##### Disclaimer
Requires mingw-w64

### Disclaimer
Testing of builds of this library has **only** been done on Windows so far.
As such that is the only officially supported target.