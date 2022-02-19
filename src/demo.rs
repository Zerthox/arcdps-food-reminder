//! Dummy windows for demo.

use crate::{
    defs::{BuffDef, Definitions},
    reminder::Reminder,
    tracking::{
        buff::BuffState,
        entry::{Entry, Profession, Specialization},
        Tracker,
    },
};
use arc_util::{
    game::Player,
    settings::HasSettings,
    ui::{align::LeftAlign, Component, Hideable, Window, WindowProps, Windowed},
};
use arcdps::imgui::{im_str, ComboBox, ImStr, ImString, Ui};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Features demo.
#[derive(Debug)]
pub struct Demo {
    defs: Definitions,
    reminder: Reminder,
    all_foods: Vec<BuffState>,
    all_utils: Vec<BuffState>,
    tracker: Window<Tracker>,
}

impl Demo {
    /// Creates a new demo.
    pub fn new(defs: Definitions) -> Self {
        Self {
            defs: defs.clone(),
            reminder: Reminder::new(),
            all_foods: [BuffState::Unknown, BuffState::None, BuffState::Some(0)]
                .iter()
                .copied()
                .chain(defs.all_food().map(|food| BuffState::Some(food.id)))
                .collect(),
            all_utils: [BuffState::Unknown, BuffState::None, BuffState::Some(0)]
                .iter()
                .copied()
                .chain(defs.all_util().map(|util| BuffState::Some(util.id)))
                .collect(),
            tracker: Tracker::new(defs).windowed(),
        }
    }

    /// Returns the display name for a given food buff.
    fn food_name(defs: &Definitions, buff: BuffState) -> Cow<'static, ImStr> {
        match buff {
            BuffState::Unknown => im_str!("Unset").into(),
            BuffState::None => im_str!("None").into(),
            BuffState::Some(buff) => {
                if let Some(BuffDef::Food(food)) = defs.get(buff) {
                    im_str!("{}", food.name).into()
                } else {
                    im_str!("Unknown").into()
                }
            }
        }
    }

    /// Returns the display name for a given utility buff.
    fn util_name(defs: &Definitions, buff: BuffState) -> Cow<'static, ImStr> {
        match buff {
            BuffState::Unknown => im_str!("Unset").into(),
            BuffState::None => im_str!("None").into(),
            BuffState::Some(buff) => {
                if let Some(BuffDef::Util(util)) = defs.get(buff) {
                    im_str!("{}", util.name).into()
                } else {
                    im_str!("Unknown").into()
                }
            }
        }
    }
}

impl Component for Demo {
    fn render(&mut self, ui: &Ui) {
        // main window

        // reminder buttons
        let mut align = LeftAlign::build();
        ui.align_text_to_frame_padding();
        align.item_with_margin(ui, 10.0, || ui.text(im_str!("Reminders:")));
        align.item(ui, || {
            if ui.button(im_str!("Trigger Food"), [0.0, 0.0]) {
                self.reminder.trigger_food();
            }
        });
        align.item(ui, || {
            if ui.button(im_str!("Trigger Util"), [0.0, 0.0]) {
                self.reminder.trigger_util();
            }
        });

        ui.spacing();
        ui.separator();
        ui.spacing();

        ui.checkbox(im_str!("Tracker"), self.tracker.is_visible_mut());

        // player editor
        if ui.begin_table(im_str!("##table"), 6) {
            const INPUT_SIZE: f32 = 100.0;

            // declare columns
            ui.table_setup_column(im_str!("Sub"));
            ui.table_setup_column(im_str!("Character"));
            ui.table_setup_column(im_str!("Account"));
            ui.table_setup_column(im_str!("Class"));
            ui.table_setup_column(im_str!("Food"));
            ui.table_setup_column(im_str!("Util"));

            // render header
            ui.table_headers_row();

            // entries
            for id in 0..self.tracker.len() {
                ui.table_next_row();
                let entry = self.tracker.player_mut(id).unwrap();

                // sub
                ui.table_next_column();
                let mut sub = ImString::with_capacity(2);
                sub.push_str(&format!("{}", entry.player.subgroup));
                if ui
                    .input_text(&im_str!("##sub-{}", id), &mut sub)
                    .chars_decimal(true)
                    .build()
                {
                    entry.player.subgroup = match sub.to_str().parse() {
                        Ok(num) if num > 15 => 15,
                        Ok(0) | Err(_) => 1,
                        Ok(num) => num,
                    };
                }

                // character name
                ui.table_next_column();
                let mut char_name = ImString::with_capacity(19);
                char_name.push_str(&entry.player.character);
                ui.push_item_width(INPUT_SIZE);
                if ui
                    .input_text(&im_str!("##char-{}", id), &mut char_name)
                    .build()
                {
                    entry.player.character = char_name.to_str().into();
                }

                // account name
                ui.table_next_column();
                let mut acc_name = ImString::with_capacity(19);
                acc_name.push_str(&entry.player.account);
                ui.push_item_width(INPUT_SIZE);
                if ui
                    .input_text(&im_str!("##acc-{}", id), &mut acc_name)
                    .build()
                {
                    entry.player.account = acc_name.to_string();
                }

                // class
                ui.table_next_column();
                const PROF_NAMES: [&ImStr; 10] = [
                    im_str!("Unknown"),
                    im_str!("Guardian"),
                    im_str!("Warrior"),
                    im_str!("Engineer"),
                    im_str!("Ranger"),
                    im_str!("Thief"),
                    im_str!("Elementalist"),
                    im_str!("Mesmer"),
                    im_str!("Necromancer"),
                    im_str!("Revenant"),
                ];
                let mut prof = entry.player.profession as usize;
                ui.push_item_width(INPUT_SIZE);
                if ComboBox::new(&im_str!("##class-{}", id)).build_simple(
                    ui,
                    &mut prof,
                    &PROF_NAMES,
                    &|prof| (*prof).into(),
                ) {
                    entry.player.profession = (prof as u32).into();
                }

                // food
                ui.table_next_column();
                let mut food_id = self
                    .all_foods
                    .iter()
                    .position(|buff| *buff == entry.food.state)
                    .unwrap();
                ui.push_item_width(INPUT_SIZE);
                if ComboBox::new(&im_str!("##food-{}", id)).build_simple(
                    ui,
                    &mut food_id,
                    &self.all_foods,
                    &|buff| Self::food_name(&self.defs, *buff),
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
                ui.push_item_width(INPUT_SIZE);
                if ComboBox::new(&im_str!("##util-{}", id)).build_simple(
                    ui,
                    &mut util_id,
                    &self.all_utils,
                    &|buff| Self::util_name(&self.defs, *buff),
                ) {
                    entry.util.state = self.all_utils[util_id];
                }
            }

            ui.end_table();
        }

        // add button
        if ui.button(im_str!("Add"), [0.0, 0.0]) {
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
        ui.same_line(button_width + 10.0);
        if ui.button(im_str!("Remove"), [0.0, 0.0]) {
            let last_id = self.tracker.len() - 1;
            self.tracker.remove_player(last_id);
        }

        // render children
        self.reminder.render(ui);
        self.tracker.render(ui);
    }
}

impl Windowed for Demo {
    fn window_props() -> WindowProps {
        WindowProps::new("Food Demo")
            .visible(true)
            .auto_resize(true)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DemoSettings {
    players: Vec<Entry>,
    tracker: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for DemoSettings {
    fn default() -> Self {
        Self {
            players: Vec::new(),
            tracker: false,
        }
    }
}

impl HasSettings for Demo {
    type Settings = DemoSettings;

    const SETTINGS_ID: &'static str = "demo";

    fn current_settings(&self) -> Self::Settings {
        Self::Settings {
            players: self.tracker.all_players().cloned().collect(),
            tracker: self.tracker.is_visible(),
        }
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        for loaded in loaded.players {
            let id = loaded.player.id;
            self.tracker.add_player(loaded.player);
            let entry = self.tracker.player_mut(id).unwrap();
            entry.food = loaded.food;
            entry.util = loaded.util;
        }
        self.tracker.set_visibility(loaded.tracker);
    }
}
