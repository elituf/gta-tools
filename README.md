# GTA Tools

A toolset of convenient things for GTA V Online.

<picture>
    <source srcset="https://i.vgy.me/M4sOHh.png" media="(prefers-color-scheme: dark)">
    <img src="https://i.vgy.me/mpO9uc.png">
</picture>

## Installing

**Option 1** — <ins>Download</ins>  
Download the latest release [here](https://github.com/elituf/gta-tools/releases/latest/download/gta-tools.exe) and place it somewhere convenient for you, such as Documents. You could then make a shortcut titled "GTA Tools", and pin it to taskbar or Start.

**Option 2** — <ins>Build from source</ins>  
You will need the Rust toolchain, which can be obtained [here](https://rustup.rs). Follow the instructions of its installer. Once you have Rust installed, clone this repo and navigate to it. At this point, you should probably `git checkout x.x.x`, where `x.x.x` is the latest tag. You can then run `cargo build --release`. Once you do that, you can use the binary located at `.\target\release\gta-tools.exe` in the same way as **Option 1**.

## Guide

Every feature of GTA Tools is Legacy/Enhanced-agnostic. Some functionality of GTA Tools requires administrator access. If necessary, GTA Tools can either be started as admin manually, or, the user can simply use the <kbd>Elevate</kbd> button to relaunch GTA Tools as admin.

It is recommended to always use an up-to-date version of GTA Tools from [releases](https://github.com/elituf/gta-tools/releases). You can also easily access this repository by going to the <kbd>About</kbd> page of GTA Tools and clicking the GitHub button beside the version number.

#### Game

This section is quite simple.

There is a <kbd>Launch</kbd> feature, which will start your game on the chosen launcher and game version.

There is also a <kbd>Force close game</kbd> feature, which simply kills all game processes. This button requires a second press after the first one for confirmation. This *does not* touch Rockstar Games Launcher or any other processes, only ones named `GTA5_Enhanced.exe` or `GTA5.exe`.

#### Session

This section also has two features.

The first being <kbd>Empty current session</kbd>, which "removes everyone" from your session by suspending your game for 10 seconds, and then resuming it. This method is exceedingly similar to the Resource Monitor method, but made convenient in one button. This can be useful in numerous ways:
* You are stuck loading into a session
* You are stuck loading into an interior
* You are about to get griefed while doing a freemode mission
* You want to quickly get a "new" session

The other feature is <kbd>Anti AFK</kbd>, which, when toggled, prevents you from getting kicked for idling by pressing keys on your keyboard every 60 seconds. At present, those keys are `VK_NUMPAD4` & `VK_NUMPAD6`, due to being a relatively good balance of keys that the game actually registers but don't interrupt gameplay too much if they activate while you're playing. You *must* be tabbed into GTA V for <kbd>Anti AFK</kbd> to work. While toggled, <kbd>Anti AFK</kbd> will only activate when all of the following conditions are true:
* The `Grand Theft Auto V` window is currently focused
* None of the designated keys are currently being pressed[^1]
* The mouse cursor is not currently visible[^2]

#### Network

The last section has a single feature, <kbd>Game's network access</kbd>, with two buttons, <kbd>Block</kbd> and <kbd>Unblock</kbd>, and a coloured indicator dot. This feature requires administrator, and blocks/unblocks GTA V's entire internet access using the Windows Firewall. The coloured indicator dot has these meanings:
* **Green** - the game is connected to the internet
* **Yellow** - the game could not be found running, and thus was not blocked
* **Red** - the game is blocked from the internet

This feature is primarily useful for **replay glitching**, which is an exploit that allows you to complete any heist/mission finale, get the money for it, and then be able to do it again right away. Here's how you can do a replay glitch:
1. Get to the finale of a heist/contract/mission series, such as **The Contract: Dr. Dre**
2. Complete the finale as normal
3. Right *after* the **HEIST/MISSION PASSED** screen you get when you complete a finale (basically, when your cut is being shown), <kbd>Block</kbd> the game's internet access
4. After a few seconds, you will be shown an error screen and kicked to the main menu
5. At this point, <kbd>Unblock</kbd> the game's internet access and load back into online
6. Finally, spend any amount of money, and you should receive the finale cut, but also be able to go play the finale again immediately

## Issues

- I have noticed that on my current Windows 10 install, when not elevated (administrator), the <kbd>Force close game</kbd> and <kbd>Empty current session</kbd> features can fail due to being denied access. This was tested by other people and is not guaranteed to happen. If this does happen to you, I recommend using GTA Tools always in elevated mode. Check "Always start elevated" in the Settings tab.



[^1]: Otherwise, if you are, for example, using those keys to fly a plane, <kbd>Anti AFK</kbd> activating could cancel the input.
[^2]: Specifically, the Windows mouse cursor, not the in-game mouse cursor. This check is mostly used to prevent typing while you are in the Rockstar overlay.
