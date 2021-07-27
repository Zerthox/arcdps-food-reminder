pub mod arc_util;
pub mod plugin;
pub mod reminder;
pub mod tracking;
pub mod ui;
pub mod win;

#[cfg(feature = "demo")]
pub mod demo;

#[cfg(feature = "log")]
pub mod log;

use arcdps::{arcdps_export, imgui::Ui, Agent, CombatEvent};
use lazy_static::lazy_static;
use plugin::Plugin;
use std::sync::Mutex;

// create exports for arcdps
arcdps_export! {
    name: "food-reminder",
    sig: 0x642baaae , // random id
    init,
    release,
    combat,
    imgui,
    options_windows,
    wnd_filter,
}

lazy_static! {
    // FIXME: a single mutex for the whole thing is potentially inefficient
    /// Main plugin instance.
    static ref PLUGIN: Mutex<Plugin> = Mutex::new(Plugin::new());
}

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
    PLUGIN.lock().unwrap().render_options(ui, window_name)
}

fn wnd_filter(key: usize, key_down: bool, prev_key_down: bool) -> bool {
    PLUGIN
        .lock()
        .unwrap()
        .key_event(key, key_down, prev_key_down)
}
