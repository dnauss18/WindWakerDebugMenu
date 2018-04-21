# The Wind Waker Debug Menu

The Wind Waker Debug Menu Rom Hack adds a Debug Menu with all kinds of useful Menus for editing the game's flags and inventory, activating cheats and warping to any stage in the game.

## How To Compile

To compile the source code you need to get the Rust Nightly compiler toolchain.
You can acquire it either through the [official website](https://www.rust-lang.org/downloads.html).

Keep in mind that we currently need a few unstable features, so you need to install the nightly toolchain.

Since we'll compile PowerPC code, you'll need to get a compiled ```libcore``` for PowerPC.
You can do so by installing the PowerPC target through rustup:

```
rustup target add powerpc-unknown-linux-gnu
```

Now that we have the whole toolchain, you will need to specify the path of your version of Wind Waker (GZLJ01) in the `RomHack.toml`. Alternatively you can copy the game into the main folder as `gzlj01.iso`.

To compile the Debug Menu, execute the following command:

```
cargo run -p compiler --release
```

The compiled ISO is now available in the `target` folder. You can change this location in the `RomHack.toml`.
