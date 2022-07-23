use super::Demo;
use crate::{
    data::{Definitions, PROFESSIONS},
    tracking::{
        buff::BuffState,
        entry::{Profession, Specialization},
    },
};
use arc_util::{
    player::Player,
    ui::{render, Component, Hideable, Windowable},
};
use arcdps::imgui::{TableColumnSetup, Ui};

impl Component<'_> for Demo {
    type Props = Definitions;

    fn render(&mut self, ui: &Ui, defs: &Self::Props) {
        // initialize data
        if self.all_foods.is_empty() {
            self.all_foods = [BuffState::Unknown, BuffState::None, BuffState::Some(0)]
                .iter()
                .copied()
                .chain(defs.all_food().map(|food| BuffState::Some(food.id)))
                .collect();
        }
        if self.all_utils.is_empty() {
            self.all_utils = [BuffState::Unknown, BuffState::None, BuffState::Some(0)]
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
            if ui.button("Trigger Food") {
                self.reminder.trigger_food();
            }

            ui.same_line_with_spacing(0.0, 5.0);
            if ui.button("Trigger Util") {
                self.reminder.trigger_util();
            }

            ui.same_line_with_spacing(0.0, 5.0);
            if ui.button("Trigger Reinf") {
                self.reminder.trigger_reinforced();
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
                ],
            ) {
                const INPUT_SIZE: f32 = 100.0;

                // entries
                for id in 0..self.tracker.len() {
                    ui.table_next_row();
                    let entry = self.tracker.player_mut(id).unwrap();

                    // subgroup
                    let mut sub = entry.player.subgroup as i32;
                    ui.table_next_column();
                    ui.set_next_item_width(render::ch_width(ui, 3));
                    if ui
                        .input_int(format!("##sub-{}", id), &mut sub)
                        .step(0)
                        .build()
                    {
                        entry.player.subgroup = match sub {
                            1..=15 => sub as usize,
                            _ => 0,
                        };
                    }

                    // character name
                    ui.table_next_column();
                    ui.set_next_item_width(INPUT_SIZE);
                    ui.input_text(format!("##char-{}", id), &mut entry.player.character)
                        .build();

                    // account name
                    ui.table_next_column();
                    ui.set_next_item_width(INPUT_SIZE);
                    ui.input_text(format!("##acc-{}", id), &mut entry.player.account)
                        .build();

                    // profession select
                    ui.table_next_column();
                    let mut index = PROFESSIONS
                        .iter()
                        .position(|prof| *prof == entry.player.profession)
                        .unwrap();
                    ui.set_next_item_width(INPUT_SIZE);
                    if ui.combo(format!("##prof-{}", id), &mut index, PROFESSIONS, |prof| {
                        <&str>::from(prof).into()
                    }) {
                        entry.player.profession = PROFESSIONS[index];
                    }

                    // food select
                    ui.table_next_column();
                    let mut food_id = self
                        .all_foods
                        .iter()
                        .position(|buff| *buff == entry.food.state)
                        .unwrap();
                    ui.set_next_item_width(INPUT_SIZE);
                    if ui.combo(
                        format!("##food-{}", id),
                        &mut food_id,
                        &self.all_foods,
                        |buff| Self::food_name(defs, *buff),
                    ) {
                        entry.food.state = self.all_foods[food_id];
                    }

                    // utility
                    ui.table_next_column();
                    let mut util_id = self
                        .all_utils
                        .iter()
                        .position(|buff| *buff == entry.util.state)
                        .unwrap();
                    ui.set_next_item_width(INPUT_SIZE);
                    if ui.combo(
                        format!("##util-{}", id),
                        &mut util_id,
                        &self.all_utils,
                        |buff| Self::util_name(defs, *buff),
                    ) {
                        entry.util.state = self.all_utils[util_id];
                    }
                }
            }

            // add button
            if ui.button("Add") {
                let next_id = self.tracker.len();
                self.tracker.add_player(Player::new(
                    next_id,
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
                let last_id = self.tracker.len() - 1;
                self.tracker.remove_player(last_id);
            }
        }

        // render children
        self.reminder.render(ui, &());
        self.tracker.render(ui, defs);
    }
}

impl Windowable<'_> for Demo {
    const CONTEXT_MENU: bool = true;
}
