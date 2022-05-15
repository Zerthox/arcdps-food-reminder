use crate::data::BuffData;
use arc_util::ui::{render, Ui};

/// Renders a tooltip for a buff.
pub fn render_buff_tooltip(ui: &Ui, buff: &BuffData) {
    if ui.is_item_hovered() {
        ui.tooltip_text(format!("{}\n{}", buff.name, buff.stats.join("\n")));
    }
}

/// Renders a context menu for a buff.
pub fn render_buff_context_menu(
    ui: &Ui,
    menu_id: impl Into<String>,
    title: &str,
    buff_id: u32,
    name: Option<&str>,
) {
    render::item_context_menu(menu_id, || {
        ui.text(title);
        if let Some(name) = name {
            if ui.small_button("Copy Name") {
                ui.set_clipboard_text(name);
            }
            if ui.small_button("Open Wiki") {
                let _ = open::that(format!(
                    "https://wiki-en.guildwars2.com/wiki/Special:Search/{}",
                    name
                ));
            }
        }
        if ui.small_button("Copy ID") {
            ui.set_clipboard_text(buff_id.to_string());
        }
    });
}

/// Renders a context menu for a food item.
pub fn render_food_context_menu(ui: &Ui, menu_id: usize, buff_id: u32, name: Option<&str>) {
    render_buff_context_menu(
        ui,
        format!("##food-context-{}", menu_id),
        "Food Options",
        buff_id,
        name,
    )
}

/// Renders a context menu for a utility item.
pub fn render_util_context_menu(ui: &Ui, menu_id: usize, buff_id: u32, name: Option<&str>) {
    render_buff_context_menu(
        ui,
        format!("##util-context-{}", menu_id),
        "Utility Options",
        buff_id,
        name,
    )
}

/// Renders a combo for buffs.
pub fn render_buff_combo<'b>(
    ui: &Ui,
    label: impl AsRef<str>,
    selected_id: u32,
    buffs: &[&'b BuffData],
) -> Option<&'b BuffData> {
    let mut index = buffs
        .iter()
        .position(|entry| entry.id == selected_id)
        .unwrap();
    if ui.combo(label, &mut index, buffs, |buff| buff.name.as_str().into()) {
        Some(buffs[index])
    } else {
        None
    }
}
