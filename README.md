# GTA Tools

A toolset of convenient things for GTA V Online.

![](https://i.vgy.me/aQ1qgw.png)

## Installing

**Option 1** — <ins>Download</ins>  
Download the latest release [here](https://codeberg.org/futile/gta-tools/releases/download/latest/gta-tools.exe) and place it somewhere convenient for you, such as Documents. You could then make a shortcut titled "GTA Tools", and pin it to taskbar or Start.

**Option 2** — <ins>Build from source</ins>  
You will need the Rust toolchain, which can be obtained [here](https://rustup.rs). Follow the instructions of its installer. Once you have Rust installed, clone this repo and navigate to it. At this point, you should probably `git checkout x.x.x`, where `x.x.x` is the latest tag. You can then run `cargo build --release`. Once you do that, you can use the binary located at `.\target\release\gta-tools.exe` in the same way as **Option 1**.
