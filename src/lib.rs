mod data;
mod plugin;
mod reminder;
mod tracking;
mod win;

#[cfg(feature = "demo")]
mod demo;

use arcdps::{arcdps_export, imgui::Ui, Agent, CombatEvent};
use once_cell::sync::Lazy;
use plugin::Plugin;
use std::sync::Mutex;

// create exports for arcdps
arcdps_export! {
    name: "Food Reminder",
    sig: 0x642baaae , // random id
    init,
    release,
    combat,
    imgui,
    options_end,
    options_windows,
    wnd_filter,
}

/// Main plugin instance.
// FIXME: a single mutex for the whole thing is potentially inefficient
static PLUGIN: Lazy<Mutex<Plugin>> = Lazy::new(|| Mutex::new(Plugin::new()));

fn init() {
    PLUGIN.lock().unwrap().load()
}

fn release() {
    PLUGIN.lock().unwrap().unload()
}

fn combat(
    event: Option<&CombatEvent>,
    src: Option<Agent>,
    dest: Option<Agent>,
    skill_name: Option<&str>,
    id: u64,
    revision: u64,
) {
    PLUGIN
        .lock()
        .unwrap()
        .combat_event(event, src, dest, skill_name, id, revision)
}

fn imgui(ui: &Ui, not_loading_or_character_selection: bool) {
    PLUGIN
        .lock()
        .unwrap()
        .render_windows(ui, not_loading_or_character_selection)
}

fn options_windows(ui: &Ui, window_name: Option<&str>) -> bool {
    PLUGIN
        .lock()
        .unwrap()
        .render_window_options(ui, window_name)
}

fn options_end(ui: &Ui) {
    PLUGIN.lock().unwrap().render_options(ui)
}

fn wnd_filter(key: usize, key_down: bool, prev_key_down: bool) -> bool {
    PLUGIN
        .lock()
        .unwrap()
        .key_event(key, key_down, prev_key_down)
}
