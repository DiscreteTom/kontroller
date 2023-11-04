# kontroller

[![version](https://img.shields.io/github/v/tag/DiscreteTom/kontroller?label=release&style=flat-square)](https://github.com/DiscreteTom/kontroller/releases/latest)
![license](https://img.shields.io/github/license/DiscreteTom/kontroller?style=flat-square)
[![rust](https://img.shields.io/badge/built_with-rust-DEA584?style=flat-square)](https://github.com/DiscreteTom/retsac)

A demo project to read all SteamDeck input (including trackpad and gyro) via SteamInput without using a game engine.

![demo](./img/demo.gif)

## Run

1. Switch to Desktop Mode on SteamDeck.
2. Download `kontroller` from [release page](https://github.com/DiscreteTom/kontroller/releases/latest), or [build](#build) it yourself. Put it in SteamDeck.
3. Put `redistributable_bin/libsteam_api.so` in the PATH (e.g. `/usr/lib`).
4. Copy `kontroller.vdf` into `/home/deck/.local/share/Steam/controller_config` (create the folder if it doesn't exist), rename it to `game_actions_480.vdf`.
5. Run `kontroller` on SteamDeck. This should open a new window, but your input can't be captured now. Close the window.
6. Start steam client on the Desktop Mode. In your library, you should find a game called `Spacewar`. [Edit it's input mapping](https://partner.steamgames.com/doc/features/steam_controller/getting_started_for_devs#14).
7. Run `kontroller` again, you should get the input you want.

## Build

> **Note**
> You'd better build the project on SteamDeck.

```bash
cargo build --release
```

## FAQ

- Why not to use a game engine?
  - This app doesn't need high quality UI update, but need to read input as fast as possible.
  - Besides, I want the built binary to be as small as possible.
- What UI library do you use?
  - [iced](https://github.com/iced-rs/iced).
  - Steam Input requires a window to be focused to read input, so a UI library is required.
  - WebView-based UI lib seems not working. Tested with [Tauri](https://tauri.app/) and [web-view](https://github.com/Boscop/web-view)
  - The UI process should be the same as the main process.
- Why the name is `Spacewar` and the app id is 480?
  - 480 is a commonly used app id for testing for steamworks developers. The name `Spacewar` is the name for the app with app id 480.
- I tried to start the executable file on SteamDeck but no window shows up.
  - Make sure you have `libsteam_api.so` in the PATH.
  - Start the executable in a terminal and show me the output by opening an issue.
