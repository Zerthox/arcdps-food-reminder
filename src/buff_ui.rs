use crate::data::BuffData;
use arc_util::ui::{render, Ui};
use arcdps::{
    exports::{self, CoreColor},
    imgui::{Selectable, StyleColor},
};

/// Renders a tooltip for a buff.
pub fn render_buff_tooltip(ui: &Ui, buff: &BuffData) {
    if ui.is_item_hovered() {
        ui.tooltip(|| {
            match buff.rarity.color() {
                Some(color) => ui.text_colored(color, &buff.name),
                None => ui.text(&buff.name),
            }
            if !buff.stats.is_empty() {
                ui.text(buff.stats.join("\n"));
            }
        });
    }
}

/// Renders a context menu for a buff.
pub fn render_buff_context_menu(
    ui: &Ui,
    menu_id: impl Into<String>,
    title: &str,
    buff_id: u32,
    name: Option<&str>,
    colors: &exports::Colors,
) {
    render::item_context_menu(menu_id, || {
        match colors.core(CoreColor::MediumGrey) {
            Some(color) => ui.text_colored(color, title),
            None => ui.text(title),
        }
        if let Some(name) = name {
            if ui.small_button("Copy name") {
                ui.set_clipboard_text(name);
            }
            if ui.small_button("Open wiki") {
                let _ = open::that(format!(
                    "https://wiki-en.guildwars2.com/wiki/Special:Search/{name}"
                ));
            }
        }
        if ui.small_button("Copy id") {
            ui.set_clipboard_text(buff_id.to_string());
        }
    });
}

/// Renders a context menu for a food item.
pub fn render_food_context_menu(
    ui: &Ui,
    menu_id: usize,
    buff_id: u32,
    name: Option<&str>,
    colors: &exports::Colors,
) {
    render_buff_context_menu(
        ui,
        format!("##food-context-{menu_id}"),
        "Food options",
        buff_id,
        name,
        colors,
    )
}

/// Renders a context menu for a utility item.
pub fn render_util_context_menu(
    ui: &Ui,
    menu_id: usize,
    buff_id: u32,
    name: Option<&str>,
    colors: &exports::Colors,
) {
    render_buff_context_menu(
        ui,
        format!("##util-context-{menu_id}"),
        "Utility options",
        buff_id,
        name,
        colors,
    )
}

/// Renders a combo for buffs.
pub fn render_buff_combo<'b>(
    ui: &Ui,
    label: impl AsRef<str>,
    selected_id: u32,
    buffs: impl Iterator<Item = &'b BuffData> + Clone,
) -> Option<&'b BuffData> {
    let preview = buffs
        .clone()
        .find(|entry| entry.id == selected_id)
        .map(|buff| buff.name.clone())
        .unwrap_or_default();

    // TODO: search?
    let mut result = None;
    if let Some(_token) = ui.begin_combo(label, preview) {
        for entry in buffs {
            let selected = entry.id == selected_id;

            let style = entry
                .rarity
                .color()
                .map(|color| ui.push_style_color(StyleColor::Text, color));
            if Selectable::new(&entry.name).selected(selected).build(ui) {
                result = Some(entry);
            }
            drop(style);

            // handle focus
            if selected {
                ui.set_item_default_focus();
            }

            // tooltip
            if ui.is_item_hovered() {
                render_buff_tooltip(ui, entry);
            }
        }
    }
    result
}
