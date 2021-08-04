# GW2 ArcDPS Food Reminder
ArcDPS plugin for Guild Wars 2 allowing tracking of buff food & utility items.

![Reminder screenshot](./screenshots/reminder.png)
![Tracker screenshot](./screenshots/tracker.png)

## Development progress
- [x] Food reminder
  - [x] On encounter start
  - [x] After encounter end
  - [ ] After death
- [x] Food tracking
  - [x] Food/Utility tracker table
  - [x] Classification of Food/Utility
  - [x] Tooltips with Food/Utility details
  - [x] Context menu to copy names/ids
  - [ ] Table sorting
- [ ] Settings
  - [x] Save window states
  - [ ] Custom hotkeys
  - [ ] Reminder on Malnourished/Diminished vs. none
  - [ ] Custom reminder delay
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
Run `cargo build --release --feature log` for optimized, `cargo build --feature log` for unoptimized.
