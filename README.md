# GW2 ArcDPS Food Reminder
ArcDPS plugin for Guild Wars 2 allowing tracking of buff food & utility items.

## Development progress
- [ ] Food reminder when Malnourished/Diminished
  - [ ] On combat enter
  - [ ] After combat exit
  - [ ] After death
- [x] Food tracking
  - [x] Food/Utility tracker table
  - [x] Tooltips with Food/Utility details
  - [x] Classification of Food/Utility
- [ ] Settings
  - [ ] Save window states
  - [ ] Custom reminder delay
  - [ ] Custom Food/Utility definitions

## Buff database
Known Food & Utility buffs are currently collected in [data/buffs](./data/buffs).

## Building from source
You need to have [Rust](https://www.rust-lang.org/learn/get-started) installed.

For the standard release version run `cargo build --release`.

For development you can include a debug log via the `--feature log` flag.
Run `cargo build --release --feature log` for optimized, `cargo build --debug --feature log` for unoptimized.
