# GW2 ArcDPS Food Reminder
[ArcDPS](https://deltaconnected.com/arcdps) plugin for [Guild Wars 2](https://guildwars2.com) allowing tracking of buff food & utility items.

Published releases can be found [here](../../releases). Click [here](../../releases/latest/download/arcdps_food_reminder.dll) to directly download the latest release.

![Reminder screenshot](./screenshots/reminder.png)
![Tracker screenshot](./screenshots/tracker.png)

## Development progress
- [x] Food reminder
  - [x] Display onscreen
  - [ ] Notification sound
- [x] Food tracking
  - [x] Food/Utility tracker table
  - [x] Classification of Food/Utility
  - [x] Tooltips with Food/Utility details
  - [x] Context menu to copy names/ids
  - [x] Table sorting
- [x] Settings
  - [x] Save window states
  - [x] Custom hotkeys
  - [x] Reminder customization
  - [ ] Custom Food/Utility definitions

## Buff database
Known Food & Utility buffs are currently collected in [data/buffs](./data/buffs).

**Found an unknown Food/Utility buff that you would like to see added?**  
Right click the entry in the tracker and select `Copy ID`.
Report the copied buff ID & the Food/Utility that applied it.

![Reporting unknown buff](./screenshots/unknown.png)

## Building from source
You need to have [Rust](https://www.rust-lang.org/learn/get-started) installed.

For the standard release version run `cargo build --release`.
For development you can include a debug log via the `--feature log` flag.

There is also a makefile provided for use with [cargo-make](https://github.com/sagiegurari/cargo-make).
To build & install the plugin run `cargo make install`.
You can provide a custom Guild Wars 2 installation path via the `GW2_PATH` environment variable.
