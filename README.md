# GW2 ArcDPS Food Reminder
ArcDPS plugin for Guild Wars 2 allowing tracking of buff food & utility items.

![Tracker screenshot](./screenshots/tracker.png)

## Development progress
- [ ] Food reminder
  - [ ] On combat enter
  - [ ] After combat exit
  - [ ] After death
- [x] Food tracking
  - [x] Food/Utility tracker table
  - [x] Classification of Food/Utility
  - [x] Tooltips with Food/Utility details
  - [ ] Table sorting
- [ ] Settings
  - [ ] Save window states
  - [ ] Reminder on Malnourished/Diminished vs. none
  - [ ] Custom reminder delay
  - [ ] Custom Food/Utility definitions

## Buff database
Known Food & Utility buffs are currently collected in [data/buffs](./data/buffs).

## Building from source
You need to have [Rust](https://www.rust-lang.org/learn/get-started) installed.

For the standard release version run `cargo build --release`.

For development you can include a debug log via the `--feature log` flag.
Run `cargo build --release --feature log` for optimized, `cargo build --debug --feature log` for unoptimized.
