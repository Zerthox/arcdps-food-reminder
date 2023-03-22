use super::Demo;
use crate::{
    data::PROFESSIONS,
    tracking::{buff::BuffState, ui::Props as TrackerProps},
};
use arc_util::{
    tracking::{Entry, Player},
    ui::{render, Component, Hideable, Windowable},
};
use arcdps::{
    imgui::{TableColumnSetup, Ui},
    Profession, Specialization,
};

pub type Props<'p> = TrackerProps<'p>;

const SPECIAL_BUFFS: [BuffState<u32>; 3] =
    [BuffState::Unknown, BuffState::None, BuffState::Some(0)];

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
                    let mut index = PROFESSIONS
                        .iter()
                        .position(|prof| *prof == player.profession)
                        .unwrap();
                    ui.set_next_item_width(INPUT_SIZE);
                    if ui.combo(format!("##prof-{id}"), &mut index, PROFESSIONS, |prof| {
                        <&str>::from(prof).into()
                    }) {
                        player.profession = PROFESSIONS[index];
                    }

                    // food select
                    ui.table_next_column();
                    let mut food_id = self
                        .all_foods
                        .iter()
                        .position(|buff| *buff == data.food.state)
                        .unwrap();
                    ui.set_next_item_width(INPUT_SIZE);
                    if ui.combo(
                        format!("##food-{id}"),
                        &mut food_id,
                        &self.all_foods,
                        |buff| Self::food_name(defs, *buff),
                    ) {
                        data.food.state = self.all_foods[food_id];
                    }

                    // utility
                    ui.table_next_column();
                    let mut util_id = self
                        .all_utils
                        .iter()
                        .position(|buff| *buff == data.util.state)
                        .unwrap();
                    ui.set_next_item_width(INPUT_SIZE);
                    if ui.combo(
                        format!("##util-{id}"),
                        &mut util_id,
                        &self.all_utils,
                        |buff| Self::util_name(defs, *buff),
                    ) {
                        data.util.state = self.all_utils[util_id];
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
