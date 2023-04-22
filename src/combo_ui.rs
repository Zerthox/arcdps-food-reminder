use std::borrow::Cow;

use crate::data::PROFESSIONS;
use arc_util::ui::render;
use arcdps::{
    exports,
    imgui::{Selectable, StyleColor, Ui},
    Profession,
};
use strum::IntoEnumIterator;

/// Renders a combo box for items from an iterator.
pub fn render_combo<T>(
    ui: &Ui,
    label: impl AsRef<str>,
    all: impl Iterator<Item = T>,
    current: &mut T,
    item_label: impl for<'a> Fn(&'a T) -> Cow<'a, str>,
    item_color: impl Fn(&T) -> Option<[f32; 4]>,
) -> bool
where
    T: PartialEq,
{
    let mut changed = false;

    if let Some(_token) = ui.begin_combo(label, item_label(current)) {
        for entry in all {
            let selected = entry == *current;

            // apply color to selectable
            let style =
                item_color(&entry).map(|color| ui.push_style_color(StyleColor::Text, color));
            if Selectable::new(item_label(&entry))
                .selected(selected)
                .build(ui)
            {
                changed = true;
                *current = entry;
            }
            drop(style);

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
    render_combo(
        ui,
        label,
        T::iter(),
        current,
        |item| item.as_ref().into(),
        |_| None,
    )
}

/// Renders a combo box for selecting a [`Profession`].
pub fn render_prof_select(ui: &Ui, label: impl AsRef<str>, current: &mut Profession) -> bool {
    let colors = exports::colors();

    render_combo(
        ui,
        label,
        PROFESSIONS.iter().cloned(),
        current,
        |prof| <&str>::from(*prof).into(),
        |prof| {
            colors
                .prof_base(*prof)
                .map(|color| render::with_alpha(color, 1.0))
        },
    )
}
