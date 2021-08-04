pub mod buff;
pub mod player;

use crate::{
    arc_util::{api::CoreColor, exports},
    settings::HasSettings,
    ui::{
        render,
        window::{WindowProps, Windowed},
        Component,
    },
};
use arcdps::imgui::{im_str, TableColumnFlags, TableFlags, Ui};
use buff::{Buff, BuffState, Categorize, Food, Utility};
use player::Player;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Player tracker.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Tracker {
    /// Currently tracked players mapped by their id.
    #[serde(skip)]
    players: HashMap<usize, Player>,

    /// Current local player (self) id.
    #[serde(skip)]
    self_id: usize,

    /// Cache for temporarily saved buffs on last character of local player (self).
    #[serde(skip)]
    cache: Option<(String, Buff<Food>, Buff<Utility>)>,
}

impl Tracker {
    /// Creates a new tracker.
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            self_id: 0,
            cache: None,
        }
    }

    /// Adds a new tracked player.
    pub fn add_player(&mut self, mut player: Player) {
        // check for self
        if player.is_self {
            // update self id
            self.self_id = player.id;

            // check & reset cache
            if let Some((character, food, util)) = self.cache.take() {
                // check for same character
                if character == player.character {
                    // use cached food
                    player.food = food;
                    player.util = util;
                }
            }
        }

        // insert player
        self.players.insert(player.id, player);
    }

    /// Removes a tracked player, returning the removed player if they were tracked.
    pub fn remove_player(&mut self, id: usize) -> Option<Player> {
        // remove player
        let removed = self.players.remove(&id);

        // check for self
        if id == self.self_id {
            if let Some(player) = &removed {
                // TODO: set cache timeout
                // cache character buffs
                self.cache = Some((player.character.clone(), player.food, player.util));
            }
        }

        // return removed player
        removed
    }

    /// Checks whether the given id is the local player (self).
    pub fn is_self(&self, id: usize) -> bool {
        self.self_id == id
    }

    /// Returns a reference to the local player (self).
    pub fn get_self(&self) -> Option<&Player> {
        self.players.get(&self.self_id)
    }

    /// Returns a mutable reference to the local player (self).
    pub fn get_self_mut(&mut self) -> Option<&mut Player> {
        self.players.get_mut(&self.self_id)
    }

    /// Returns a reference to a tracked player.
    pub fn get_player(&self, id: usize) -> Option<&Player> {
        self.players.get(&id)
    }

    /// Returns a mutable reference to a tracked player.
    pub fn get_player_mut(&mut self, id: usize) -> Option<&mut Player> {
        self.players.get_mut(&id)
    }

    /// Returns an unsorted iterator over all tracked players.
    pub fn get_players(&self) -> impl Iterator<Item = &Player> {
        self.players.values()
    }

    /// Returns an unsorted mutable iterator over all tracked players.
    pub fn get_players_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.players.values_mut()
    }

    /// Returns all tracked players sorted by subgroup.
    pub fn get_players_by_sub(&self) -> Vec<&Player> {
        let mut players = self.get_players().collect::<Vec<_>>();
        players.sort_by_key(|player| player.subgroup);
        players
    }

    /// Returns the number of tracked players.
    pub fn len(&self) -> usize {
        self.players.len()
    }

    /// Returns true if there is no tracked players.
    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    /// Renders a context menu for a food item.
    fn render_food_context_menu(ui: &Ui, menu_id: usize, buff_id: u32, name: Option<&str>) {
        render::item_context_menu(
            &im_str!("##food-reminder-tracker-context-food-{}", menu_id),
            || {
                ui.text(im_str!("Food Options"));
                if let Some(name) = name {
                    if ui.small_button(im_str!("Copy Name")) {
                        ui.set_clipboard_text(&im_str!("{}", name));
                    }
                }
                if ui.small_button(im_str!("Copy ID")) {
                    ui.set_clipboard_text(&im_str!("{}", buff_id));
                }
            },
        );
    }

    /// Renders a context menu for a utility item.
    fn render_util_context_menu(ui: &Ui, menu_id: usize, buff_id: u32, name: Option<&str>) {
        render::item_context_menu(
            &im_str!("##food-reminder-tracker-context-util-{}", menu_id),
            || {
                ui.text(im_str!("Utility Options"));
                if let Some(name) = name {
                    if ui.small_button(im_str!("Copy Name")) {
                        ui.set_clipboard_text(&im_str!("{}", name));
                    }
                }
                if ui.small_button(im_str!("Copy ID")) {
                    ui.set_clipboard_text(&im_str!("{}", buff_id));
                }
            },
        );
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Tracker {
    fn render(&mut self, ui: &Ui) {
        if self.players.is_empty() {
            ui.text("No players in range");
        } else {
            // create table
            if ui.begin_table_with_flags(
                im_str!("##food-reminder-tracker-table"),
                4,
                TableFlags::NONE,
            ) {
                // declare columns
                ui.table_setup_column_with_flags(im_str!("Sub"), TableColumnFlags::DEFAULT_SORT);
                ui.table_setup_column_with_flags(im_str!("Player"), TableColumnFlags::NO_SORT);
                ui.table_setup_column_with_flags(im_str!("Food"), TableColumnFlags::NO_SORT);
                ui.table_setup_column_with_flags(im_str!("Util"), TableColumnFlags::NO_SORT);

                // render header
                ui.table_headers_row();

                // TODO: sorting, imgui-rs currently has no wrapping, need to use imgui::sys directly

                // grab arc colors
                let colors = exports::get_colors();
                let red = colors
                    .get_core(CoreColor::LightRed)
                    .map(|vec| vec.into())
                    .unwrap_or([1.0, 0.0, 0.0, 1.0]);
                let green = colors
                    .get_core(CoreColor::LightGreen)
                    .map(|vec| vec.into())
                    .unwrap_or([0.0, 1.0, 0.0, 1.0]);
                let yellow = colors
                    .get_core(CoreColor::LightYellow)
                    .map(|vec| vec.into())
                    .unwrap_or([1.0, 1.0, 0.0, 1.0]);

                // iterate over tracked players
                for player in self.get_players_by_sub() {
                    // new row for each player
                    ui.table_next_row();

                    // render subgroup cell
                    ui.table_next_column();
                    let sub = format!("{:>2}", player.subgroup);
                    match colors.get_sub_base(player.subgroup) {
                        Some(color) => ui.text_colored(color.into(), sub),
                        None => ui.text(sub),
                    }

                    // render name cell
                    ui.table_next_column();
                    match colors.get_prof_base(player.profession) {
                        Some(color) => ui.text_colored(color.into(), &player.character),
                        None => ui.text(&player.character),
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip_text(&player.account);
                    }

                    // render food cell
                    ui.table_next_column();
                    match player.food.state {
                        BuffState::Unset => {
                            ui.text("???");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("Uncertain");
                            }
                        }
                        BuffState::None => {
                            ui.text_colored(red, "NONE");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("No Food");
                            }
                        }
                        BuffState::Unknown(id) => {
                            ui.text_colored(yellow, "SOME");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("Unknown Food");
                            }
                            Self::render_food_context_menu(ui, player.id, id, None);
                        }
                        BuffState::Known(food @ Food::Malnourished) => {
                            ui.text_colored(red, "MAL");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("Malnourished");
                            }
                            Self::render_food_context_menu(
                                ui,
                                player.id,
                                food.into(),
                                Some(food.name()),
                            );
                        }
                        BuffState::Known(food) => {
                            ui.text_colored(green, food.categorize());
                            if ui.is_item_hovered() {
                                ui.tooltip_text(format!(
                                    "{}\n{}",
                                    food.name(),
                                    food.stats().join("\n")
                                ));
                            }
                            Self::render_food_context_menu(
                                ui,
                                player.id,
                                food.into(),
                                Some(food.name()),
                            );
                        }
                    }

                    // render util cell
                    ui.table_next_column();
                    match player.util.state {
                        BuffState::Unset => {
                            ui.text("???");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("Uncertain");
                            }
                        }
                        BuffState::None => {
                            ui.text_colored(red, "NONE");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("No Utility");
                            }
                        }
                        BuffState::Unknown(id) => {
                            ui.text_colored(yellow, "SOME");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("Unknown Utility");
                            }
                            Self::render_util_context_menu(ui, player.id, id, None);
                        }
                        BuffState::Known(util @ Utility::Diminished) => {
                            ui.text_colored(red, "DIM");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("Diminished");
                            }
                            Self::render_util_context_menu(
                                ui,
                                player.id,
                                util.into(),
                                Some(util.name()),
                            );
                        }
                        BuffState::Known(util) => {
                            ui.text_colored(green, util.categorize());
                            if ui.is_item_hovered() {
                                ui.tooltip_text(format!(
                                    "{}\n{}",
                                    util.name(),
                                    util.stats().join("\n")
                                ));
                            }
                            Self::render_util_context_menu(
                                ui,
                                player.id,
                                util.into(),
                                Some(util.name()),
                            );
                        }
                    }
                }

                ui.end_table();
            }
        }
    }
}

impl Windowed for Tracker {
    fn window_props() -> WindowProps {
        WindowProps::new("Food Tracker")
            .visible(false)
            .auto_resize(true)
    }
}

impl HasSettings for Tracker {
    type Settings = ();
    fn settings_name() -> &'static str {
        "tracker"
    }
    fn get_settings(&self) -> Self::Settings {}
    fn load_settings(&mut self, _: Self::Settings) {}
}
