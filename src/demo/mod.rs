//! Dummy windows for demo.

pub mod settings;

use crate::{
    defs::{BuffDef, Definitions},
    reminder::Reminder,
    tracking::{
        buff::BuffState,
        entry::{Profession, Specialization},
        Tracker,
    },
};
use arc_util::{
    game::Player,
    ui::{align::LeftAlign, Component, Hideable, Window, WindowOptions, Windowable},
};
use arcdps::imgui::{TableColumnSetup, Ui};
use std::borrow::Cow;

/// Features demo.
#[derive(Debug)]
pub struct Demo {
    reminder: Reminder,
    all_foods: Vec<BuffState>,
    all_utils: Vec<BuffState>,
    tracker: Window<Tracker>,
}

impl Demo {
    /// Creates a new demo.
    pub fn new() -> Self {
        Self {
            reminder: Reminder::new(),
            all_foods: Vec::new(),
            all_utils: Vec::new(),
            tracker: Window::new(
                WindowOptions {
                    auto_resize: true,
                    ..WindowOptions::new("Demo Food Tracker")
                },
                Tracker::new(),
            ),
        }
    }

    /// Returns the display name for a given food buff.
    fn food_name(defs: &Definitions, buff: BuffState) -> Cow<'static, str> {
        match buff {
            BuffState::Unknown => "Unset".into(),
            BuffState::None => "None".into(),
            BuffState::Some(buff) => {
                if let Some(BuffDef::Food(food)) = defs.get_buff(buff) {
                    food.name.clone().into()
                } else {
                    "Unknown".into()
                }
            }
        }
    }

    /// Returns the display name for a given utility buff.
    fn util_name(defs: &Definitions, buff: BuffState) -> Cow<'static, str> {
        match buff {
            BuffState::Unknown => "Unset".into(),
            BuffState::None => "None".into(),
            BuffState::Some(buff) => {
                if let Some(BuffDef::Util(util)) = defs.get_buff(buff) {
                    util.name.clone().into()
                } else {
                    "Unknown".into()
                }
            }
        }
    }
}

impl Component for Demo {
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

        // reminder buttons
        let mut align = LeftAlign::build();
        align.item(ui, || {
            ui.align_text_to_frame_padding();
            ui.text("Reminders:");
        });
        align.item_with_spacing(ui, 10.0, || {
            if ui.button("Trigger Food") {
                self.reminder.trigger_food();
            }
        });
        align.item(ui, || {
            if ui.button("Trigger Util") {
                self.reminder.trigger_util();
            }
        });

        ui.spacing();
        ui.separator();
        ui.spacing();

        ui.checkbox("Tracker", self.tracker.visible_mut());

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

                // sub
                ui.table_next_column();
                let mut sub = String::with_capacity(2);
                sub.push_str(&entry.player.subgroup.to_string());
                if ui
                    .input_text(format!("##sub-{}", id), &mut sub)
                    .chars_decimal(true)
                    .build()
                {
                    entry.player.subgroup = match sub.parse() {
                        Ok(num) if num > 15 => 15,
                        Ok(0) | Err(_) => 1,
                        Ok(num) => num,
                    };
                }

                // character name
                ui.table_next_column();
                let mut char_name = String::with_capacity(19);
                char_name.push_str(&entry.player.character);
                ui.set_next_item_width(INPUT_SIZE);
                if ui
                    .input_text(format!("##char-{}", id), &mut char_name)
                    .build()
                {
                    entry.player.character = char_name;
                }

                // account name
                ui.table_next_column();
                let mut acc_name = String::with_capacity(19);
                acc_name.push_str(&entry.player.account);
                ui.set_next_item_width(INPUT_SIZE);
                if ui
                    .input_text(format!("##acc-{}", id), &mut acc_name)
                    .build()
                {
                    entry.player.account = acc_name;
                }

                // class
                ui.table_next_column();
                const PROF_NAMES: [&str; 10] = [
                    "Unknown",
                    "Guardian",
                    "Warrior",
                    "Engineer",
                    "Ranger",
                    "Thief",
                    "Elementalist",
                    "Mesmer",
                    "Necromancer",
                    "Revenant",
                ];
                let mut prof = entry.player.profession as usize;
                ui.set_next_item_width(INPUT_SIZE);
                if ui.combo(format!("##class-{}", id), &mut prof, &PROF_NAMES, |prof| {
                    (*prof).into()
                }) {
                    entry.player.profession = (prof as u32).into();
                }

                // food
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

        // render children
        self.reminder.render(ui, &());
        self.tracker.render(ui, defs);
    }
}

impl Windowable for Demo {
    const CONTEXT_MENU: bool = true;
}
