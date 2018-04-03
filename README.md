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

We'll need to target the GameCube specifically, so we'll need a special linker which you can get by installing [DevkitPPC](http://devkitpro.org/wiki/Getting_Started/devkitPPC).

Now that we have the whole toolchain, you will need to unpack your version of Wind Waker (GZLJ01) into the folder called ```game```.
You can use the [GameCube ISO Tool](http://www.wiibackupmanager.co.uk/gcit.html) for that.
The ```game``` folder should contain the following folders if done correctly: ```root``` and ```sys```.
In the ```sys``` folder you can find a ```main.dol```.
This is the main executable of the game and will be the one we compile into.
We'll need to create a backup of the file called ```original.dol``` that you put directly into the game folder.

The folder structure should look like this now:

 - game
   - sys
   - root
   - original.dol
 - libtww
 - patcher
 - src
 - ...

The compiled executable will be located in the ```game/sys/``` folder.

You can use the GameCube ISO Tool or run `make iso` to convert this into an ISO or directly boot up the folder with Dolphin.
