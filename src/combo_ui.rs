use arcdps::imgui::{Selectable, Ui};
use std::cmp::PartialEq;
use strum::IntoEnumIterator;

/// Renders a combo box for items from an iterator.
pub fn render_combo<T>(
    ui: &Ui,
    label: impl AsRef<str>,
    all: impl Iterator<Item = T>,
    current: &mut T,
) -> bool
where
    T: PartialEq + AsRef<str>,
{
    let mut changed = false;
    if let Some(_token) = ui.begin_combo(label, &current) {
        for entry in all {
            let selected = entry == *current;
            if Selectable::new(&entry).selected(selected).build(ui) {
                changed = true;
                *current = entry;
            }

            // handle focus
            if selected {
                ui.set_item_default_focus();
            }
        }
    }
    changed
}

/// Renders a combo box for an enum implementing [`IntoEnumIterator`].
pub fn render_enum_combo<T>(ui: &Ui, label: impl AsRef<str>, current: &mut T) -> bool
where
    T: PartialEq + AsRef<str> + IntoEnumIterator,
{
    render_combo(ui, label, T::iter(), current)
}
