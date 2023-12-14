mod assets;
mod buff_ui;
mod builds;
mod combo_ui;
mod data;
mod plugin;
mod reminder;
mod tracking;
mod util;

#[cfg(feature = "demo")]
mod demo;

use arcdps::{
    extras::{ExtrasAddonInfo, UserInfoIter},
    imgui::Ui,
    Agent, Event,
};
use plugin::Plugin;

// create exports for arcdps
arcdps::export! {
    name: "Food Reminder",
    sig: 0x642baaae, // random id
    init,
    release,
    combat,
    imgui,
    options_end,
    options_windows,
    wnd_filter,
    extras_init,
    extras_squad_update,
}

fn init() -> Result<(), String> {
    // TODO: use error
    Plugin::lock().load();
    Ok(())
}

fn release() {
    Plugin::lock().unload()
}

fn combat(
    event: Option<&Event>,
    src: Option<&Agent>,
    dest: Option<&Agent>,
    skill_name: Option<&str>,
    id: u64,
    revision: u64,
) {
    Plugin::area_event(event, src, dest, skill_name, id, revision)
}

fn imgui(ui: &Ui, not_loading_or_character_selection: bool) {
    Plugin::lock().render_windows(ui, not_loading_or_character_selection)
}

fn options_windows(ui: &Ui, window_name: Option<&str>) -> bool {
    Plugin::render_window_options(ui, window_name)
}

fn options_end(ui: &Ui) {
    Plugin::lock().render_settings(ui)
}

fn wnd_filter(key: usize, key_down: bool, prev_key_down: bool) -> bool {
    Plugin::key_event(key, key_down, prev_key_down)
}

fn extras_init(addon_info: ExtrasAddonInfo, account_name: Option<&str>) {
    Plugin::lock().extras_init(addon_info, account_name)
}

fn extras_squad_update(users: UserInfoIter) {
    Plugin::lock().extras_squad_update(users)
}
