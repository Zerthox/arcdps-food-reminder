//! Dummy windows for demo.

use crate::{
    reminder::Reminder,
    tracking::{
        buff::{Buff, Food, Utility},
        player::{Player, Profession, Specialization},
        Tracker,
    },
    ui::{
        window::{Window, WindowProps, Windowed},
        Component, Hideable,
    },
};
use arcdps::imgui::{im_str, ComboBox, ImStr, ImString, Ui};
use std::borrow::Cow;
use strum::IntoEnumIterator;

/// Features demo.
#[derive(Debug)]
pub struct Demo {
    reminder: Reminder,
    next_id: usize,
    all_foods: Vec<Buff<Food>>,
    all_utils: Vec<Buff<Utility>>,
    tracker: Window<Tracker>,
}

impl Demo {
    /// Creates a new demo.
    pub fn new() -> Self {
        Self {
            reminder: Reminder::new(),
            next_id: 0,
            all_foods: [Buff::Unset, Buff::None, Buff::Unknown]
                .iter()
                .copied()
                .chain(Food::iter().map(Buff::Known))
                .collect(),
            all_utils: [Buff::Unset, Buff::None, Buff::Unknown]
                .iter()
                .copied()
                .chain(Utility::iter().map(Buff::Known))
                .collect(),
            tracker: Tracker::create(),
        }
    }

    /// Returns the display name for a given food buff.
    fn food_name(buff: Buff<Food>) -> Cow<'static, ImStr> {
        match buff {
            Buff::Unset => im_str!("Unset").into(),
            Buff::None => im_str!("None").into(),
            Buff::Unknown => im_str!("Unknown").into(),
            Buff::Known(food) => im_str!("{}", food.name()).into(),
        }
    }

    /// Returns the display name for a given utility buff.
    fn util_name(buff: Buff<Utility>) -> Cow<'static, ImStr> {
        match buff {
            Buff::Unset => im_str!("Unset").into(),
            Buff::None => im_str!("None").into(),
            Buff::Unknown => im_str!("Unknown").into(),
            Buff::Known(util) => im_str!("{}", util.name()).into(),
        }
    }
}

impl Component for Demo {
    fn render(&mut self, ui: &Ui) {
        // main window
        // TODO: left align helper

        // reminder buttons
        ui.align_text_to_frame_padding();
        ui.text(im_str!("Reminders:"));
        let [text_width, _] = ui.item_rect_size();

        ui.same_line(text_width + 10.0);
        if ui.button(im_str!("Trigger Food"), [0.0, 0.0]) {
            self.reminder.trigger_food();
        }
        let [button_width, _] = ui.item_rect_size();

        ui.same_line(text_width + button_width + 20.0);
        if ui.button(im_str!("Trigger Util"), [0.0, 0.0]) {
            self.reminder.trigger_util();
        }

        ui.spacing();
        ui.separator();
        ui.spacing();

        ui.checkbox(im_str!("Tracker"), self.tracker.visibility());

        // player editor
        if ui.begin_table(im_str!("##food-reminder-demo-table"), 6) {
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
            for id in 0..self.next_id {
                ui.table_next_row();
                let player = self.tracker.get_player_mut(id).unwrap();

                // sub
                ui.table_next_column();
                let mut sub = ImString::with_capacity(2);
                sub.push_str(&format!("{}", player.subgroup));
                ui.input_text(&im_str!("##food-reminder-demo-sub-{}", id), &mut sub)
                    .chars_decimal(true)
                    .build();
                player.subgroup = match AsRef::<str>::as_ref(&sub).parse() {
                    Ok(num) if num > 15 => 15,
                    Ok(0) | Err(_) => 1,
                    Ok(num) => num,
                };

                // character name
                ui.table_next_column();
                let mut char_name = ImString::with_capacity(19);
                char_name.push_str(&player.character);
                ui.push_item_width(INPUT_SIZE);
                ui.input_text(&im_str!("##food-reminder-demo-char-{}", id), &mut char_name)
                    .build();
                player.character = AsRef::<str>::as_ref(&char_name).into();

                // account name
                ui.table_next_column();
                let mut acc_name = ImString::with_capacity(19);
                acc_name.push_str(&player.account);
                ui.push_item_width(INPUT_SIZE);
                ui.input_text(&im_str!("##food-reminder-demo-acc-{}", id), &mut acc_name)
                    .build();
                player.account = AsRef::<str>::as_ref(&acc_name).into();

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
                let mut prof = player.profession as usize;
                ui.push_item_width(INPUT_SIZE);
                ComboBox::new(&im_str!("##food-reminder-demo-class-{}", id)).build_simple(
                    ui,
                    &mut prof,
                    &PROF_NAMES,
                    &|prof| (*prof).into(),
                );
                player.profession = (prof as u32).into();

                // food
                ui.table_next_column();
                let mut food_id = self
                    .all_foods
                    .iter()
                    .position(|buff| *buff == player.food)
                    .unwrap();
                ui.push_item_width(INPUT_SIZE);
                ComboBox::new(&im_str!("##food-reminder-demo-food-{}", id)).build_simple(
                    ui,
                    &mut food_id,
                    &self.all_foods,
                    &|buff| Self::food_name(*buff),
                );
                player.food = self.all_foods[food_id];

                // utility
                ui.table_next_column();
                let mut util_id = self
                    .all_utils
                    .iter()
                    .position(|buff| *buff == player.util)
                    .unwrap();
                ui.push_item_width(INPUT_SIZE);
                ComboBox::new(&im_str!("##food-reminder-demo-util-{}", id)).build_simple(
                    ui,
                    &mut util_id,
                    &self.all_utils,
                    &|buff| Self::util_name(*buff),
                );
                player.util = self.all_utils[util_id];
            }

            ui.end_table();
        }

        // add button
        if ui.button(im_str!("Add"), [0.0, 0.0]) {
            self.tracker.add_player(Player::new(
                self.next_id,
                "char",
                "acc",
                false,
                Profession::Unknown,
                Specialization::Unknown,
                1,
            ));
            self.next_id += 1;
        }
        let [button_width, _] = ui.item_rect_size();

        // remove button
        ui.same_line(button_width + 10.0);
        if ui.button(im_str!("Remove"), [0.0, 0.0]) {
            self.next_id -= 1;
            self.tracker.remove_player(self.next_id);
        }

        // render children
        self.reminder.render(ui);
        self.tracker.render(ui);
    }
}

impl Windowed for Demo {
    fn props() -> WindowProps {
        WindowProps::new("Food Demo")
            .visible(true)
            .auto_resize(true)
    }
}

impl Default for Demo {
    fn default() -> Self {
        Self::new()
    }
}
