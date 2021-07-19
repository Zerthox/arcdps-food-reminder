pub mod buff;
pub mod player;

use crate::{
    tracking::{
        buff::Categorize,
        player::{Food, Utility},
    },
    ui::{color, Component},
};
use arcdps::imgui::{im_str, Ui};
use buff::Buff;
use player::Player;
use std::collections::HashMap;

/// Player tracker.
#[derive(Debug)]
pub struct Tracker {
    /// Currently tracked players sorted by id.
    players: HashMap<usize, Player>,
}

impl Tracker {
    /// Creates a new tracker.
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    /// Adds a new tracked player.
    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id, player);
    }

    /// Removes a tracked player, returning the removed player if they were tracked.
    pub fn remove_player(&mut self, id: usize) -> Option<Player> {
        self.players.remove(&id)
    }

    /// Returns a reference to a tracked player.
    pub fn get_player(&self, id: usize) -> Option<&Player> {
        self.players.get(&id)
    }

    /// Returns a mutable reference to a tracked player.
    pub fn get_player_mut(&mut self, id: usize) -> Option<&mut Player> {
        self.players.get_mut(&id)
    }

    /// Returns all tracked players.
    pub fn get_players(&self) -> Vec<&Player> {
        let mut players = self.players.values().collect::<Vec<_>>();
        players.sort_by_key(|player| player.subgroup);
        players
    }

    /// Returns an unsorted iterator over all tracked players.
    pub fn get_players_unsorted(&self) -> impl Iterator<Item = &Player> {
        self.players.values()
    }
}

impl Component for Tracker {
    type Props = ();

    fn create(_props: Self::Props) -> Self {
        Self::new()
    }

    fn render(&mut self, ui: &Ui) {
        if self.players.is_empty() {
            ui.text("No tracked players...");
        } else {
            // create table
            ui.begin_table(im_str!("food-reminder-tracker-table"), 4);

            // header
            ui.table_headers_row();
            ui.table_next_column();
            ui.table_header(im_str!("Sub"));
            ui.table_next_column();
            ui.table_header(im_str!("Player"));
            ui.table_next_column();
            ui.table_header(im_str!("Food"));
            ui.table_next_column();
            ui.table_header(im_str!("Util"));

            // iterate over tracked players
            for player in self.get_players() {
                // new row for each player
                ui.table_next_row();

                // subgroup
                ui.table_next_column();
                ui.text(player.subgroup.to_string());

                // name
                ui.table_next_column();
                ui.text(&player.character_name);
                if ui.is_item_hovered() {
                    ui.tooltip_text(&player.account_name);
                }

                // food
                ui.table_next_column();
                match player.food {
                    Buff::Unset => {
                        ui.text("???");
                        if ui.is_item_hovered() {
                            ui.tooltip_text("Uncertain");
                        }
                    }
                    Buff::None | Buff::Known(Food::Malnourished) => {
                        ui.text_colored(color::RED.to_rgba_f32s(), "NONE");
                        if ui.is_item_hovered() {
                            ui.tooltip_text("No Food");
                        }
                    }
                    Buff::Unknown => {
                        ui.text_colored(color::YELLOW.to_rgba_f32s(), "SOME");
                        if ui.is_item_hovered() {
                            ui.tooltip_text("Unknown Food");
                        }
                    }
                    Buff::Known(food) => {
                        ui.text_colored(color::GREEN.to_rgba_f32s(), food.categorize());
                        if ui.is_item_hovered() {
                            ui.tooltip_text(format!(
                                "{}\n{}",
                                food.name(),
                                food.stats().join("\n")
                            ));
                        }
                    }
                }

                // util
                ui.table_next_column();
                match player.util {
                    Buff::Unset => {
                        ui.text("???");
                        if ui.is_item_hovered() {
                            ui.tooltip_text("Uncertain");
                        }
                    }
                    Buff::None | Buff::Known(Utility::Diminished) => {
                        ui.text_colored(color::RED.to_rgba_f32s(), "NONE");
                        if ui.is_item_hovered() {
                            ui.tooltip_text("No Utility");
                        }
                    }
                    Buff::Unknown => {
                        ui.text_colored(color::YELLOW.to_rgba_f32s(), "SOME");
                        if ui.is_item_hovered() {
                            ui.tooltip_text("Unknown Utility");
                        }
                    }
                    Buff::Known(util) => {
                        ui.text_colored(color::GREEN.to_rgba_f32s(), util.categorize());
                        if ui.is_item_hovered() {
                            ui.tooltip_text(format!(
                                "{}\n{}",
                                util.name(),
                                util.stats().join("\n")
                            ));
                        }
                    }
                }
            }

            // end table
            ui.end_table();
        }
    }
}
