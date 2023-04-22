use super::Demo;
use crate::{
    buff_ui::render_buff_tooltip,
    combo_ui::render_prof_select,
    data::Definitions,
    tracking::{
        buff::{BuffState, TrackedBuff},
        ui::Props as TrackerProps,
    },
};
use arc_util::{
    tracking::{Entry, Player},
    ui::{render, Component, Hideable, Windowable},
};
use arcdps::{
    imgui::{Selectable, StyleColor, TableColumnSetup, Ui},
    Profession, Specialization,
};
use std::borrow::Cow;

pub type Props<'p> = TrackerProps<'p>;

const SPECIAL_BUFFS: [BuffState<u32>; 3] =
    [BuffState::Unknown, BuffState::None, BuffState::Some(0)];

impl Demo {
    fn render_combo<'b>(
        ui: &Ui,
        defs: &Definitions,
        label: impl AsRef<str>,
        all: &'b [BuffState<u32>],
        current: &mut BuffState<u32>,
        item_label: impl Fn(&BuffState<u32>) -> Cow<str>,
    ) -> Option<&'b BuffState<u32>> {
        let mut result = None;
        if let Some(_token) = ui.begin_combo(label, item_label(current)) {
            for entry in all {
                let selected = entry == current;
                let data = match entry {
                    BuffState::Some(id) => defs.definition(*id).and_then(|def| def.data()),
                    _ => None,
                };

                // apply color to selectable
                let style = data
                    .and_then(|data| data.rarity.color())
                    .map(|color| ui.push_style_color(StyleColor::Text, color));
                if Selectable::new(item_label(entry))
                    .selected(selected)
                    .build(ui)
                {
                    result = Some(entry);
                }
                drop(style);

                // handle focus
                if selected {
                    ui.set_item_default_focus();
                }

                // tooltip
                if ui.is_item_hovered() {
                    if let Some(buff) = data {
                        render_buff_tooltip(ui, buff);
                    }
                }
            }
        }
        result
    }
}

impl Component<Props<'_>> for Demo {
    fn render(&mut self, ui: &Ui, (defs, custom): Props) {
        // initialize data
        if self.all_foods.is_empty() {
            self.all_foods = SPECIAL_BUFFS
                .iter()
                .copied()
                .chain(defs.all_food().map(|food| BuffState::Some(food.id)))
                .collect();
        }
        if self.all_utils.is_empty() {
            self.all_utils = SPECIAL_BUFFS
                .iter()
                .copied()
                .chain(defs.all_util().map(|util| BuffState::Some(util.id)))
                .collect();
        }

        // main window
        {
            let _style = render::small_padding(ui);

            // reminder buttons
            ui.align_text_to_frame_padding();
            ui.text("Reminders:");

            ui.same_line_with_spacing(0.0, 10.0);
            if ui.button("Food") {
                self.reminder.trigger_food();
            }

            ui.same_line_with_spacing(0.0, 5.0);
            if ui.button("Util") {
                self.reminder.trigger_util();
            }

            for remind in custom {
                ui.same_line_with_spacing(0.0, 5.0);
                if ui.button(remind.display_name()) {
                    self.reminder.trigger_custom(remind.id);
                }
            }

            ui.spacing();
            ui.separator();
            ui.spacing();

            ui.checkbox("Demo Tracker", self.tracker.visible_mut());

            // player editor
            if let Some(_table) = ui.begin_table_header(
                "##table",
                [
                    TableColumnSetup::new("Sub"),
                    TableColumnSetup::new("Character"),
                    TableColumnSetup::new("Account"),
                    TableColumnSetup::new("Class"),
                    TableColumnSetup::new("Food"),
                    TableColumnSetup::new("Util"),
                    TableColumnSetup::new("Custom"),
                ],
            ) {
                const INPUT_SIZE: f32 = 100.0;

                // entries
                for id in 0..self.tracker.players.len() {
                    ui.table_next_row();
                    let Entry { player, data } = self.tracker.players.player_mut(id).unwrap();

                    // subgroup
                    let mut sub = player.subgroup as i32;
                    ui.table_next_column();
                    ui.set_next_item_width(render::ch_width(ui, 3));
                    if ui
                        .input_int(format!("##sub-{id}"), &mut sub)
                        .step(0)
                        .build()
                    {
                        player.subgroup = match sub {
                            1..=15 => sub as usize,
                            _ => 0,
                        };
                    }

                    // character name
                    ui.table_next_column();
                    ui.set_next_item_width(INPUT_SIZE);
                    ui.input_text(format!("##char-{id}"), &mut player.character)
                        .build();

                    // account name
                    ui.table_next_column();
                    ui.set_next_item_width(INPUT_SIZE);
                    ui.input_text(format!("##acc-{id}"), &mut player.account)
                        .build();

                    // profession select
                    ui.table_next_column();
                    ui.set_next_item_width(INPUT_SIZE);
                    render_prof_select(ui, format!("##prof-{id}"), &mut player.profession);

                    // food select
                    ui.table_next_column();
                    ui.set_next_item_width(INPUT_SIZE);
                    if let Some(new) = Self::render_combo(
                        ui,
                        defs,
                        format!("##food-{id}"),
                        &self.all_foods,
                        &mut data.food.state,
                        |buff| Self::food_name(defs, *buff),
                    ) {
                        data.food.state = *new;
                    }

                    // utility select
                    ui.table_next_column();
                    ui.set_next_item_width(INPUT_SIZE);
                    if let Some(new) = Self::render_combo(
                        ui,
                        defs,
                        format!("##util-{id}"),
                        &self.all_utils,
                        &mut data.util.state,
                        |buff| Self::util_name(defs, *buff),
                    ) {
                        data.util.state = *new;
                    }

                    // custom
                    ui.table_next_column();
                    for custom in self.reminder.all_custom() {
                        let mut applied = data
                            .custom
                            .get(&custom.id)
                            .map(|entry| matches!(entry.state, BuffState::Some(_)))
                            .unwrap_or(false);
                        ui.same_line();
                        if ui.checkbox(format!("##custom-{}", custom.id), &mut applied) {
                            data.custom.insert(
                                custom.id,
                                TrackedBuff::new(if applied {
                                    BuffState::None
                                } else {
                                    BuffState::Some(())
                                }),
                            );
                        }
                        if ui.is_item_hovered() {
                            ui.tooltip_text(&custom.name);
                        }
                    }
                }
            }

            // add button
            if ui.button("Add") {
                let next_id = self.tracker.players.len();
                self.tracker.add_player(Player::new(
                    next_id,
                    0,
                    "char",
                    "acc",
                    false,
                    Profession::Unknown,
                    Specialization::Unknown,
                    1,
                ));
            }
            let [button_width, _] = ui.item_rect_size();

            // remove button
            ui.same_line_with_pos(button_width + 10.0);
            if ui.button("Remove") {
                let last_id = self.tracker.players.len() - 1;
                self.tracker.remove_player(last_id);
            }
        }

        // render children
        self.reminder.render(ui, ());
        self.tracker.render(ui, (defs, custom));
    }
}

impl Windowable<Props<'_>> for Demo {
    const CONTEXT_MENU: bool = true;
}
