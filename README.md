# GW2 ArcDPS Food Reminder
ArcDPS plugin for Guild Wars 2 allowing tracking of buff food & utility items.

## Development
- [ ] Food reminder:
  - [ ] On combat enter with Malnourished/Diminished
  - [ ] After combat exit with Malnourished/Diminished
  - [ ] After death with Malnourished/Diminished
  - [ ] Custom reminder delay (respecting ArcDPS realtime API delta)
- [ ] Food tracking:
  - [ ] Food/Utility tracker table
  - [ ] Tooltips with Food/Utility details
  - [ ] Classification of Food/Utility (`PWR`, `PREC`, `CND`, `EXP`, `CONC`, `HEAL` etc.)
  - [ ] Extensive buff database

## Building from source
You need to have [Rust](https://www.rust-lang.org/learn/get-started) installed.

For standard release verison run `cargo build --release`.

For development you can include a debug log via the `--feature log` flag.
Run `cargo build --release --feature log` for optimized, `cargo build --debug --feature log` for unoptimized.
